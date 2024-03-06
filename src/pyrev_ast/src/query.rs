use crate::Expression;
use std::any::{Any, TypeId};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Queryable trait is used to convert the struct to dyn Any
/// It is used to search for the type T in the Struct
pub trait Queryable {
    fn as_any(&self) -> &dyn Any;
    fn try_query<T: 'static>(&self) -> Option<&T> {
        if TypeId::of::<T>() == self.as_any().type_id() {
            unsafe { Some(&*(self.as_any() as *const dyn Any as *const T)) }
        } else {
            None
        }
    }
}

impl<T: 'static> Queryable for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Query example:
/// ```
/// use pyrev_ast::*;
/// use pyrev_ast_derive::*;
/// #[derive(Query, Debug)]
/// struct A {
///     b: B,
///     c: C,
/// }
/// #[derive(Query, Debug)]
/// struct B {
///     d: String,
/// }
/// #[derive(Query, Debug, PartialEq)]
/// struct C {
///     e: usize,
/// }
///     let a = A {
///         b: B {
///             d: "hello".to_string(),
///         },
///         c: C {
///             e: 1,
///         },
///     };
///     let result = a.query::<C>();
///     assert_eq!(
///         result,
///         vec![&C {
///             e: 1,
///         }]
///     );
/// ```
pub trait Query {
    fn query<T: std::fmt::Debug + Expression + 'static>(&self) -> Vec<&T>;
    fn query_singleton<T: std::fmt::Debug + Expression + 'static>(&self) -> Result<&T> {
        let result = self.query::<T>();
        if result.len() == 1 {
            Ok(result[0])
        } else {
            Err("Query error: Not singleton".into())
        }
    }
}

/// The following implementations are used to search for the type T in the Struct

impl Query for String {
    fn query<U: 'static>(&self) -> Vec<&U> {
        vec![]
    }
}

impl Query for usize {
    fn query<U: 'static>(&self) -> Vec<&U> {
        vec![]
    }
}

impl<T: Query> Query for Vec<T> {
    fn query<U: std::fmt::Debug + Expression + 'static>(&self) -> Vec<&U> {
        let mut result = Vec::new();
        for item in self.iter() {
            result.extend(item.query::<U>());
        }
        result
    }
}

impl<T: Query> Query for Box<T> {
    fn query<U: std::fmt::Debug + Expression + 'static>(&self) -> Vec<&U> {
        self.as_ref().query::<U>()
    }
}

impl<T: Query> Query for Option<T> {
    fn query<U: std::fmt::Debug + Expression + 'static>(&self) -> Vec<&U> {
        match self {
            Some(t) => t.query::<U>(),
            None => vec![],
        }
    }
}
