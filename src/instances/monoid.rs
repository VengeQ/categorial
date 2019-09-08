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

/// Monoid have two methods:
/// combine(m1:Monoid, m2:Monoid) - return new monoid.
/// id() - return identity element for type A and combine operation.
/// MonoidInstance from categorial::instances::MonoidInstances constraint by deriving Default, because default value for type A returned as id().
/// There are any value (not necessary Default for type A) in the custom implementation for monoid
/// A simple example where id = 1 and associative operation is multiplication show below
/// ```
///    use categorial::Monoid;
///    #[derive(Debug, PartialEq, Clone)]
///    struct MonoidExample{
///        value:usize
///    }
///    impl Monoid<usize> for MonoidExample{
///        fn combine(x: Self, y: Self) -> Self {
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
///    assert_eq!(Monoid::combine(x1.clone(), x2), MonoidExample{ value: (12 * 13) as usize });
///    assert_eq!(Monoid::combine(x1, Monoid::id()), MonoidExample{ value: (12 * 1) as usize });
/// ```
///
impl<A> Monoid<A> for MonoidInstance<A> where A: Add<Output=A> + Default {
    fn combine(x: Self, y: Self) -> Self {
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
    fn monoid_instance_combine_test() {
        let m1 = MonoidInstance::new(12);
        let m2 = MonoidInstance::new(14);
        assert_eq!(Monoid::combine(m1, m2), MonoidInstance::new(12 + 14));
    }

    #[test]
    fn monoid_instance_id_test() {
        let m1 = MonoidInstance::new(12);
        assert_eq!(Monoid::combine(m1, MonoidInstance::default()), MonoidInstance::new(12 + 0));
        let m1 = MonoidInstance::new(11);
        assert_eq!(Monoid::combine(MonoidInstance::default(), m1), MonoidInstance::new(0 + 11));
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
        fn combine(x: Self, y: Self) -> Self {
            StringMonoid::new(x.value.to_owned().add(&y.value))
        }
        fn id() -> Self {
            StringMonoid::new("".to_owned())
        }
    }

    #[test]
    fn string_monoid_combine_test() {
        let s1 = StringMonoid::from_slice("Hello");
        let s2 = StringMonoid::from_slice("World");
        assert_eq!(StringMonoid::combine(s1,s2), StringMonoid::from_slice("HelloWorld"));

    }
    #[test]
    fn string_monoid_id_test() {
        let s1 = StringMonoid::from_slice("Hello");
        let s2 = StringMonoid::from_slice("World");
        assert_eq!(StringMonoid::combine(StringMonoid::id(),s1.clone()),
                   StringMonoid::combine(s1.clone(),StringMonoid::id()));
        assert_eq!(StringMonoid::combine(s2.clone(),StringMonoid::id()), s2);
    }

    #[test]
    fn string_assoc_id_test(){
        let s1 = StringMonoid::from_slice("Hello ");
        let s2 = StringMonoid::from_slice("beautiful ");
        let s3 = StringMonoid::from_slice("world!");
        assert_eq!(StringMonoid::combine(StringMonoid::combine(s1.clone(),s2.clone()),s3.clone()),
                   StringMonoid::combine(s1.clone(),StringMonoid::combine(s2.clone(),s3.clone())));
    }
}