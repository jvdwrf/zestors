use super::*;
use std::any::TypeId;

/// The [Msg] trait must be implemented for all messages you would like to send.
/// This trait defines of what [MsgKind] the message is. By using a different
/// [MsgKind], behaviour of sending can be changed. (For example to require a
/// reply)
///
/// This trait can be derived using [`#[derive(Debug)]`](crate::Msg!).
pub trait Msg: Sized {
    /// The type of this message.
    type Kind: MsgKind<Self>;
}

/// The [MsgKind] trait is implemented for custom message types. The [MsgKind] decides
/// what happens when a message is created/sent, and when it is canceled. For most uses,
/// this does not have to be implemented manually, but one of the following can be used:
/// - `()`: For sending one-off messages.
/// - `Rx<T>`: For messages with reply of type `T`.
pub trait MsgKind<M> {
    /// The message that is sent.
    type Sent;

    /// The value that is returned when a message is sent.
    type Returned;

    /// This is called before the message is sent.
    fn create(msg: M) -> (Self::Sent, Self::Returned);

    /// This is called if the message cannot be sent succesfully.
    fn cancel(sent: Self::Sent, returned: Self::Returned) -> M;
}

/// Every actor has to define a [Protocol], which defines the messages that it
/// [Accepts].
///
/// This can be derived with [`#derive[protocol]`](crate::protocol!).
pub trait Protocol: Send + 'static + Sized {
    /// Convert the protocol into a [BoxedMsg].
    fn into_box(self) -> BoxedMsg;

    /// Attempt to convert a [BoxedMsg] into the [Protocol].
    ///
    /// This should succeed if the [Protocol] [Accepts] a message, otherwise this
    /// should fail.
    fn try_from_box(boxed: BoxedMsg) -> Result<Self, BoxedMsg>;

    /// Whether the [Protocol] accepts a message. The `TypeId` is that of the
    /// message.
    ///
    /// This should succeed if the [Protocol] [Accepts] a message, otherwise this
    /// should fail.
    fn accepts_msg(msg_id: &TypeId) -> bool;
}

/// The trait [Accepts<M>] should be implemented for all messages `M` that a
/// [Protocol] accepts.
///
/// This can be derived with [`#derive[protocol]`](crate::protocol!).
pub trait Accepts<M: Msg> {
    /// Convert the [Msg] into the [Protocol].
    fn from_msg(msg: Sent<M>) -> Self
    where
        Self: Sized;

    /// Attempt to convert the [Protocol] into a specific [Msg] if it is of that
    /// type.
    fn try_into_msg(self) -> Result<Sent<M>, Self>
    where
        Self: Sized;

    /// Automatically implemented method to unwrap and cancel a message.
    fn unwrap_and_cancel(self, returned: Returned<M>) -> M
    where
        Self: Sized,
    {
        if let Ok(sent) = self.try_into_msg() {
            <M::Kind as MsgKind<M>>::cancel(sent, returned)
        } else {
            panic!("")
        }
    }
}

/// A shorthand for writing [<M::Type as MsgType<M>>::Sent](MsgType).
pub type Sent<M> = <<M as Msg>::Kind as MsgKind<M>>::Sent;

/// A shorthand for writing [<M::Type as MsgType<M>>::Returned](MsgType).
pub type Returned<M> = <<M as Msg>::Kind as MsgKind<M>>::Returned;

mod test {
    use super::*;

    #[allow(unused)]
    type TestDyn1 = Box<dyn Accepts<u32>>;
    #[allow(unused)]
    type TestDyn2 = Box<dyn Protocol>;
}
