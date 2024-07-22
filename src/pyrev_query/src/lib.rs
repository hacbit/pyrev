//! The expressions holder.
//! And provides the safety query for the expressions.

use pyrev_ast::*;
use std::{
    any::TypeId,
    collections::{HashMap, HashSet},
    marker::PhantomData,
};

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
    pub(crate) fn new(id: usize, type_id: TypeId) -> Self {
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

/// Holds all the datas and their associated query ids.
#[derive(Debug)]
pub(crate) struct DataMap<T> {
    map: HashMap<QueryId, T>,
}

impl<T: 'static> DataMap<T> {
    /// Create a new Resources instance.
    #[inline]
    pub(crate) fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    /// Add a new data to the resources.
    /// Returns the query id associated with the data.
    #[inline]
    pub(crate) fn add(&mut self, query_id: QueryId, data: T) -> Option<T> {
        self.map.insert(query_id, data)
    }

    /// Set the data associated with the query id.
    /// Returns the old data.
    #[inline]
    pub(crate) fn set_by_id(&mut self, query_id: QueryId, data: T) -> Option<T> {
        self.map.insert(query_id, data)
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
#[derive(Debug)]
pub struct Map {
    /// The map of the expressions.
    maps: HashMap<TypeId, DataMap<ExpressionEnum>>,
    /// The id generator.
    id_generator: usize,
    /// The set of the existing query ids.
    ids: HashSet<usize>,
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

        Self {
            maps,
            id_generator: usize::MAX,
            ids: HashSet::new(),
        }
    }

    /// Query the expressions by the type id.
    /// Returns the DataMap
    #[inline]
    pub(crate) fn query_map(&self, type_id: TypeId) -> Option<&DataMap<ExpressionEnum>> {
        self.maps.get(&type_id)
    }

    /// Get the iterator of the data.
    ///
    /// # Example
    /// ```rust
    /// use pyrev_query::*;
    /// use pyrev_ast::*;
    ///
    /// let mut map = Map::new();
    /// let bin_op = BinaryOperation::default();
    /// let bin_op2 = BinaryOperation {
    ///     operator: "+".to_string(),
    ///     ..Default::default()
    /// };
    /// map.add::<BinaryOperation>(bin_op.clone().into()).expect("Failed to add");
    /// map.add::<BinaryOperation>(bin_op2.clone().into()).expect("Failed to add");
    ///
    /// let q = map.query::<BinaryOperation>().collect::<Vec<_>>();
    ///
    /// assert_eq!(q.len(), 2);
    /// ```
    #[inline]
    pub fn query<T: Expression + 'static>(
        &self,
    ) -> impl Iterator<Item = (QueryId, &ExpressionEnum)> {
        self.query_map(TypeId::of::<T>())
            .and_then(|map| Some(map.iter()))
            .expect("No such query")
    }

    /// Query the mutable expressions of the type T.
    #[inline]
    pub(crate) fn query_map_mut(
        &mut self,
        type_id: TypeId,
    ) -> Option<&mut DataMap<ExpressionEnum>> {
        self.maps.get_mut(&type_id)
    }

    /// Get the mutable iterator of the data.
    ///
    /// # Example
    /// ```rust
    /// use pyrev_query::*;
    /// use pyrev_ast::*;
    ///
    /// let mut map = Map::new();
    /// let mut func = Function::default();
    /// let mut func2 = Function {
    ///     name: "func2".to_string(),
    ///     ..Default::default()
    /// };
    /// map.add::<Function>(func.clone().into()).expect("Failed to add");
    /// map.add::<Function>(func2.clone().into()).expect("Failed to add");
    ///
    /// let q = map.query_mut::<Function>().collect::<Vec<_>>();
    ///
    /// assert_eq!(q.len(), 2);
    /// ```
    #[inline]
    pub fn query_mut<T: Expression + 'static>(
        &mut self,
    ) -> impl Iterator<Item = (QueryId, &mut ExpressionEnum)> {
        self.query_map_mut(TypeId::of::<T>())
            .and_then(|map| Some(map.iter_mut()))
            .expect("No such query")
    }

    /// Get the data associated with the query id.
    ///
    /// # Example
    /// ```rust
    /// use pyrev_query::*;
    /// use pyrev_ast::*;
    ///
    /// let mut map = Map::new();
    /// let func = Function::default();
    /// let query_id = map.add::<Function>(func.clone().into()).expect("Failed to add");
    /// let q_func = map.get_single(query_id).expect("Failed to get");
    ///
    /// assert_eq!(q_func.as_ref_function(), Some(&func));
    /// ```
    #[inline]
    pub fn get_single(&self, query_id: QueryId) -> Option<&ExpressionEnum> {
        self.query_map(query_id.type_id())?.get(query_id)
    }

    /// Get the mutable reference to the data associated with the query id.
    ///
    /// # Example
    /// ```rust
    /// use pyrev_query::*;
    /// use pyrev_ast::*;
    ///
    /// let mut map = Map::new();
    /// let mut func = Function::default();
    /// let query_id = map.add::<Function>(func.clone().into()).expect("Failed to add");
    ///
    /// let q_mut_func = map.get_single_mut(query_id).expect("Failed to get");
    ///
    /// assert_eq!(q_mut_func.as_ref_function(), Some(&func));
    ///
    /// // And then you can modify the data.
    /// if let ExpressionEnum::Function(func) = q_mut_func {
    ///     func.name = "new_name".to_string();
    /// }
    ///
    /// // query the data again. The data should be modified.
    /// let q_func = map.get_single(query_id).expect("Failed to get");
    ///
    /// // change original data to compare.
    /// func.name = "new_name".to_string();
    ///
    /// assert_eq!(q_func.as_ref_function(), Some(&func));
    /// ```
    #[inline]
    pub fn get_single_mut(&mut self, query_id: QueryId) -> Option<&mut ExpressionEnum> {
        self.query_map_mut(query_id.type_id())?.get_mut(query_id)
    }

    /// Add a new data to the resources.
    ///
    /// And return a new query id associated with the data.
    ///
    /// # Example
    /// ```rust
    /// use pyrev_query::*;
    /// use pyrev_ast::*;
    ///
    /// let mut map = Map::new();
    /// let class = Class::default();
    /// let query_id = map.add::<Class>(class.into()).expect("Failed to add");
    ///
    /// assert_eq!(query_id.type_id(), std::any::TypeId::of::<Class>());
    /// ```
    #[inline]
    pub fn add<T: Expression + 'static>(&mut self, data: ExpressionEnum) -> Option<QueryId> {
        self.id_generator = self.id_generator.wrapping_add(1);
        let id = self.id_generator;
        assert!(self.ids.contains(&id) == false);
        self.ids.insert(id);
        let query_id = QueryId::new(id, TypeId::of::<T>());
        self.query_map_mut(query_id.type_id())?.add(query_id, data);
        Some(query_id)
    }

    /// Remove the data associated with the query id.
    ///
    /// Returns the old data.
    ///
    /// # Example
    /// ```rust
    /// use pyrev_query::*;
    /// use pyrev_ast::*;
    ///
    /// let mut map = Map::new();
    ///
    /// let base = BaseValue::default();
    /// let query_id = map.add::<BaseValue>(base.clone().into()).expect("Failed to add");
    /// let data = map.remove(query_id).expect("Failed to remove");
    ///
    /// assert_eq!(data.as_ref_base_value(), Some(&base));
    /// ```
    #[inline]
    pub fn remove(&mut self, query_id: QueryId) -> Option<ExpressionEnum> {
        self.query_map_mut(query_id.type_id())?.remove(query_id)
    }

    /// Set the data associated with the query id.
    ///
    /// Returns new query id which is marked with the type of U and the old data.
    ///
    /// # Example
    /// ```rust
    /// use pyrev_query::*;
    /// use pyrev_ast::*;
    ///
    /// let mut map = Map::new();
    /// let func = Function::default();
    /// let query_id = map.add::<Function>(func.clone().into()).expect("Failed to add");
    /// let base = BaseValue::default();
    /// let old_data = map.replace::<BaseValue>(query_id, base.into()).expect("Failed to replace");
    /// assert_eq!(old_data.as_ref_function(), Some(&func));
    /// let data = map.query::<Function>().collect::<Vec<_>>();
    /// assert_eq!(data.len(), 0);
    /// let data = map.query::<BaseValue>().collect::<Vec<_>>();
    /// assert_eq!(data.len(), 1);
    /// ```
    #[inline]
    pub fn replace<U: Expression + 'static>(
        &mut self,
        query_id: QueryId,
        data: ExpressionEnum,
    ) -> Option<ExpressionEnum> {
        let old_data = self.remove(query_id);
        query_id.set_type_id(TypeId::of::<U>());
        self.query_map_mut(query_id.type_id())?
            .set_by_id(query_id, data);
        old_data
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_data_map() {
        let mut map = Map::new();

        let func = Function::default();
        let base = BaseValue::default();

        let query_id = map.add::<Function>(func.into()).unwrap();

        // map.replace::<Function>(query_id, func2.into());

        let data = map.query::<Function>().collect::<Vec<_>>();
        dbg!(data);

        map.replace::<BaseValue>(query_id, base.into());

        let data = map.query::<Function>().collect::<Vec<_>>();
        dbg!(data);

        let data = map.query::<BaseValue>().collect::<Vec<_>>();
        dbg!(data);
    }

    #[test]
    fn test_mut_query() {
        let mut map = Map::new();
        let mut func = Function::default();
        let query_id = map
            .add::<Function>(func.clone().into())
            .expect("Failed to add");

        let q_mut_func = map.get_single_mut(query_id).expect("Failed to get");

        assert_eq!(q_mut_func.as_ref_function(), Some(&func));

        // And then you can modify the data.
        if let ExpressionEnum::Function(func) = q_mut_func {
            func.name = "new_name".to_string();
        }
        dbg!(&q_mut_func);
        // query the data again. The data should be modified.
        let q_func = map.get_single(query_id).expect("Failed to get");
        dbg!(&q_func);
        func.name = "new_name".to_string();
        assert_eq!(q_func.as_ref_function(), Some(&func));
    }
}
