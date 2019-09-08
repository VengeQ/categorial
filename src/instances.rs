pub mod semigroup;


///Base instances for some type classes.
///
/// This crate provides some implementation of type classes.
/// For all monoid instances associative operations are adding and appending.
/// MonoidVec derive Clone, Debug, PartialEq
/// # Example
/// ```
/// use categorial::instances::monoid::MonoidVec;
/// use categorial::Monoid;
/// let m0 = Monoid::id();
/// let m1 = MonoidVec::new(vec![1,2,3,4]);
/// let m2 = MonoidVec::new(vec![5,6,7,8]);
/// let m3 = MonoidVec::new(vec!["Hello".to_owned(),"Beautiful".to_owned(),"World".to_owned()]);
/// assert_eq!(Monoid::combine_owned(m1, m2),MonoidVec::new(vec![1,2,3,4,5,6,7,8]));
/// assert_eq!(Monoid::combine_owned(m0, m3.clone()), m3); //inference correct type for m0;
///
/// ```

pub mod monoid;