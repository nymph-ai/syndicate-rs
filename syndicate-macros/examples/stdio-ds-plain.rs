use syndicate_macros::*;
use syndicate::enclose;

use syndicate::actor::*;
use syndicate::dataspace::*;
use syndicate::preserves::*;

#[tokio::main]
async fn main() -> ActorResult {
    syndicate::convenient_logging()?;
    syndicate::relay::stdio_service(move |t| {
        let local_ds = Dataspace::new(t, Some(AnyValue::symbol("service")));

        during!(t, local_ds, $v, |t: &mut Activation| {
            t.on_stop(enclose!((v) move |_t| { tracing::info!("- {v:?}"); Ok(()) }));
            tracing::info!("+ {v:?}");
            Ok(())
        });

        Ok(local_ds)
    }).await
}
