use tiny_actor::Channel;

use crate::message::{Dyn, Protocol};

mod box_channel;
mod can_send;
mod dynamic;
pub use {box_channel::*, can_send::*, dynamic::*};

/// An `ActorType` signifies the type that an actor can be. This can be either
/// a [Protocol] or a [Dyn<_>] type.
pub trait ActorType {
    type Channel: BoxChannel + ?Sized;
}

impl<P> ActorType for P
where
    P: Protocol + Send,
{
    type Channel = Channel<P>;
}

impl<D: ?Sized> ActorType for Dyn<D> {
    type Channel = dyn BoxChannel;
}
