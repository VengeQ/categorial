use std::ops::Add;
use super::super::type_classes::Semigroup;

#[derive(Debug, PartialOrd, PartialEq)]
pub struct SemigroupInstance<A> where A: Add<Output=A> {
    value: A
}

/// Return reference, because SemigroupInstance doesn't constraint by deriving (Copy, Clone)
impl<A> SemigroupInstance<A> where A: Add<Output=A> {
    pub fn value(&self) -> &A {
        &self.value
    }
    pub fn new(value: A) -> Self {
        SemigroupInstance { value }
    }
}

///combine_owned method in semigroup return new semigroup with new value.
///
///```
/// use categorial::type_classes::Semigroup;
/// #[derive(Debug, PartialEq)]
///       struct SemigroupExample{
///            value:usize
///        }
///        impl Semigroup<usize> for SemigroupExample{
///            fn combine_owned(x: Self, y: Self) -> Self {
///                SemigroupExample{value:x.value+y.value}
///            }
///       }
///       let x1 = SemigroupExample { value: 12_usize };
///       let x2 = SemigroupExample { value: 13_usize };
///       assert_eq!(Semigroup::combine_owned(x1, x2), SemigroupExample{ value: (12 + 13) as usize });
///```
///
impl<A> Semigroup<A> for SemigroupInstance<A> where A: Add<Output=A> {
    fn combine_owned(x: Self, y: Self) -> Self {
        SemigroupInstance { value: x.value.add(y.value) }
    }
}

#[cfg(test)]
mod test {
    use super::SemigroupInstance;
    use super::Semigroup;


    ///Default instance of semigroup test
    #[test]
    fn semigroup_instance_test() {
        let s = SemigroupInstance::new(23);
        let t = SemigroupInstance::new(24);
        assert_eq!(Semigroup::combine_owned(s, t), SemigroupInstance::new(23 + 24));
    }

    ///Custom instance of semigroup test
    #[test]
    fn semigroup_instance_custom_test() {
        #[derive(Debug, PartialEq)]
        struct SemigroupExample { value: usize }
        impl Semigroup<usize> for SemigroupExample {
            fn combine_owned(x: Self, y: Self) -> Self {
                SemigroupExample { value: x.value * y.value }
            }
        }
        let x1 = SemigroupExample { value: 12_usize };
        let x2 = SemigroupExample { value: 13_usize };
        assert_eq!(Semigroup::combine_owned(x1, x2), SemigroupExample { value: (12 * 13) as usize });
    }
}