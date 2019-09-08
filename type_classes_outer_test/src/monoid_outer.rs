use categorial::Monoid;
use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Example{
    value:String
}

impl Monoid<String> for Example{
    fn combine(x: Self, y: Self) -> Self {
        Example{value:x.value.clone().add(&y.value)}
    }
    fn id() -> Self {
        Example{value:"".to_owned()}
    }
}

#[cfg(test)]
mod test{
    use self::super::Example;
    use categorial::Monoid;
    #[test]
    fn monoid_combine_example_test(){
        let e1= Example{value:"hello ".to_owned()};
        let e2= Example{value:"world".to_owned()};
        assert_eq!(Monoid::combine(e1,e2), Example{value:"hello world".to_owned()});
    }
    #[test]
    fn monoid_identity_example_test(){
        let e1= Example{value:"hello".to_owned()};
        assert_eq!(Monoid::combine(e1,Example::id()), Example{value:"hello".to_owned()});
    }
    use categorial::instances::monoid::MonoidInstance;
    #[test]
    fn monoid_instance_combine_example_test(){
        let e1= MonoidInstance::new(11);
        let e2= MonoidInstance::new(12);
        assert_eq!(Monoid::combine(e1,e2), MonoidInstance::new(23));
    }
    #[test]
    fn monoid_instance_id_example_test(){
        let e1= MonoidInstance::new(11);
        assert_eq!(Monoid::combine(e1,MonoidInstance::id()), MonoidInstance::new(11));
    }
}

