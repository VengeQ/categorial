pub trait Semigroup<A>  {
    fn combine(x:Self,y:Self) ->Self;
}

pub trait Monoid<A> {
    fn combine(x:Self,y:Self) ->Self;
    fn id() -> Self;
}

