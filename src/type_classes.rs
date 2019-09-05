pub trait Semigroup<A>  {
    fn combine(x:Self,y:Self) ->Self;
}

