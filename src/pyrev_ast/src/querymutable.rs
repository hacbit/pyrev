use crate::{Expression, Queryable};
use std::any::{Any, TypeId};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct QueryResMut<T>
where
    T: Sized,
{
    value: *mut T,
}

pub trait QueryMutable
where
    Self: Queryable + Sized,
{
    fn with_mut(&self) -> QueryResMut<Self>;
}

impl<T: 'static> QueryMutable for T
where
    T: Queryable,
{
    fn with_mut(&self) -> QueryResMut<Self> {
        QueryResMut {
            value: self as *const T as *mut T,
        }
    }
}

impl<E> QueryResMut<E>
where
    E: Expression + 'static,
{
    pub fn try_get_mut<T: 'static>(&self) -> Option<&mut T> {
        if TypeId::of::<T>() == self.value.type_id() {
            unsafe { Some(&mut *(self.value as *mut T)) }
        } else {
            None
        }
    }

    pub fn try_get<T: 'static>(&self) -> Option<&T> {
        if TypeId::of::<T>() == self.value.type_id() {
            unsafe { Some(&*(self.value as *const T)) }
        } else {
            None
        }
    }

    pub fn patch<T: 'static>(&self, value: T) -> Result<()> {
        if TypeId::of::<T>() == self.value.type_id() {
            unsafe {
                *(self.value as *mut T) = value;
            }
            Ok(())
        } else {
            Err("[QueryResMut] Patch error".into())
        }
    }

    pub fn patch_by<F>(&self, f: F) -> Result<()>
    where
        F: FnOnce(&mut E),
    {
        if TypeId::of::<*mut E>() == self.value.type_id() {
            unsafe {
                f(&mut *self.value);
            }
            Ok(())
        } else {
            Err("[QueryResMut] Patch_by error".into())
        }
    }
}
