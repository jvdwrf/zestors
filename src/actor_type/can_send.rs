use crate::{
    error::{SendUncheckedError, TrySendUncheckedError},
    message::{Accepts, AcceptsDyn, AcceptsOne, Dyn, Msg, MsgKind, Protocol, Returned, Sent},
    process::SendFut,
    *,
};
use tiny_actor::{SendError, TrySendError};

use super::*;

/// Whether an actor accepts messages of a certain kind. If this is implemented for the
/// [ActorType] then messages of type `M` can be sent to it's address.
pub trait CanSend<M: Msg>: ActorType {
    fn try_send(address: &Self::Channel, msg: M) -> Result<Returned<M>, TrySendError<M>>;
    fn send_now(address: &Self::Channel, msg: M) -> Result<Returned<M>, TrySendError<M>>;
    fn send_blocking(address: &Self::Channel, msg: M) -> Result<Returned<M>, SendError<M>>;
    fn send(address: &Self::Channel, msg: M) -> SendFut<'_, M>;
}

impl<M, T> CanSend<M> for Dyn<T>
where
    Self: AcceptsDyn<Dyn<dyn AcceptsOne<M>>>,
    M: Msg + Send + 'static,
    Sent<M>: Send + 'static,
    Returned<M>: Send,
    T: ?Sized,
{
    fn try_send(address: &Self::Channel, msg: M) -> Result<Returned<M>, TrySendError<M>> {
        address.try_send_unchecked(msg).map_err(|e| match e {
            TrySendUncheckedError::Full(msg) => TrySendError::Full(msg),
            TrySendUncheckedError::Closed(msg) => TrySendError::Closed(msg),
            TrySendUncheckedError::NotAccepted(_) => {
                panic!("Sent message which was not accepted by actor")
            }
        })
    }

    fn send_now(address: &Self::Channel, msg: M) -> Result<Returned<M>, TrySendError<M>> {
        address.send_now_unchecked(msg).map_err(|e| match e {
            TrySendUncheckedError::Full(msg) => TrySendError::Full(msg),
            TrySendUncheckedError::Closed(msg) => TrySendError::Closed(msg),
            TrySendUncheckedError::NotAccepted(_) => {
                panic!("Sent message which was not accepted by actor")
            }
        })
    }

    fn send_blocking(address: &Self::Channel, msg: M) -> Result<Returned<M>, SendError<M>> {
        address.send_blocking_unchecked(msg).map_err(|e| match e {
            SendUncheckedError::Closed(msg) => SendError(msg),
            SendUncheckedError::NotAccepted(_) => {
                panic!("Sent message which was not accepted by actor")
            }
        })
    }

    fn send(address: &Self::Channel, msg: M) -> SendFut<'_, M> {
        SendFut(Box::pin(async move {
            address.send_unchecked(msg).await.map_err(|e| match e {
                SendUncheckedError::Closed(msg) => SendError(msg),
                SendUncheckedError::NotAccepted(_) => {
                    panic!("Sent message which was not accepted by actor")
                }
            })
        }))
    }
}

impl<M, P> CanSend<M> for P
where
    P: Protocol + Accepts<M>,
    Returned<M>: Send,
    Sent<M>: Send + 'static,
    M: Msg + Send + 'static,
{
    fn try_send(address: &Self::Channel, msg: M) -> Result<Returned<M>, TrySendError<M>> {
        let (sends, returns) = <M::Kind as MsgKind<M>>::create(msg);

        match address.try_send(P::from_msg(sends)) {
            Ok(()) => Ok(returns),
            Err(e) => match e {
                TrySendError::Closed(prot) => {
                    Err(TrySendError::Closed(prot.unwrap_and_cancel(returns)))
                }
                TrySendError::Full(prot) => {
                    Err(TrySendError::Full(prot.unwrap_and_cancel(returns)))
                }
            },
        }
    }

    fn send_now(address: &Self::Channel, msg: M) -> Result<Returned<M>, TrySendError<M>> {
        let (sends, returns) = <M::Kind as MsgKind<M>>::create(msg);

        match address.send_now(P::from_msg(sends)) {
            Ok(()) => Ok(returns),
            Err(e) => match e {
                TrySendError::Closed(prot) => {
                    Err(TrySendError::Closed(prot.unwrap_and_cancel(returns)))
                }
                TrySendError::Full(prot) => {
                    Err(TrySendError::Full(prot.unwrap_and_cancel(returns)))
                }
            },
        }
    }

    fn send_blocking(address: &Self::Channel, msg: M) -> Result<Returned<M>, SendError<M>> {
        let (sends, returns) = <M::Kind as MsgKind<M>>::create(msg);

        match address.send_blocking(P::from_msg(sends)) {
            Ok(()) => Ok(returns),
            Err(SendError(prot)) => Err(SendError(prot.unwrap_and_cancel(returns))),
        }
    }

    fn send(address: &Self::Channel, msg: M) -> SendFut<'_, M> {
        SendFut(Box::pin(async move {
            let (sends, returns) = <M::Kind as MsgKind<M>>::create(msg);

            match address.send(P::from_msg(sends)).await {
                Ok(()) => Ok(returns),
                Err(SendError(prot)) => Err(SendError(prot.unwrap_and_cancel(returns))),
            }
        }))
    }
}
