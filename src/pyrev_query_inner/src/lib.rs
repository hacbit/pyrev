use std::{any::TypeId, marker::PhantomData};

/// QueryId is a marker for a query. It can be an integer or a string.
///
/// Each data is associated with a query id in the DataMap.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QueryId {
    /// The id of the query.
    id: usize,
    /// A type marker.
    type_id: *mut TypeId,
    /// To mark the ownership of the type_id.
    _marker: PhantomData<*mut TypeId>,
}

impl QueryId {
    /// Create a new QueryId instance.
    ///
    /// # Safety
    #[inline]
    pub fn new(id: usize, type_id: TypeId) -> Self {
        QueryId {
            id,
            type_id: unsafe { std::mem::transmute(Box::new(type_id)) },
            _marker: PhantomData,
        }
    }

    /// Get the type id of the query id.
    #[inline]
    pub fn type_id(&self) -> TypeId {
        unsafe { *self.type_id }
    }

    /// Update the type id of the query id.
    #[inline]
    pub fn set_type_id(&self, type_id: TypeId) {
        unsafe {
            *self.type_id = type_id;
        }
    }
}