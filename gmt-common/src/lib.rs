pub mod gmt_user;
pub mod repositories;

pub mod messages {
  include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));

  pub use messages::*;
}

mod proto_utils;
