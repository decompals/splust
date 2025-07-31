use wasmtime_wasi::{
    p2::{IoView, WasiCtx, WasiView},
    ResourceTable,
};

pub mod yaml;
struct MyState {
    ctx: WasiCtx,
    table: ResourceTable,
}
impl IoView for MyState {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}
impl WasiView for MyState {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
}
#[cfg(test)]
mod tests {
    use wasmtime::component::{bindgen, Component, Linker, ResourceTable};
    use wasmtime::{Config, Engine, Store};
    use wasmtime_wasi::p2::WasiCtxBuilder;

    use crate::*;

    #[test]
    fn test_segments() {
        let mut config = Config::new();
        config.wasm_component_model(true);
        config.debug_info(true);

        let engine = Engine::new(&config).unwrap();

        bindgen!("segment" in "../splat-segment-api/wit/world.wit");

        let mut linker = Linker::new(&engine);

        // Add all the WASI extensions to the linker
        wasmtime_wasi::p2::add_to_linker_sync(&mut linker).unwrap();

        //NewWorld::add_to_linker(&mut linker, |state: &mut MyState| state)?;

        // ... configure `builder` more to add env vars, args, etc ...
        let mut builder = WasiCtxBuilder::new();
        builder.inherit_stdio();
        let mut store = Store::new(
            &engine,
            MyState {
                ctx: builder.build(),
                table: ResourceTable::new(),
            },
        );
        let component =
            Component::from_file(&engine, "../target/wasm32-wasip1/debug/splat_segment.wasm")
                .unwrap();
        let bindings = Segment::instantiate(&mut store, &component, &linker).unwrap();

        let greeting = bindings.call_testfunc(&mut store, "Ben").unwrap();
        println!("{greeting}");
    }
}
