use std::ops::Add;
use crate::Monoid;

#[derive(Debug, PartialOrd, PartialEq, Default)]
pub struct MonoidInstance<A> where A: Add<Output=A> + Default {
    value: A
}

/// Return reference, because MonoidInstance doesn't constraint by deriving (Copy, Clone)
impl<A> MonoidInstance<A> where A: Add<Output=A> + Default {
    pub fn value(&self) -> &A {
        &self.value
    }
    pub fn new(value: A) -> Self {
        MonoidInstance { value }
    }
}


impl<A> Monoid<A> for MonoidInstance<A> where A: Add<Output=A> + Default {
    fn combine_owned(x: Self, y: Self) -> Self {
        MonoidInstance { value: x.value + y.value }
    }

    fn id() -> Self {
        MonoidInstance::default()
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
        pub fn new(value:String) -> Self{
            StringMonoid{value}
        }
        pub fn from_slice(value:&str) ->Self{
            StringMonoid{value:value.to_owned()}
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
        assert_eq!(StringMonoid::combine_owned(s1,s2), StringMonoid::from_slice("HelloWorld"));

    }
    #[test]
    fn string_monoid_id_test() {
        let s1 = StringMonoid::from_slice("Hello");
        let s2 = StringMonoid::from_slice("World");
        assert_eq!(StringMonoid::combine_owned(StringMonoid::id(),s1.clone()),
                   StringMonoid::combine_owned(s1.clone(),StringMonoid::id()));
        assert_eq!(StringMonoid::combine_owned(s2.clone(),StringMonoid::id()), s2);
    }

    #[test]
    fn string_assoc_id_test(){
        let s1 = StringMonoid::from_slice("Hello ");
        let s2 = StringMonoid::from_slice("beautiful ");
        let s3 = StringMonoid::from_slice("world!");
        assert_eq!(StringMonoid::combine_owned(StringMonoid::combine_owned(s1.clone(),s2.clone()),s3.clone()),
                   StringMonoid::combine_owned(s1.clone(),StringMonoid::combine_owned(s2.clone(),s3.clone())));
    }
}