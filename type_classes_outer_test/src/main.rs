use categorial::type_classes::Semigroup;
use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Example{
    value:String
}

impl Semigroup<String> for Example{
    fn combine(x: Self, y: Self) -> Self {
        Example{value:x.value.clone().add(&y.value)}
    }
}


#[cfg(test)]
mod test{
    use crate::Example;
    use categorial::type_classes::Semigroup;
    #[test]
    fn semigroup_example_test(){
        let e1= Example{value:"hello ".to_owned()};
        let e2= Example{value:"world".to_owned()};
        assert_eq!(Semigroup::combine(e1,e2), Example{value:"hello world".to_owned()});
    }
}

