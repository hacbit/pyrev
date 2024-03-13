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
    pub fn get_mut(&self) -> &mut E {
        unsafe { &mut *self.value }
    }

    pub fn get(&self) -> &E {
        unsafe { &*self.value }
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
