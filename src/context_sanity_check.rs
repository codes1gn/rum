use crate::rum_utils::{mlirContextCreate, mlirContextDestroy, mlirContextEqual};

#[cfg(test)]
mod mlir_tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_context() {
        unsafe {
            let ctx = mlirContextCreate();
            assert!(mlirContextEqual(ctx, ctx));
            mlirContextDestroy(ctx);
        }
    }
}
