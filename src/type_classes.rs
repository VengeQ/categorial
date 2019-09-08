pub trait Semigroup<A>  {
    fn combine_owned(x:Self,y:Self) ->Self;
}

pub trait Monoid<A> {
    fn combine_owned(x:Self,y:Self) ->Self;
    fn id() -> Self;
}

pub trait Ant<A>{
    fn nothing(&self);
}

