// use std::{error::Error, marker::PhantomData, process::Output, time::Duration};

// use async_trait::async_trait;
// use futures::{future::BoxFuture, Future};
// use tiny_actor::{Capacity, Link};
// use zestors::{
//     config::Config,
//     error::ExitError,
//     process::{spawn_process, Address, Child, Inbox},
//     protocol::Protocol,
//     supervision::{
//         ChildSpec, RestartError, RestartStrategy, ShutdownStrategy, SpecifiesChild, Supervisable,
//         SupervisionStrategy, SupervisorBuilder,
//     },
//     *,
// };

// // // Define the message
// // #[protocol]
// // pub enum MyProtocol {
// //     Variant(u32),
// // }

// // struct MyInit;
// // struct MyExit;

// // // and the spawn function
// // async fn run_my_actor(mut inbox: Inbox<MyProtocol>, init: MyInit) -> MyExit {
// //     loop {
// //         let MyProtocol::Variant(msg) = inbox.recv().await.unwrap();
// //         if msg == 0 {
// //             break MyExit;
// //         } else {
// //             break MyExit;
// //         }
// //     }
// // }

// // fn restart_my_actor(
// //     exit: Result<MyExit, ExitError>,
// // ) -> Option<impl Future<Output = Result<MyInit, RestartError>>> {
// //     Some(async move {
// //         match exit {
// //             Ok(_) => Ok(MyInit),
// //             Err(_e) => Err(RestartError::CouldNotSpawn),
// //         }
// //     })
// // }

// // #[tokio::main]
// // async fn main() {
// //     let (child, address) = SupervisorBuilder::new(
// //         SupervisionStrategy::OneForOne {
// //             max: 10,
// //             within: Duration::from_secs(10),
// //         },
// //         ShutdownStrategy::HaltThenAbort(Duration::from_secs(1)),
// //         RestartStrategy::NormalOnly,
// //     )
// //     .add_child(ChildSpec {
// //         run_fn: run_my_actor,
// //         init: MyInit,
// //         restart_fn: restart_my_actor,
// //         config: Config::default(),
// //     })
// //     .add_child(ChildSpec {
// //         run_fn: run_my_actor,
// //         init: MyInit,
// //         restart_fn: restart_my_actor,
// //         config: Config::default(),
// //     })
// //     .spawn();
// // }

// fn main() {}

// #[async_trait]
// trait SpawnFn {
//     async fn spawn(self, config: Config) -> Box<dyn Supervisable + Send>;
// }

// #[async_trait]
// trait RestartFn {
//     async fn restart(&mut self) -> Result<(), RestartError>;
// }

// struct Specification<Prot, Exit, Init, SpawnFn, SpawnFut, ToRestartFn, ToRestartFut>
// where
//     Prot: Protocol,
//     Exit: Send + 'static,
//     Init: Send + 'static,
//     SpawnFn: Fn(Init, Link) -> SpawnFut,
//     SpawnFut: Future<Output = (Child<Exit, Prot>, Address<Prot>)>,
//     ToRestartFn: Fn(Result<Exit, ExitError>) -> ToRestartFut,
//     ToRestartFut: Future<Output = Option<Init>>,
// {
//     id: Option<String>,
//     spawn_fn: SpawnFn, // async fn (Inbox<Protocol>, Init) -> (Child<Exit, Prot>, Address<Prot>)
//     to_restart: ToRestartFn, // async fn(Exit) -> Option<Init>
//     init: Init,
//     link: Link,
//     phantom_data: PhantomData<(Exit, Prot, SpawnFut, ToRestartFut)>,
// }

fn main() {}

// struct Supervised<Prot, Exit, Init, SpawnFn, SpawnFut, ToRestartFn, ToRestartFut>
// where
//     Prot: Protocol,
//     Exit: Send + 'static,
//     Init: Send + 'static,
//     SpawnFn: Fn(Init, Link) -> SpawnFut,
//     SpawnFut: Future<Output = (Child<Exit, Prot>, Address<Prot>)>,
//     ToRestartFn: Fn(Result<Exit, ExitError>) -> ToRestartFut,
//     ToRestartFut: Future<Output = Option<Init>>,
// {
//     spawn_fn: SpawnFn, // async fn (Init, Link) -> (Child<Exit, Prot>, Address<Prot>)
//     to_restart: ToRestartFn, // async fn(Exit) -> Option<Init>
//     child: Child<Exit, Prot>,
//     phantom_data: PhantomData<(Exit, Prot, SpawnFut, ToRestartFut)>,
// }

// async fn example_spawn_fn(init: u32, link: Link) -> (Child<bool, ()>, Address<()>) {
//     spawn_process(
//         Config {
//             link,
//             capacity: Capacity::default(),
//         },
//         |inbox| async move { todo!() },
//     )
// }

// async fn example_restart_fn(exit: Result<bool, ExitError>) -> Option<u32> {
//     match exit {
//         Ok(true) => Some(1),
//         Ok(false) => Some(10),
//         Err(_) => None,
//     }
// }

// fn example() {
//     let x = Specification {
//         id: Some("example".to_string()),
//         spawn_fn: example_spawn_fn,
//         to_restart: example_restart_fn,
//         init: 0,
//         link: Link::Attached(Duration::from_secs(1)),
//         phantom_data: PhantomData,
//     };
// }

// //------------------------------------------------------------------------------------------------
// //  V2
// //------------------------------------------------------------------------------------------------

