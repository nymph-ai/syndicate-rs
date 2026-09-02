use std::sync::Arc;

use syndicate::actor::*;
use syndicate_macros::*;

// Not a #[test], compilation only
#[allow(unused)]
fn pattern_arc_cap(t: &mut Activation, ds: Cap) -> ActorResult {
    during!(t, ds, $v: Arc<Cap>, |t: &mut Activation| {
        Ok(())
    });
    Ok(())
}

// Not a #[test], compilation only
#[allow(unused)]
fn pattern_explicit_anyvalue(t: &mut Activation, ds: Cap) -> ActorResult {
    during!(t, ds, $v: AnyValue, |t: &mut Activation| {
        Ok(())
    });
    Ok(())
}

// Not a #[test], compilation only
#[allow(unused)]
fn pattern_implicit_anyvalue(t: &mut Activation, ds: Cap) -> ActorResult {
    during!(t, ds, $v, |t: &mut Activation| {
        Ok(())
    });
    Ok(())
}
