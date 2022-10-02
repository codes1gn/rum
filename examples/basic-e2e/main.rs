use rum::prelude::*;
use std::ffi::CString;

fn main() {
    unsafe {
        // create context
        let ctx = mlirContextCreate();
        assert!(mlirContextEqual(ctx, ctx));

        // create dialect registry
        let registry = mlirDialectRegistryCreate();

        // dialect registry => bind => context
        mlirContextAppendDialectRegistry(ctx, registry);

        // load func
        // let toy_name = CString::new("toy").unwrap();
        // let toy_name_sr = mlirStringRefCreateFromCString(toy_name.as_ptr());
        // let toy_dialect = mlirContextGetOrLoadDialect(ctx, toy_name_sr);
        let reged_dialect_cnt = mlirContextGetNumRegisteredDialects(ctx);
        let loaded_dialect_cnt = mlirContextGetNumLoadedDialects(ctx);
        println!("{:?}", reged_dialect_cnt);
        println!("{:?}", loaded_dialect_cnt);

        // destroy dialect registry
        mlirDialectRegistryDestroy(registry);

        // destroy context
        mlirContextDestroy(ctx);
    }
    ()
}
