#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub(crate) mod rum_utils {
    #![allow(unused_imports, dead_code)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub mod prelude {
    pub use crate::rum_utils::*;
    // pub use crate::rum_utils::{mlirContextCreate, mlirContextDestroy, mlirContextEqual};
}

