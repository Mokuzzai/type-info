#![feature(const_type_id)]
#![feature(inline_const)]
#![feature(ptr_metadata)]

use std::any::Any;
use std::any::TypeId;
use std::any;

use std::marker::PhantomData;

use std::ptr::DynMetadata;
use std::ptr;

use std::mem;

unsafe trait DynTypeInfo {
	fn name(&self) -> &'static str;

	fn type_id_ref(&self) -> &'static TypeId;

	fn type_id(&self) -> TypeId;
}

#[repr(C, align(1))]
struct TypeCarrier<T: ?Sized> {
	_p: PhantomData<T>,
}

impl<T: ?Sized> TypeCarrier<T> {
	fn new() -> Self {
		TypeCarrier {
			_p: PhantomData,
		}
	}
}

unsafe impl<T: ?Sized + Any> DynTypeInfo for TypeCarrier<T> {
	fn name(&self) -> &'static str {
		any::type_name::<T>()
	}
	fn type_id_ref(&self) -> &'static TypeId {
		&const { TypeId::of::<T>() }
	}
	fn type_id(&self) -> TypeId {
		TypeId::of::<T>()
	}
}

#[derive(Copy, Clone)] // TODO: delegate `Ord` + `Hash` to `Self::type_id`
pub struct TypeInfo {
	carrier: DynMetadata<dyn DynTypeInfo>,
}

impl TypeInfo {
	pub fn new<T: ?Sized + Any>() -> Self {
		Self {
			carrier: ptr::metadata(&TypeCarrier::<T>::new() as &dyn DynTypeInfo),
		}
	}
	pub fn from_val<T: ?Sized + Any>(_: &T) -> Self {
		Self::new::<T>()
	}
	fn type_info(&self) -> &dyn DynTypeInfo {
		unsafe { &*ptr::from_raw_parts(mem::align_of::<TypeCarrier<()>>() as *const (), self.carrier) }
	}
	pub fn name(&self) -> &'static str {
		self.type_info().name()
	}
	pub fn type_id_ref(&self) -> &'static TypeId {
		self.type_info().type_id_ref()
	}
	pub fn type_id(&self) -> TypeId {
		self.type_info().type_id()
	}
	pub fn is<T: Any>(&self) -> bool {
		self.type_id() == TypeId::of::<T>()
	}
}
/*
NOTE: this is probably too error-prone

impl<'a, T: Any> From<&'a T> for TypeInfo {
	fn from(carrier: &'a T) -> Self {
		Self::new::<T>()
	}
}*/

const _: () = {
	use std::fmt::*;

	impl Debug for TypeInfo {
		fn fmt(&self, f: &mut Formatter) -> Result {
			f.debug_struct("TypeInfo")
			.field("name", &self.name())
			.field("type_id", &self.type_id())
			.finish()
		}
	}
};

#[cfg(test)]
#[test]
fn basic() {
	let type_info = TypeInfo::new::<String>();

	assert_eq!(type_info.type_id(), TypeId::of::<String>());
	assert_eq!(type_info.name(), "alloc::string::String");

}

