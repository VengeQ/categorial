use std::ops::Add;
use crate::type_classes::Semigroup;

pub mod type_classes;


#[derive(Debug)]
struct SemigroupInstance<A> where A: Add<Output=A> {
    value: A
}


/// Return reference, because SemigroupInstance doesn't constraint by deriving (Copy, Clone)
impl<A> SemigroupInstance<A> where A: Add<Output=A> {
    pub fn value(&self) -> &A {
        &self.value
    }
}


///Main method in semigroup
///
///```
/// use categorial::type_classes::Semigroup;
/// #[derive(Debug, PartialEq)]
///       struct SemigroupExample{
///            value:usize
///        }
///        impl Semigroup<usize> for SemigroupExample{
///            fn combine(x: Self, y: Self) -> Self {
///                SemigroupExample{value:x.value+y.value}
///            }
///       }
///       let x1 = SemigroupExample { value: 12_usize };
///       let x2 = SemigroupExample { value: 13_usize };
///       assert_eq!(Semigroup::combine(x1, x2), SemigroupExample{ value: (12 + 13) as usize });
///```
///
impl<A> Semigroup<A> for SemigroupInstance<A> where A: Add<Output=A> {
    fn combine(x: Self, y: Self) -> Self {
        SemigroupInstance { value: x.value.add(y.value) }
    }
}

#[cfg(test)]
mod test {
    use super::SemigroupInstance;
    use crate::type_classes::Semigroup;

    #[test]
    fn semigroup_impl_test() {
        let s = SemigroupInstance {
            value: 23
        };

        let s2 = SemigroupInstance {
            value: 23
        };
        let s3 = SemigroupInstance {
            value: 11_f64
        };


        #[derive(Debug, PartialEq)]
        struct SemigroupExample{
            value:usize
        }
        impl Semigroup<usize> for SemigroupExample{
            fn combine(x: Self, y: Self) -> Self {
                SemigroupExample{value:x.value+y.value}
            }
        }
        let x1 = SemigroupExample { value: 12_usize };
        let x2 = SemigroupExample { value: 13_usize };
        assert_eq!(Semigroup::combine(x1, x2), SemigroupExample{ value: (12 + 13) as usize });
    }

    #[test]
    fn semigroup_type_incorrect_test() {

    }
}