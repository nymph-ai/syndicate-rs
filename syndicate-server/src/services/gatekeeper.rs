use std::sync::Arc;

use syndicate::actor::*;
use syndicate::enclose;
use syndicate::schemas::gatekeeper;

use syndicate_macros::during;
use syndicate_macros::template;

use preserves_schema::Unparse;

use crate::lifecycle;
use crate::resolution::sturdy;
use crate::resolution::noise;
use crate::schemas::internal_services::Gatekeeper;

pub fn on_demand(t: &mut Activation, ds: Arc<Cap>) {
    t.spawn(Some(AnyValue::symbol("gatekeeper_listener")), move |t| {
        Ok(during!(t, ds, <run-service $spec: Gatekeeper::<Arc<Cap>>>, |t: &mut Activation| {
            let spec_v = spec.unparse();
            t.spawn_link(Some(template!("<gatekeeper =spec_v>")), enclose!((ds) |t| run(t, ds, spec)));
            Ok(())
        }))
    });
}

fn run(t: &mut Activation, ds: Arc<Cap>, spec: Gatekeeper<Arc<Cap>>) -> ActorResult {
    let gk = create_gatekeeper(t, &spec.bindspace)?;
    ds.assert(t, &syndicate::schemas::service::ServiceObject {
        service_name: spec.unparse(),
        object: AnyValue::embedded(gk),
    });
    ds.assert(t, &lifecycle::started(&spec));
    ds.assert(t, &lifecycle::ready(&spec));
    Ok(())
}

pub fn create_gatekeeper(t: &mut Activation, bindspace: &Arc<Cap>) -> Result<Arc<Cap>, ActorError> {
    // For convenience, we support resolution of sturdy and noise steps in every gatekeeper. We
    // don't make this support optional, because if you want a completely blank gatekeeper,
    // that's easily manually constructible from a blank entity (respond to Resolve requests as
    // you like). Adding support for optionality of sturdy/noise would be too much complexity
    // in the Rust code.
    sturdy::handle_sturdy_bindspace(t, bindspace)?;
    noise::handle_noise_bindspace(t, bindspace)?;
    Ok(Cap::guard(t.create(
        syndicate::entity(Arc::clone(bindspace))
            .on_asserted_facet(move |bindspace, t, a: gatekeeper::Resolve| {
                bindspace.assert(t, &a);
                Ok(())
            }))))
}
