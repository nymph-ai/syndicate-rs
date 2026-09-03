pub mod counter;
pub mod dependencies;
pub mod http;
pub mod lifecycle;
pub mod protocol;
pub mod resolution;
pub mod script;
pub mod services;

pub mod schemas {
    include!(concat!(env!("OUT_DIR"), "/src/schemas/mod.rs"));
}
