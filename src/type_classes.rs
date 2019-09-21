use std::fmt::Debug;

///combine methods in semigroup return new semigroup with new value:
///* combine_owned(m1:Semigroup, m2:Semigroup) - m1 and m2 are owned by this method.
///* combine(m1:&Semigroup, m2:&Semigroup) - m1 and m2 are borrowed by this method.
///
///# Example
/// ```
/// use categorial::type_classes::Semigroup;
/// #[derive(Debug, PartialEq)]
///       struct SemigroupExample{
///            value:usize
///        }
///        impl Semigroup<usize> for SemigroupExample{
///            fn combine_owned(x: Self, y: Self) -> Self {
///                SemigroupExample{value:x.value+y.value}
///            }
///
///            fn combine(x: &Self,y: &Self) -> Self {
///                  SemigroupExample{value:x.value+y.value}
///             }
///       }
///       let x1 = SemigroupExample { value: 12_usize };
///       let x2 = SemigroupExample { value: 13_usize };
///       assert_eq!(Semigroup::combine_owned(x1, x2), SemigroupExample{ value: (12 + 13) as usize });
///```
///
pub trait Semigroup<A> {
    fn combine_owned(x: Self, y: Self) -> Self;
    fn combine(x: &Self, y: &Self) -> Self;
}

/// Monoid have two methods:
/// 1) combine - return new monoid with some operations. There are two variances in this crate.
///    * combine_owned(m1:Monoid, m2:Monoid) - m1 and m2 are owned by this method.
///    * combine(m1:&Monoid, m2:&Monoid) - m1 and m2 are borrowed by this method.
/// 2) id() - return identity element for type A and combine_owned operation.
/// MonoidInstance from categorial::instances::MonoidInstances constraint by deriving Default, because default value for type A returned as id().
///
/// There are any value (not necessary Default for type A) in the custom implementation for monoid
///
/// A simple example where id = 1 and associative operation is multiplication show below
/// # Example
/// ```
///    use categorial::Monoid;
///    #[derive(Debug, PartialEq, Clone)]
///    struct MonoidExample{
///        value:usize
///    }
///    impl Monoid<usize> for MonoidExample{
///        fn combine_owned(x: Self, y: Self) -> Self {
///            MonoidExample{value:x.value*y.value}
///        }
///
///        fn id() -> Self {
///            MonoidExample{value:1}
///        }
///    }
///    let x1 = MonoidExample { value: 12_usize };
///    let x2 = MonoidExample { value: 13_usize };
///
///    assert_eq!(Monoid::combine_owned(x1.clone(), x2), MonoidExample{ value: (12 * 13) as usize });
///    assert_eq!(Monoid::combine_owned(x1, Monoid::id()), MonoidExample{ value: (12 * 1) as usize });
/// ```
///
pub trait Monoid<A> {
    fn combine_owned(x: Self, y: Self) -> Self;
    fn id() -> Self;
}

pub trait Gen {
    type Typet;
}

pub trait NewGen<T>:Gen {
    type Type;
}


///I can't do this(
pub trait Functor: Gen {
    //fn map_to_value<B, F>(&self, f: F) -> B where F: Fn(A) -> B;
    fn fmap<R, F: Fn(Self::Type) -> R>(&self, f: F) -> Self::Type
        where Self: NewGen<R>;
}


#[cfg(test)]
mod test {
    use std::fmt::Debug;
    use crate::type_classes::{Functor,};

    struct FunctorVec<T>{
        value: Vec<T>
    }

    #[test]
    fn func_test() {
        use super::Functor;
        #[derive(Debug, PartialEq, Clone)]
        struct FunctorInstance {
            value: String
        }
    }
}

