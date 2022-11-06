use crate::{
    message::{AcceptsDyn, Dyn},
    process::Address,
};

use super::*;

pub trait IntoAddress<T: ActorType> {
    fn into_address(self) -> Address<T>;
}

impl<R: ?Sized, T: ?Sized> IntoAddress<Dyn<T>> for Address<Dyn<R>>
where
    Dyn<R>: ActorType<Channel = dyn BoxChannel> + AcceptsDyn<Dyn<T>>,
{
    fn into_address(self) -> Address<Dyn<T>> {
        self.transform()
    }
}

impl<P, T> IntoAddress<Dyn<T>> for Address<P>
where
    P: Protocol + AcceptsDyn<Dyn<T>>,
    T: ?Sized,
{
    fn into_address(self) -> Address<Dyn<T>> {
        self.into_dyn()
    }
}
