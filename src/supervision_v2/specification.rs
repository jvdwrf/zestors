use super::*;
use crate::{self as zestors, *};
use async_trait::async_trait;
use futures::{future::BoxFuture, Future};
use std::{error::Error, marker::PhantomData, process::Output, time::Duration};
use tiny_actor::{Capacity, Link};

pub(super) type AnyResult<T> = Result<T, Box<dyn Error + Send>>;

/// A child-specification defines exactly how a child may be spawned and subsequently
/// restarted. The child-specification can be added to a supervisor in order to automatically
/// spawn and supervise the process.
pub struct ChildSpec<P, E, I, SpawnFn, SpawnFut> {
    spawn_fn: SpawnFn,
    init: I,
    link: Link,
    phantom_data: PhantomData<(SpawnFut, P, E)>,
}

impl<P, E, I, SpawnFn, SpawnFut> Clone for ChildSpec<P, E, I, SpawnFn, SpawnFut>
where
    I: Clone,
    SpawnFn: Clone,
{
    fn clone(&self) -> Self {
        Self {
            spawn_fn: self.spawn_fn.clone(),
            init: self.init.clone(),
            link: self.link.clone(),
            phantom_data: self.phantom_data.clone(),
        }
    }
}

impl<P, E, I, SpawnFn, SpawnFut> ChildSpec<P, E, I, SpawnFn, SpawnFut>
where
    P: Protocol + Send,
    E: Send + 'static,
    I: Send + 'static,
    SpawnFn: Fn(SpawnAction<I, E>, Link) -> SpawnFut + Send + 'static,
    SpawnFut: Future<Output = AnyResult<Option<Child<E, P>>>> + Send + 'static,
{
    pub fn new(spawn_fn: SpawnFn, init: I, duration: Duration) -> Self {
        Self {
            spawn_fn,
            init,
            link: Link::Attached(duration),
            phantom_data: PhantomData,
        }
    }

    pub fn into_dyn(self) -> DynamicChildSpec {
        DynamicChildSpec(Box::new(self))
    }

    pub async fn spawn(self) -> AnyResult<SupervisedChild<P, E, I, SpawnFn, SpawnFut>> {
        let spawn_fut = (self.spawn_fn)(SpawnAction::Spawn(self.init), self.link);

        match spawn_fut.await {
            Ok(Some(child)) => Ok(SupervisedChild::new(child, self.spawn_fn)),
            Ok(None) => panic!("May not return None when spawning for the first time"),
            Err(e) => Err(e),
        }
    }
}

pub struct DynamicChildSpec(Box<dyn SpecifiesChild + Send>);

impl DynamicChildSpec {
    pub async fn spawn(self) -> AnyResult<DynamicSupervisedChild> {
        Ok(DynamicSupervisedChild(self.0.spawn_boxed().await?))
    }
}

#[async_trait]
pub(super) trait SpecifiesChild {
    /// Spawn the supervi
    async fn spawn_boxed(self: Box<Self>) -> AnyResult<Box<dyn Supervisable + Send>>;
    fn link(&self) -> &Link;
    fn link_mut(&mut self) -> &mut Link;
}

#[async_trait]
impl<P, E, I, SpawnFn, SpawnFut> SpecifiesChild for ChildSpec<P, E, I, SpawnFn, SpawnFut>
where
    P: Protocol + Send,
    E: Send + 'static,
    I: Send + 'static,
    SpawnFn: Fn(SpawnAction<I, E>, Link) -> SpawnFut + Send + 'static,
    SpawnFut: Future<Output = AnyResult<Option<Child<E, P>>>> + Send + 'static,
{
    async fn spawn_boxed(self: Box<Self>) -> AnyResult<Box<dyn Supervisable + Send>> {
        Ok(Box::new(self.spawn().await?))
    }

    fn link(&self) -> &Link {
        &self.link
    }

    fn link_mut(&mut self) -> &mut Link {
        &mut self.link
    }
}

pub enum SpawnAction<I, E> {
    Spawn(I),
    Respawn(Result<E, ExitError>),
}
