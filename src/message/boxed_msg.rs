use super::*;
use std::any::Any;

/// A simple wrapper around a `Box<dyn Any>`.
#[derive(Debug)]
pub struct BoxedMsg(Box<dyn Any + Send>);

impl BoxedMsg {
    pub fn new<M>(sends: Sent<M>) -> Self
    where
        M: Msg,
        Sent<M>: Send + 'static,
    {
        Self(Box::new(sends))
    }

    pub fn downcast<M>(self) -> Result<Sent<M>, Self>
    where
        M: Msg,
        Sent<M>: 'static,
    {
        match self.0.downcast() {
            Ok(cast) => Ok(*cast),
            Err(boxed) => Err(Self(boxed)),
        }
    }

    pub fn downcast_cancel<M>(self, returns: Returned<M>) -> Result<M, Self>
    where
        M: Msg,
        Sent<M>: 'static,
    {
        match self.downcast::<M>() {
            Ok(sends) => Ok(<M::Kind as MsgKind<M>>::cancel(sends, returns)),
            Err(boxed) => Err(boxed),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn boxed_msg() {
        struct Msg1;
        struct Msg2;

        impl Msg for Msg1 {
            type Kind = ();
        }

        impl Msg for Msg2 {
            type Kind = ();
        }

        let boxed = BoxedMsg::new::<Msg1>(Msg1);
        assert!(boxed.downcast::<Msg1>().is_ok());

        let boxed = BoxedMsg::new::<Msg1>(Msg1);
        assert!(boxed.downcast::<Msg2>().is_err());
    }
}
