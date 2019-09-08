use crate::Monoid;

///Get 3 instances of monoid and return true if Associativity and Identity rules was completed
///This monoid need to derive (PartialEq, Clone)
/// ```
/// use categorial::Monoid;
/// use std::ops::Add;
/// use categorial::laws::is_monoid_law_complete;
/// #[derive(Clone, PartialEq)]
///        struct ExampleMonoid {
///            value: String
///        }
///        impl Monoid<String> for ExampleMonoid {
///            fn combine_owned(x: Self, y: Self) -> Self {
///                ExampleMonoid{ value: x.value.clone().add(&y.value) }
///            }
///            fn id() -> Self {
///                ExampleMonoid{ value: "".to_string() }
///            }
///        }
///
///        let x1= ExampleMonoid{value:"hello".to_owned()};
///        let x2= ExampleMonoid{value:"beautiful".to_owned()};
///        let x3= ExampleMonoid{value:"world".to_owned()};
///
///        assert_eq!(is_monoid_law_complete(x1,x2,x3), true)
/// ```
pub fn is_monoid_law_complete<A: Monoid<T> + PartialEq + Clone, T>(m1: A, m2: A, m3: A) -> bool {
    let id_complete = Monoid::combine_owned(m1.clone(), A::id()) == Monoid::combine_owned(A::id(), m1.clone()) &&
        Monoid::combine_owned(m1.clone(), A::id()) == m1.clone();
    let assoc_complete = Monoid::combine_owned(Monoid::combine_owned(m1.clone(), m2.clone()), m3.clone()) ==
        Monoid::combine_owned(m1.clone(), Monoid::combine_owned(m2.clone(), m3.clone()));
    id_complete && assoc_complete
}

#[cfg(test)]
mod test {

    use crate::Monoid;
    use std::ops::Add;



    #[test]
    fn is_monoid_law_complete_test_true() {
        #[derive(Clone, PartialEq)]
        struct CorrectMonoid {
            value: String
        }
        impl Monoid<String> for CorrectMonoid {
            fn combine_owned(x: Self, y: Self) -> Self {
                CorrectMonoid{ value: x.value.clone().add(&y.value) }
            }
            fn id() -> Self {
                CorrectMonoid{ value: "".to_string() }
            }
        }

        let x1= CorrectMonoid{value:"hello".to_owned()};
        let x2= CorrectMonoid{value:"beautiful".to_owned()};
        let x3= CorrectMonoid{value:"world".to_owned()};

        assert_eq!(super::is_monoid_law_complete(x1,x2,x3), true)
    }

    ///combine_owned function where `m * id = id * m = m` is not true
    #[test]
    fn is_monoid_law_complete_test_false() {
        #[derive(Clone, PartialEq)]
        struct IncorrectMonoid {
            value: String
        }
        impl Monoid<String> for IncorrectMonoid {
            fn combine_owned(x: Self, y: Self) -> Self {
                IncorrectMonoid{ value: x.value.clone().add(&x.value).add(&y.value) }
            }
            fn id() -> Self {
                IncorrectMonoid{ value: "".to_string() }
            }
        }

        let x1= IncorrectMonoid{value:"hello".to_owned()};
        let x2= IncorrectMonoid{value:"beautiful".to_owned()};
        let x3= IncorrectMonoid{value:"world".to_owned()};

        assert_eq!(super::is_monoid_law_complete(x1,x2,x3), false)
    }
}


