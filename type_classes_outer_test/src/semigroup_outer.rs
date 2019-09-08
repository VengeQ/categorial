use categorial::Semigroup;
use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Example{
    value:String
}

impl Semigroup<String> for Example{
    fn combine_owned(x: Self, y: Self) -> Self {
        Example{value:x.value.clone().add(&y.value)}
    }
    fn combine(x: &Self, y: &Self) -> Self {
        Example{value:x.value.clone().add(&y.value)}
    }
}

#[cfg(test)]
mod test{
    use self::super::Example;
    use categorial::type_classes::Semigroup;
    #[test]
    fn semigroup_example_test(){
        let e1= Example{value:"hello ".to_owned()};
        let e2= Example{value:"world".to_owned()};
        assert_eq!(Semigroup::combine(&e1,&e2), Example{value:"hello world".to_owned()});
        assert_eq!(Semigroup::combine_owned(e1,e2), Example{value:"hello world".to_owned()});
    }


    #[test]
    fn semigroup_instance_example_test(){
        use categorial::instances::semigroup::SemigroupInstance;
        let e1= SemigroupInstance::new(5);
        let e2= SemigroupInstance::new(20);
        assert_eq!(Semigroup::combine(&e1,&e2), SemigroupInstance::new(25));
        assert_eq!(Semigroup::combine_owned(e1,e2), SemigroupInstance::new(25));
    }
}

