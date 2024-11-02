use options::*;
pub use server::*;
pub use user::*;
#[cfg(test)]
mod client_tests;
mod options;
mod server;
mod user;

#[allow(clippy::needless_raw_strings)]
mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}
