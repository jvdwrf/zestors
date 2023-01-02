// #![feature(async_fn_in_trait)]
// #![feature(return_position_impl_trait_in_trait)]

pub mod channel;
pub mod config;
pub mod error;
pub mod process;
pub mod protocol;
pub mod request;

pub(crate) mod _gen;
pub mod distributed;
// mod supervision_v2;
pub mod supervision_v3;

pub(crate) use {
    channel::*, config::*, distributed::*, error::*, process::*, protocol::*, request::*,
    supervision_v3::*,
};

pub use zestors_codegen::{protocol, Message};
