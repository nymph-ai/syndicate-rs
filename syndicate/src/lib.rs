#![doc = include_str!("../README.md")]
#![feature(min_specialization)]

#[doc(inline)]
pub use preserves;

#[doc(inline)]
pub use preserves_schema;

pub mod actor;
pub mod bag;
pub mod dataflow;
pub mod dataspace;
pub mod during;
pub mod error;
#[doc(hidden)]
pub mod pattern;
pub mod relay;
pub mod rewrite;
pub mod rpc;
pub mod supervise;

pub mod schemas {
    //! Auto-generated codecs for [Syndicate protocol
    //! schemas](https://git.syndicate-lang.org/syndicate-lang/syndicate-protocols/src/branch/main/schemas).
    include!(concat!(env!("OUT_DIR"), "/src/schemas/mod.rs"));
}

pub mod skeleton;
pub mod sturdy;
pub mod trace;

#[doc(inline)]
pub use during::entity;

use std::fmt::Write;
pub fn tai64n_label()-> String {
  let now = tai64::Tai64N::now();
  // Only need a 25 characters.
  let mut result = String::with_capacity(32);
  write!(result, "@{:0>16x}{:0>8x}", now.0.0, now.1).unwrap();
  result
}

struct Tai64NTime;

use tracing_subscriber::fmt::format::Writer;
impl tracing_subscriber::fmt::time::FormatTime for Tai64NTime {

    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        let now = tai64::Tai64N::now();
        write!(w, "@{:0>16x}{:0>8x}", now.0.0, now.1)
    }

}

/// Sets up [`tracing`] logging in a reasonable way.
///
/// Useful at the top of `main` functions.
pub fn convenient_logging() -> actor::ActorResult {
    let filter = match std::env::var(tracing_subscriber::filter::EnvFilter::DEFAULT_ENV) {
        Err(std::env::VarError::NotPresent) =>
            tracing_subscriber::filter::EnvFilter::default()
            .add_directive(tracing_subscriber::filter::LevelFilter::INFO.into()),
        _ =>
            tracing_subscriber::filter::EnvFilter::try_from_default_env()?,
    };
    let subscriber = tracing_subscriber::fmt()
        .with_ansi(false)
        .with_thread_ids(false)
        .with_max_level(tracing::Level::TRACE)
        .with_env_filter(filter)
        .with_timer(Tai64NTime)
        .with_writer(std::io::stderr)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Could not set tracing global subscriber");
    Ok(())
}

/// Retrieve the version of the Syndicate crate.
pub fn syndicate_package_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod protocol_test {
    use crate::*;
    use preserves::*;
    use preserves_schema::Deserialize;

    #[test] fn decode_sync() {
        let input_str = "[[2 <S #:[0 11]>]]";
        let mut src = BytesBinarySource::new(input_str.as_bytes());
        let mut r = src.text();
        let packet: schemas::protocol::Packet<IOValue> =
            schemas::protocol::Packet::deserialize(&mut r, &mut IOValueDomainDecode).unwrap();
        println!("{:?}", packet);
    }
}
