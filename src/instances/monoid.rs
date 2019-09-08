use std::ops::Add;
use crate::Monoid;
use std::fmt::Debug;

#[derive(Debug, PartialOrd, PartialEq, Default)]
pub struct MonoidInstance<A> where A: Add<Output=A> + Default {
    value: A
}

///Simple generic implementation with value and new method.
impl<A> MonoidInstance<A> where A: Add<Output=A> + Default {
    /// Return reference, because MonoidInstance doesn't constraint by deriving (Copy, Clone)
    pub fn value(&self) -> &A {
        &self.value
    }
    pub fn new(value: A) -> Self {
        MonoidInstance { value }
    }
}

///Simple implementation with sum function as associative operation.
impl<A> Monoid<A> for MonoidInstance<A> where A: Add<Output=A> + Default {
    fn combine_owned(x: Self, y: Self) -> Self {
        MonoidInstance { value: x.value + y.value }
    }
    fn id() -> Self {
        MonoidInstance::default()
    }
}

///Some base implementation
///Monoid for string
#[derive(Debug, Default, Ord, PartialOrd, Eq, PartialEq, Hash, Clone)]
pub struct MonoidString {
    value: String
}

impl MonoidString {
    pub fn value(&self) -> &str {
        &self.value
    }
    pub fn new(value: String) -> Self { MonoidString { value } }
}

impl Monoid<String> for MonoidString {
    fn combine_owned(x: Self, y: Self) -> Self {
        MonoidString { value: x.value.clone().add(&y.value) }
    }
    fn id() -> Self {
        MonoidString { value: "".to_owned() }
    }
}

///Monoid for usize and isize
#[derive(Debug, Default, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy)]
pub struct MonoidUnumber {
    value: usize
}

impl MonoidUnumber {
    pub fn new(value: usize) -> Self {
        MonoidUnumber { value }
    }
    pub fn value(&self) -> usize {
        self.value
    }
}

impl Monoid<usize> for MonoidUnumber {
    fn combine_owned(x: Self, y: Self) -> Self {
        MonoidUnumber { value: x.value + y.value }
    }

    fn id() -> Self {
        MonoidUnumber { value: 0 }
    }
}

#[derive(Debug, Default, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy)]
pub struct MonoidInumber {
    value: isize
}

impl MonoidInumber {
    pub fn new(value: isize) -> Self {
        MonoidInumber { value }
    }
    pub fn value(&self) -> isize {
        self.value
    }
}

impl Monoid<isize> for MonoidInumber {
    fn combine_owned(x: Self, y: Self) -> Self {
        MonoidInumber { value: x.value + y.value }
    }

    fn id() -> Self {
        MonoidInumber { value: 0 }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MonoidVec<T> where T: Clone+Debug +PartialEq{
    value: Vec<T>
}

impl<T> MonoidVec<T> where T: Clone+Debug +PartialEq {
    pub fn new(value: Vec<T>) -> Self {
        MonoidVec { value }
    }
    pub fn value(&self) -> &Vec<T> {
        &(self.value)
    }
}

impl<T> Monoid<Vec<T>> for MonoidVec<T> where T: Clone+Debug +PartialEq {
    fn combine_owned(x: Self, y: Self) -> Self {
        let value: Vec<T> = x.value.iter().cloned().chain(y.value.iter().cloned()).collect();
        MonoidVec{value}
    }

    fn id() -> Self {
        let value:Vec<T> = Vec::new();
        MonoidVec{value}
    }
}

#[cfg(test)]
mod test {
    use super::MonoidInstance;
    use super::Monoid;

    #[test]
    fn monoid_instance_test() {
        let m = MonoidInstance::new(12);
        assert_eq!(m, MonoidInstance::new(12));
    }

    #[test]
    fn monoid_instance_combine_owned_test() {
        let m1 = MonoidInstance::new(12);
        let m2 = MonoidInstance::new(14);
        assert_eq!(Monoid::combine_owned(m1, m2), MonoidInstance::new(12 + 14));
    }

    #[test]
    fn monoid_instance_id_test() {
        let m1 = MonoidInstance::new(12);
        assert_eq!(Monoid::combine_owned(m1, MonoidInstance::default()), MonoidInstance::new(12 + 0));
        let m1 = MonoidInstance::new(11);
        assert_eq!(Monoid::combine_owned(MonoidInstance::default(), m1), MonoidInstance::new(0 + 11));
    }

    ///Simple String monoid.
    #[derive(Debug, PartialEq, Default, Clone)]
    struct StringMonoid {
        value: String
    }

    use std::ops::Add;

    impl StringMonoid {
        pub fn value(&self) -> String {
            self.value.to_owned()
        }
        pub fn new(value: String) -> Self {
            StringMonoid { value }
        }
        pub fn from_slice(value: &str) -> Self {
            StringMonoid { value: value.to_owned() }
        }
    }

    impl Monoid<String> for StringMonoid {
        fn combine_owned(x: Self, y: Self) -> Self {
            StringMonoid::new(x.value.to_owned().add(&y.value))
        }
        fn id() -> Self {
            StringMonoid::new("".to_owned())
        }
    }

    #[test]
    fn string_monoid_combine_owned_test() {
        let s1 = StringMonoid::from_slice("Hello");
        let s2 = StringMonoid::from_slice("World");
        assert_eq!(StringMonoid::combine_owned(s1, s2), StringMonoid::from_slice("HelloWorld"));
    }

    #[test]
    fn string_monoid_id_test() {
        let s1 = StringMonoid::from_slice("Hello");
        let s2 = StringMonoid::from_slice("World");
        assert_eq!(StringMonoid::combine_owned(StringMonoid::id(), s1.clone()),
                   StringMonoid::combine_owned(s1.clone(), StringMonoid::id()));
        assert_eq!(StringMonoid::combine_owned(s2.clone(), StringMonoid::id()), s2);
    }

    #[test]
    fn string_assoc_id_test() {
        let s1 = StringMonoid::from_slice("Hello ");
        let s2 = StringMonoid::from_slice("beautiful ");
        let s3 = StringMonoid::from_slice("world!");
        assert_eq!(StringMonoid::combine_owned(StringMonoid::combine_owned(s1.clone(), s2.clone()), s3.clone()),
                   StringMonoid::combine_owned(s1.clone(), StringMonoid::combine_owned(s2.clone(), s3.clone())));
    }

    ///Instances test
    #[test]
    fn monoid_string_test() {
        use self::super::MonoidString;
        let m1 = MonoidString::new("Hello".to_owned());
        let m2 = MonoidString::new("World".to_owned());
        assert_eq!(Monoid::combine_owned(m1.clone(), m2), MonoidString::new("HelloWorld".to_owned()));
        assert_eq!(Monoid::combine_owned(m1.clone(), Monoid::id()), m1);
    }

    #[test]
    fn monoid_unumber_test() {
        use self::super::MonoidUnumber;
        let m1 = MonoidUnumber::new(12);
        let m2 = MonoidUnumber::new(23);
        assert_eq!(Monoid::combine_owned(m1, m2), MonoidUnumber::new(12 + 23));
        assert_eq!(Monoid::combine_owned(m1, Monoid::id()), m1);
    }

    #[test]
    fn monoid_inumber_test() {
        use self::super::MonoidInumber;
        let m1 = MonoidInumber::new(-12);
        let m2 = MonoidInumber::new(23);
        assert_eq!(Monoid::combine_owned(m1, m2), MonoidInumber::new(-12 + 23));
        assert_eq!(Monoid::combine_owned(m1, Monoid::id()), m1);
    }
    #[test]
    fn monoid_vec_test() {
        use self::super::MonoidVec;
        let m1 = MonoidVec::new(vec![1,2,3]);
        let m2 = MonoidVec::new(vec![4,5,6]);
        assert_eq!(Monoid::combine_owned(m1.clone(), m2), MonoidVec::new(vec![1,2,3,4,5,6]));
        assert_eq!(Monoid::combine_owned(m1.clone(), Monoid::id()), m1);
    }
}