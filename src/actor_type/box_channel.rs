use crate::{
    error::{SendUncheckedError, TrySendUncheckedError},
    message::{BoxedMsg, Msg, MsgKind, Returned, Sent},
};

use super::*;
use futures::Future;
use std::{any::TypeId, fmt::Debug, pin::Pin};
use tiny_actor::{Channel, SendError, TrySendError};

/// The internal channel-trait used within zestors, which allows for sending messages dynamically.
///
/// This never has to be implemented manually.
pub trait BoxChannel: tiny_actor::AnyChannel + tiny_actor::DynChannel + Debug {
    fn try_send_boxed(&self, boxed: BoxedMsg) -> Result<(), TrySendUncheckedError<BoxedMsg>>;
    fn send_now_boxed(&self, boxed: BoxedMsg) -> Result<(), TrySendUncheckedError<BoxedMsg>>;
    fn send_blocking_boxed(&self, boxed: BoxedMsg) -> Result<(), SendUncheckedError<BoxedMsg>>;
    fn send_boxed<'a>(
        &'a self,
        boxed: BoxedMsg,
    ) -> Pin<Box<dyn Future<Output = Result<(), SendUncheckedError<BoxedMsg>>> + Send + 'a>>;
    fn accepts(&self, id: &TypeId) -> bool;
}

impl<P: Protocol> BoxChannel for Channel<P> {
    fn try_send_boxed(&self, boxed: BoxedMsg) -> Result<(), TrySendUncheckedError<BoxedMsg>> {
        match P::try_from_box(boxed) {
            Ok(prot) => self.try_send(prot).map_err(|e| match e {
                TrySendError::Full(prot) => TrySendUncheckedError::Full(prot.into_box()),
                TrySendError::Closed(prot) => TrySendUncheckedError::Closed(prot.into_box()),
            }),
            Err(boxed) => Err(TrySendUncheckedError::NotAccepted(boxed)),
        }
    }

    fn send_now_boxed(&self, boxed: BoxedMsg) -> Result<(), TrySendUncheckedError<BoxedMsg>> {
        match P::try_from_box(boxed) {
            Ok(prot) => self.send_now(prot).map_err(|e| match e {
                TrySendError::Full(prot) => TrySendUncheckedError::Full(prot.into_box()),
                TrySendError::Closed(prot) => TrySendUncheckedError::Closed(prot.into_box()),
            }),
            Err(boxed) => Err(TrySendUncheckedError::NotAccepted(boxed)),
        }
    }

    fn send_blocking_boxed(&self, boxed: BoxedMsg) -> Result<(), SendUncheckedError<BoxedMsg>> {
        match P::try_from_box(boxed) {
            Ok(prot) => self
                .send_blocking(prot)
                .map_err(|SendError(prot)| SendUncheckedError::Closed(prot.into_box())),
            Err(boxed) => Err(SendUncheckedError::NotAccepted(boxed)),
        }
    }

    fn send_boxed<'a>(
        &'a self,
        boxed: BoxedMsg,
    ) -> Pin<Box<dyn Future<Output = Result<(), SendUncheckedError<BoxedMsg>>> + Send + 'a>> {
        Box::pin(async move {
            match P::try_from_box(boxed) {
                Ok(prot) => self
                    .send(prot)
                    .await
                    .map_err(|SendError(prot)| SendUncheckedError::Closed(prot.into_box())),
                Err(boxed) => Err(SendUncheckedError::NotAccepted(boxed)),
            }
        })
    }

    fn accepts(&self, id: &TypeId) -> bool {
        <P as Protocol>::accepts_msg(id)
    }
}

impl dyn BoxChannel {
    pub(crate) fn try_send_unchecked<M>(
        &self,
        msg: M,
    ) -> Result<Returned<M>, TrySendUncheckedError<M>>
    where
        M: Msg,
        Sent<M>: Send + 'static,
    {
        let (sends, returns) = <M::Kind as MsgKind<M>>::create(msg);
        let res = self.try_send_boxed(BoxedMsg::new::<M>(sends));

        match res {
            Ok(()) => Ok(returns),
            Err(e) => match e {
                TrySendUncheckedError::Full(boxed) => Err(TrySendUncheckedError::Full(
                    boxed.downcast_cancel(returns).unwrap(),
                )),
                TrySendUncheckedError::Closed(boxed) => Err(TrySendUncheckedError::Closed(
                    boxed.downcast_cancel(returns).unwrap(),
                )),
                TrySendUncheckedError::NotAccepted(boxed) => Err(
                    TrySendUncheckedError::NotAccepted(boxed.downcast_cancel(returns).unwrap()),
                ),
            },
        }
    }

    pub(crate) fn send_now_unchecked<M>(
        &self,
        msg: M,
    ) -> Result<Returned<M>, TrySendUncheckedError<M>>
    where
        M: Msg,
        Sent<M>: Send + 'static,
    {
        let (sends, returns) = <M::Kind as MsgKind<M>>::create(msg);
        let res = self.send_now_boxed(BoxedMsg::new::<M>(sends));

        match res {
            Ok(()) => Ok(returns),
            Err(e) => match e {
                TrySendUncheckedError::Full(boxed) => Err(TrySendUncheckedError::Full(
                    boxed.downcast_cancel(returns).unwrap(),
                )),
                TrySendUncheckedError::Closed(boxed) => Err(TrySendUncheckedError::Closed(
                    boxed.downcast_cancel(returns).unwrap(),
                )),
                TrySendUncheckedError::NotAccepted(boxed) => Err(
                    TrySendUncheckedError::NotAccepted(boxed.downcast_cancel(returns).unwrap()),
                ),
            },
        }
    }

    pub(crate) fn send_blocking_unchecked<M>(
        &self,
        msg: M,
    ) -> Result<Returned<M>, SendUncheckedError<M>>
    where
        M: Msg,
        Sent<M>: Send + 'static,
    {
        let (sends, returns) = <M::Kind as MsgKind<M>>::create(msg);
        let res = self.send_blocking_boxed(BoxedMsg::new::<M>(sends));

        match res {
            Ok(()) => Ok(returns),
            Err(e) => match e {
                SendUncheckedError::Closed(boxed) => Err(SendUncheckedError::Closed(
                    boxed.downcast_cancel(returns).unwrap(),
                )),
                SendUncheckedError::NotAccepted(boxed) => Err(SendUncheckedError::NotAccepted(
                    boxed.downcast_cancel(returns).unwrap(),
                )),
            },
        }
    }

    pub(crate) async fn send_unchecked<M>(
        &self,
        msg: M,
    ) -> Result<Returned<M>, SendUncheckedError<M>>
    where
        M: Msg,
        Sent<M>: Send + 'static,
    {
        let (sends, returns) = <M::Kind as MsgKind<M>>::create(msg);
        let res = self.send_boxed(BoxedMsg::new::<M>(sends)).await;

        match res {
            Ok(()) => Ok(returns),
            Err(e) => match e {
                SendUncheckedError::Closed(boxed) => Err(SendUncheckedError::Closed(
                    boxed.downcast_cancel(returns).unwrap(),
                )),
                SendUncheckedError::NotAccepted(boxed) => Err(SendUncheckedError::NotAccepted(
                    boxed.downcast_cancel(returns).unwrap(),
                )),
            },
        }
    }
}
