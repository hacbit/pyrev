use crate::{Expression, Queryable};
use std::{
    any::{Any, TypeId},
    ops::{Deref, DerefMut},
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct QueryResMut<T>
where
    T: Queryable + Sized,
{
    value: *mut T,
}

pub trait QueryMutable
where
    Self: Queryable + Sized,
{
    fn with_mut_unchecked(&self) -> QueryResMut<Self>;
}

impl<T> QueryMutable for T
where
    T: Queryable + Sized,
{
    fn with_mut_unchecked(&self) -> QueryResMut<Self> {
        QueryResMut {
            value: self as *const T as *mut T,
        }
    }
}

impl<E> Deref for QueryResMut<E>
where
    E: Expression + 'static,
{
    type Target = E;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.value }
    }
}

impl<E> DerefMut for QueryResMut<E>
where
    E: Expression + 'static,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.value }
    }
}

impl<E> QueryResMut<E>
where
    E: Expression + 'static,
{
    pub fn get(&self) -> &E {
        unsafe { &*self.value }
    }

    pub fn patch<T: 'static>(&self, value: T) -> Result<()> {
        if TypeId::of::<*mut T>() == self.value.type_id() {
            unsafe {
                *(self.value as *mut T) = value;
            }
            Ok(())
        } else {
            Err("[QueryResMut] Patch error".into())
        }
    }

    /// Patch by function
    /// # Example
    /// ```rust
    /// use pyrev_ast::*;
    ///
    /// #[derive(Expression, Clone, Debug, PartialEq, Eq, Query, Default)]
    /// struct A {
    ///     b: B,
    /// }
    /// #[derive(Expression, Clone, Debug, PartialEq, Eq, Query, Default)]
    /// struct B {
    ///     c: usize,
    /// }
    ///
    /// let a = A {
    ///     b: B { c: 1 },
    /// };
    /// let b = a.query::<B>();
    /// assert_eq!(b, vec![&B { c: 1 }]);
    /// b[0].with_mut_unchecked().patch_by(|mut f| {
    ///     f.c = 2;
    /// }).unwrap();
    /// assert_eq!(b, vec![&B { c: 2 }]);
    /// ```
    pub fn patch_by<F>(self, f: F) -> Result<()>
    where
        F: FnOnce(Self),
    {
        if TypeId::of::<*mut E>() == self.value.type_id() {
            f(self);
            Ok(())
        } else {
            Err("[QueryResMut] Patch_by error".into())
        }
    }
}
