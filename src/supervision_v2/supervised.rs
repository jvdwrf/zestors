use super::*;
use crate::{self as zestors, *};
use async_trait::async_trait;
use futures::{future::BoxFuture, Future};
use std::{error::Error, marker::PhantomData, mem, pin::Pin, process::Output, time::Duration};
use tiny_actor::{Capacity, Link};

pub struct SupervisedChild<P, E, I, SpawnFn, SpawnFut>
where
    P: Protocol + Send,
    E: Send + 'static,
    I: Send + 'static,
    SpawnFn: Fn(SpawnAction<I, E>, Link) -> SpawnFut + Send + 'static,
    SpawnFut: Future<Output = AnyResult<Option<Child<E, P>>>> + Send + 'static,
{
    child: Child<E, P>,
    spawn_fn: SpawnFn,
    respawn_fut: Option<Pin<Box<SpawnFut>>>, // todo: remove Pin<Box<_>>
    phantom_data: PhantomData<I>,
}

impl<P, E, I, SpawnFn, SpawnFut> SupervisedChild<P, E, I, SpawnFn, SpawnFut>
where
    P: Protocol + Send,
    E: Send + 'static,
    I: Send + 'static,
    SpawnFn: Fn(SpawnAction<I, E>, Link) -> SpawnFut + Send + 'static,
    SpawnFut: Future<Output = AnyResult<Option<Child<E, P>>>> + Send + 'static,
{
    pub fn new(child: Child<E, P>, spawn_fn: SpawnFn) -> Self {
        assert!(
            child.is_attached(),
            "Can't supervise child that is not attached"
        );

        Self {
            child,
            spawn_fn,
            respawn_fut: None,
            phantom_data: PhantomData,
        }
    }

    pub fn into_dyn(self) -> DynamicSupervisedChild {
        DynamicSupervisedChild(Box::new(self))
    }

    pub async fn supervise(&mut self) {
        let exit = (&mut self.child).await;
        let link = self.child.link();
        self.respawn_fut = Some(Box::pin((self.spawn_fn)(
            SpawnAction::Respawn(exit),
            link.clone(),
        )));
    }

    pub async fn respawn(&mut self) -> Result<Supervision, RespawnError> {
        let respawn_fut = self.respawn_fut.as_mut().expect("Should have exited");

        match respawn_fut.await {
            Ok(Some(mut child)) => {
                mem::swap(&mut self.child, &mut child);
                Ok(Supervision::Restarted)
            }
            Ok(None) => Ok(Supervision::Finished),
            Err(e) => Err(RespawnError(e)),
        }
    }
}

pub struct DynamicSupervisedChild(pub(super) Box<dyn Supervisable + Send>);

impl DynamicSupervisedChild {
    pub async fn supervise(&mut self) {
        self.0.supervise().await
    }

    pub async fn respawn(&mut self) -> Result<Supervision, RespawnError> {
        self.0.respawn().await
    }
}

pub enum Supervision {
    Restarted,
    Finished,
}

#[async_trait]
pub(super) trait Supervisable {
    async fn supervise(&mut self);
    async fn respawn(&mut self) -> Result<Supervision, RespawnError>;
}

#[async_trait]
impl<P, E, I, SpawnFn, SpawnFut> Supervisable for SupervisedChild<P, E, I, SpawnFn, SpawnFut>
where
    P: Protocol + Send,
    E: Send + 'static,
    I: Send + 'static,
    SpawnFn: Fn(SpawnAction<I, E>, Link) -> SpawnFut + Send,
    SpawnFut: Future<Output = AnyResult<Option<Child<E, P>>>> + Send + 'static,
{
    async fn supervise(&mut self) {
        self.supervise().await
    }

    async fn respawn(&mut self) -> Result<Supervision, RespawnError> {
        self.respawn().await
    }
}

/// Respawning has failed
pub struct RespawnError(pub Box<dyn Error + Send>);

impl<E: Error + Send + 'static> From<Box<E>> for RespawnError {
    fn from(value: Box<E>) -> Self {
        Self(value)
    }
}
