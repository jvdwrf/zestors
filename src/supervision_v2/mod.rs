pub mod specification;
pub mod supervised;

use std::{marker::PhantomData, time::Duration};

pub use specification::*;
pub use supervised::*;
use tiny_actor::Link;

use crate::process::Child;

async fn example_fn_v2(
    init: SpawnAction<u32, bool>,
    link: Link,
) -> AnyResult<Option<Child<bool, ()>>> {
    match init {
        SpawnAction::Spawn(u32) => todo!(),
        SpawnAction::Respawn(Ok(bool)) => todo!(),
        SpawnAction::Respawn(Err(e)) => todo!(),
    }
}

async fn example_v2() {
    let spec = ChildSpec::new(example_fn_v2, 0, Duration::from_secs(1));
    let mut child = spec.spawn().await.unwrap();
    child.supervise().await;
    match child.respawn().await {
        Ok(supervision) => match supervision {
            Supervision::Restarted => println!("Child restarted successfully"),
            Supervision::Finished => println!("Child finished successfully"),
        },
        Err(e) => println!("Restarting has failed"),
    }

    let spec = ChildSpec::new(example_fn_v2, 0, Duration::from_secs(1)).into_dyn();
    let mut child = spec.spawn().await.unwrap();
    child.supervise().await;
    match child.respawn().await {
        Ok(supervision) => match supervision {
            Supervision::Restarted => println!("Child restarted successfully"),
            Supervision::Finished => println!("Child finished successfully"),
        },
        Err(e) => println!("Restarting has failed"),
    }
}
