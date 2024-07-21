//! The expressions holder.
//! And provides the safety query for the expressions.

use pyrev_ast::*;
use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

/// QueryId is a marker for a query. It can be an integer or a string.
///
/// Each data is associated with a query id in the DataMap.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QueryId {
    /// The id of the query.
    ///
    /// It is unique in each DataMap.
    /// However, the same ID may exist in different DataMaps
    id: i32,
}

impl QueryId {
    /// Create a new QueryId instance.
    #[inline]
    pub(crate) fn new(id: i32) -> Self {
        QueryId { id }
    }
}

/// Holds all the datas and their associated query ids.
pub(crate) struct DataMap<T> {
    map: HashMap<QueryId, T>,
    id_generator: i32,
}

impl<T> DataMap<T> {
    /// Create a new Resources instance.
    #[inline]
    pub(crate) fn new() -> Self {
        Self {
            map: HashMap::new(),
            id_generator: i32::MIN,
        }
    }

    /// Add a new data to the resources.
    /// Returns the query id associated with the data.
    #[inline]
    pub(crate) fn add(&mut self, data: T) -> QueryId {
        let query_id = QueryId::new(self.id_generator);
        self.map.insert(query_id, data);
        self.id_generator += 1;
        query_id
    }

    /// Remove the data associated with the query id.
    #[inline]
    pub(crate) fn remove(&mut self, query_id: QueryId) -> Option<T> {
        self.map.remove(&query_id)
    }

    /// Get the data associated with the query id.
    #[inline]
    pub(crate) fn get(&self, query_id: QueryId) -> Option<&T> {
        self.map.get(&query_id)
    }

    /// Get the mutable reference to the data associated with the query id.
    #[inline]
    pub(crate) fn get_mut(&mut self, query_id: QueryId) -> Option<&mut T> {
        self.map.get_mut(&query_id)
    }

    /// Replace the data associated with the query id.
    #[inline]
    pub(crate) fn replace(&mut self, query_id: QueryId, data: T) -> Option<T> {
        self.map.insert(query_id, data)
    }

    /// Get the iterator of the data.
    #[inline]
    pub(crate) fn iter(&self) -> impl Iterator<Item = (QueryId, &T)> {
        self.map.iter().map(|(k, v)| (*k, v))
    }

    /// Get the mutable iterator of the data.
    #[inline]
    pub(crate) fn iter_mut(&mut self) -> impl Iterator<Item = (QueryId, &mut T)> {
        self.map.iter_mut().map(|(k, v)| (*k, v))
    }
}

/// The expressions holder.
/// It provides a DataMap for each type of expression.
pub struct Map {
    /// The map of the expressions.
    maps: HashMap<TypeId, DataMap<ExpressionEnum>>,
}

/// Add the expressions type to the map.
macro_rules! add_expressions_type {
    (
        #[add_to($map:expr)]
        enum $enum_name:ident {
            $(
                $variant:ident($type:ty),
            )*
        }
    ) => {
        $(
            $map.insert(TypeId::of::<$type>(), DataMap::new());
        )*
    };
}

impl Map {
    /// Create a new Map instance.
    pub fn new() -> Self {
        let mut maps = HashMap::new();
        // The enum copy from pyrev_ast
        add_expressions_type!(
            #[add_to(&mut maps)]
            enum ExpressionEnum {
                Import(Import),
                Class(Class),
                FastVariable(FastVariable),
                Function(Function),
                Return(Return),
                Yield(Yield),
                Assign(Assign),
                Alias(Alias),
                Try(Try),
                Except(Except),
                Finally(Finally),
                Assert(Assert),
                Raise(Raise),
                BaseValue(BaseValue),
                FormatValue(FormatValue),
                Format(Format),
                BinaryOperation(BinaryOperation),
                Subscr(Subscr),
                UnaryOperation(UnaryOperation),
                Call(Call),
                With(With),
                For(For),
                If(If),
                Await(Await),
                Jump(Jump),
                Container(Container),
                Slice(Slice),
                Attribute(Attribute),
            }
        );

        Self { maps }
    }

    /// Query the expressions of the type T.
    /// Returns the DataMap
    #[inline]
    pub(crate) fn query_map<T: Expression + 'static>(&self) -> Option<&DataMap<ExpressionEnum>> {
        self.maps.get(&TypeId::of::<T>())
    }

    /// Get the iterator of the data.
    #[inline]
    pub fn query<T: Expression + 'static>(
        &self,
    ) -> impl Iterator<Item = (QueryId, &ExpressionEnum)> {
        self.query_map::<T>()
            .and_then(|map| Some(map.iter()))
            .expect("No such query")
    }

    /// Query the mutable expressions of the type T.
    #[inline]
    pub(crate) fn query_map_mut<T: Expression + 'static>(
        &mut self,
    ) -> Option<&mut DataMap<ExpressionEnum>> {
        self.maps.get_mut(&TypeId::of::<T>())
    }

    /// Get the mutable iterator of the data.
    #[inline]
    pub fn query_mut<T: Expression + 'static>(
        &mut self,
    ) -> impl Iterator<Item = (QueryId, &mut ExpressionEnum)> {
        self.query_map_mut::<T>()
            .and_then(|map| Some(map.iter_mut()))
            .expect("No such query")
    }

    /// Get the data associated with the query id.
    #[inline]
    pub fn get_single<T: Expression + Any + 'static>(
        &self,
        query_id: QueryId,
    ) -> Option<&ExpressionEnum> {
        self.query_map::<T>()?.get(query_id)
    }

    /// Get the mutable reference to the data associated with the query id.
    #[inline]
    pub fn get_single_mut<T: Expression + Any + 'static>(
        &mut self,
        query_id: QueryId,
    ) -> Option<&mut ExpressionEnum> {
        self.query_map_mut::<T>()?.get_mut(query_id)
    }

    /// Add a new data to the resources.
    /// 
    /// Returns the query id associated with the data.
    #[inline]
    pub fn add<T: Expression + 'static>(&mut self, data: ExpressionEnum) -> Result<QueryId, ()> {
        Ok(self.query_map_mut::<T>().ok_or(())?.add(data))
    }

    /// Remove the data associated with the query id.
    /// 
    /// Returns the old data.
    #[inline]
    pub fn remove<T: Expression + 'static>(&mut self, query_id: QueryId) -> Option<ExpressionEnum> {
        self.query_map_mut::<T>()?.remove(query_id)
    }

    /// Replace the data associated with the query id.
    /// 
    /// Returns the old data.
    #[inline]
    pub fn replace<T: Expression + 'static>(
        &mut self,
        query_id: QueryId,
        data: ExpressionEnum,
    ) -> Option<ExpressionEnum> {
        self.query_map_mut::<T>()?.replace(query_id, data)
    }
}
