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

/// Retrieve a string representing the current time, expressed in the timestamp format from the
/// tai64n program, <https://cr.yp.to/daemontools/tai64n.html>.
pub fn tai64n_label() -> String {
    use std::fmt::Write;
    let now = tai64::Tai64N::now();
    // We preallocate exactly 25 characters.
    let mut result = String::with_capacity(25);
    write!(result, "@{:0>16x}{:0>8x}", now.0.0, now.1).unwrap();
    result
}

// Exists solely for use in convenient_logging() below.
struct Tai64NTime;
impl tracing_subscriber::fmt::time::FormatTime for Tai64NTime {
    fn format_time(&self, w: &mut tracing_subscriber::fmt::format::Writer<'_>) -> std::fmt::Result {
        let now = tai64::Tai64N::now();
        write!(w, "@{:0>16x}{:0>8x}", now.0.0, now.1)
    }
}

#[doc(hidden)]
pub fn use_ansi_color<T: std::io::IsTerminal>(sink: Option<&T>) -> bool {
    let no_color = std::env::var("NO_COLOR").unwrap_or("".to_owned()).len() > 0;
    let clicolor_force = std::env::var("CLICOLOR_FORCE").unwrap_or("".to_owned()).len() > 0;
    !no_color && (clicolor_force || sink.is_some_and(|t| t.is_terminal()))
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
        .with_ansi(use_ansi_color(Some(&std::io::stderr())))
        .with_thread_ids(std::env::var("SYNDICATE_LOG_THREAD_IDS").unwrap_or("".to_owned()).len() > 0)
        .with_max_level(tracing::Level::TRACE)
        .with_env_filter(filter)
        .with_writer(std::io::stderr);

    use tracing::subscriber::set_global_default;
    (match std::env::var("SYNDICATE_LOG_TIMESTAMP").unwrap_or("".to_owned()).as_ref() {
        "default" =>
            set_global_default(subscriber.finish()),
        "tai64n" =>
            set_global_default(subscriber.with_timer(Tai64NTime).finish()),
        "" | "none" | _ =>
            set_global_default(subscriber.without_time().finish()),
    }).expect("Could not set tracing global subscriber");

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
