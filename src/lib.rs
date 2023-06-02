extern crate thrift;
extern crate ordered_float;
extern crate try_from;

mod fb303;
mod hive_metastore;

pub use fb303::*;
pub use hive_metastore::*;

pub const GREETING: &'static str = "together, forever";
