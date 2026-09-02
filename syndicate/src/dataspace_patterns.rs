use std::sync::Arc;

use preserves::*;
use preserves_schema::Parse;

use crate::actor::AnyValue;
use crate::actor::Cap;
use crate::schemas::dataspace_patterns::*;

/// Trait implemented by types able to serve in pattern context (in macros provided by `syndicate-macros`).
pub trait AsDataspacePattern {
    fn wildcard_dataspace_pattern() -> Pattern;
}

macro_rules! discard_dataspace_pattern {
    ($t:ty) => {
        impl AsDataspacePattern for $t {
            fn wildcard_dataspace_pattern() -> Pattern {
                Pattern::Discard
            }
        }
    };
}

discard_dataspace_pattern!(AnyValue);
discard_dataspace_pattern!(Double);
discard_dataspace_pattern!(f64);
discard_dataspace_pattern!(SignedInteger);
discard_dataspace_pattern!(u8);
discard_dataspace_pattern!(i8);
discard_dataspace_pattern!(u16);
discard_dataspace_pattern!(i16);
discard_dataspace_pattern!(u32);
discard_dataspace_pattern!(i32);
discard_dataspace_pattern!(u64);
discard_dataspace_pattern!(i64);
discard_dataspace_pattern!(u128);
discard_dataspace_pattern!(i128);
discard_dataspace_pattern!(usize);
discard_dataspace_pattern!(isize);
discard_dataspace_pattern!(String);
discard_dataspace_pattern!(Bytes);
discard_dataspace_pattern!(Symbol);

impl<T: Domain + AsDataspacePattern> AsDataspacePattern for Embedded<T> {
    fn wildcard_dataspace_pattern() -> Pattern {
        Pattern::Discard
    }
}

impl<T: Parse<Arc<Cap>>> AsDataspacePattern for Vec<T> {
    fn wildcard_dataspace_pattern() -> Pattern {
        Pattern::Group {
            type_: GroupType::Arr,
            entries: Map::new(),
        }
    }
}

impl<K: Parse<Arc<Cap>>, V: Parse<Arc<Cap>>> AsDataspacePattern for Map<K, V> {
    fn wildcard_dataspace_pattern() -> Pattern {
        Pattern::Group {
            type_: GroupType::Dict,
            entries: Map::new(),
        }
    }
}

impl<T: AsDataspacePattern> AsDataspacePattern for std::sync::Arc<T> {
    fn wildcard_dataspace_pattern() -> Pattern {
        T::wildcard_dataspace_pattern()
    }
}

impl<T: AsDataspacePattern> AsDataspacePattern for std::rc::Rc<T> {
    fn wildcard_dataspace_pattern() -> Pattern {
        T::wildcard_dataspace_pattern()
    }
}

impl<T: AsDataspacePattern> AsDataspacePattern for Box<T> {
    fn wildcard_dataspace_pattern() -> Pattern {
        T::wildcard_dataspace_pattern()
    }
}
