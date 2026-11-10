#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));


/// Safe Rust value types and state snapshot accessors.
pub mod state;

pub use state::{
    Accelerometer, ButtonState, NunchukButtons, NunchukState, WiimoteButtons,
    WiimoteState, get_state,
};
