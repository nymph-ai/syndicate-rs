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
pub mod dataspace_patterns;
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

/// A convenient way of starting a service that communicates using
/// Syndicate protocol over standard input and standard
/// output.[^feeble-imitation]
///
/// Expands into a use of `#[tokio:main] async fn main() ...` containing a
/// call to [`stdio_service`][crate::relay::stdio_service], creating a new
/// dataspace and installing a `during!` handler (defined in the
/// syndicate-macros crate) for each pattern/body
/// clause. The created dataspace is installed as the
/// [service
/// object](https://synit.org/book/operation/service.html#service-object)
/// for peers to use to access the service.
///
/// [^feeble-imitation]: In many ways, this macro is a feeble imitation of
///     a Racket
///     [`#lang`](https://docs.racket-lang.org/guide/Module_Syntax.html#%28part._hash-lang%29).
///
/// ### Convenience toplevel `use` declarations
///
/// The expanded code includes these `use` statements *at toplevel*,
/// meaning that e.g. all of `syndicate::actor::*` and all the macros from
/// the `syndicate_macros` crate are in scope for all the code in the
/// module, not just the `$pat`s and `$body`s:
///
/// ```
/// use std::sync::Arc;
/// use syndicate::actor::*;
/// use syndicate::dataspace::Dataspace;
/// use syndicate::enclose;
/// use syndicate::preserves;
/// use syndicate_macros::*;
/// ```
///
/// ### Example
///
/// This is a complete program. (It is included as an example in the
/// `syndicate-macros` crate, as `examples/stdio-ds.rs`.)
///
/// ```ignore (needs syndicate_macros, which this crate cannot depend on)
/// syndicate::stdio_service!{
///     ($v: AnyValue) => |t| {
///         t.on_stop(enclose!((v) move |_t| { tracing::info!("- {v:?}"); Ok(()) }));
///         tracing::info!("+ {v:?}");
///         Ok(())
///     };
/// }
/// ```
///
#[macro_export]
macro_rules! stdio_service {
    {
        $(
            ( $($pat:tt)+ ) => |$t:ident| $body:expr ;
        )*
    } => {
        use std::sync::Arc;
        use syndicate::actor::*;
        use syndicate::dataspace::Dataspace;
        use syndicate::enclose;
        use syndicate::preserves;
        use syndicate_macros::*;

        #[tokio::main]
        async fn main() -> syndicate::actor::ActorResult {
            syndicate::convenient_logging()?;
            syndicate::relay::stdio_service(|t| {
                let service_object = syndicate::actor::Cap::new(&t.create(
                    syndicate::dataspace::Dataspace::new(
                        Some(syndicate::actor::AnyValue::symbol("service")))));
                $(
                    syndicate_macros::during!(
                        t, service_object, $($pat)+,
                        move |$t: &mut syndicate::actor::Activation| {
                            $body
                        });
                )*
                Ok(service_object)
            }).await
        }
    };
}
