use std::sync::Arc;

use syndicate::enclose;
use syndicate::actor::*;

use syndicate_macros::during;

use syndicate::schemas::gatekeeper;
use syndicate::schemas::sturdy;
use syndicate::schemas::rpc as R;

use preserves_schema::Parse;
use preserves_schema::Unparse;
use preserves_schema::unparse_iovalue;

fn sturdy_step_type() -> String {
    unparse_iovalue(&sturdy::SturdyStepType).to_symbol().unwrap().into_owned()
}

pub fn handle_sturdy_bindspace(t: &mut Activation, bindspace: &Arc<Cap>) -> ActorResult {
    // Compute the Step corresponding to the Description in a Bind.
    enclose!((bindspace) during!(t, bindspace, <bind <#(sturdy::SturdyStepType.unparse()) $desc> $_target:Arc<Cap> $observer>, |t: &mut Activation| {
        t.spawn_link(None, move |t| {
            let response = match sturdy::SturdyDescriptionDetail::parse(&desc) {
                Ok(desc) => {
                    let sr = sturdy::SturdyRef::mint(desc.oid, &desc.key);
                    gatekeeper::Bound::Bound {
                        path_step: gatekeeper::PathStep {
                            step_type: sturdy_step_type(),
                            detail: sr.parameters.unparse(),
                        },
                    }
                }
                Err(_) => gatekeeper::Bound::Error(R::Error{
                    error: AnyValue::symbol("sturdyref-description-invalid")
                }),
            };
            if let Ok(gatekeeper::BindObserver::Present(o)) = gatekeeper::BindObserver::parse(&observer) {
                o.assert(t, &response);
            }
            Ok(())
        });
        Ok(())
    }));

    // Reply to Resolve requests.
    enclose!((bindspace) during!(t, bindspace, <resolve <#(sturdy::SturdyStepType.unparse()) $step> $observer:Arc<Cap>>, |t: &mut Activation| {
        match sturdy::SturdyStepDetail::parse(&step) {
            Err(_) => {
                observer.assert(t, &R::Error{ error: AnyValue::symbol("invalid") });
            }
            Ok(step) => {
                // TODO: codegen plugin to generate pattern constructors
                let queried_oid = step.0.oid.clone();
                let sturdyref = sturdy::SturdyRef { parameters: step.0 };
                enclose!((bindspace)
                         during!(t, bindspace,
                                 <bind <#(sturdy::SturdyStepType.unparse()) {
                                     oid: #(&queried_oid),
                                     key: $key
                                 }> $unattenuated_target: Arc<Cap> _>,
                                 |t: &mut Activation| {
                                     if let Some(key) = key.as_bytestring() {
                                         match sturdyref.validate_and_attenuate(key.as_ref(), &unattenuated_target) {
                                             Err(e) => {
                                                 tracing::warn!(sturdyref = ?sturdyref.unparse(),
                                                                "sturdyref failed validation: {}", e);
                                                 observer.assert(t, &R::Result::Error(R::Error {
                                                     error: AnyValue::symbol("sturdyref-failed-validation"),
                                                 }));
                                             },
                                             Ok(target) => {
                                                 tracing::trace!(sturdyref = ?sturdyref.unparse(),
                                                                 ?target,
                                                                 "sturdyref resolved");
                                                 observer.assert(t, &R::Result::Ok(R::Ok {
                                                     value: target.unparse(),
                                                 }));
                                             }
                                         }
                                     }
                                     Ok(())
                                 }));
            }
        }
        Ok(())
    }));

    Ok(())
}
