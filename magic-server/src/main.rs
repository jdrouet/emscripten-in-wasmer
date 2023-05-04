use std::path::PathBuf;
use wasmer::{FunctionEnv, Instance, Module, Store, TypedFunction};
use wasmer_compiler_singlepass::Singlepass;
use wasmer_emscripten::{generate_emscripten_env, EmEnv, EmscriptenGlobals};

fn main() -> anyhow::Result<()> {
    println!("load wasm file");
    let arg = std::env::args()
        .enumerate()
        .find_map(|(idx, item)| if idx == 1 { Some(item) } else { None })
        .unwrap_or_else(|| String::from("../magic-lib/pkg/magic_lib_bg.wasm"));
    let arg = PathBuf::from(arg);

    let wasm = std::fs::read(arg)?;

    println!("create env");
    let em_env = EmEnv::new();

    // Create a Store.
    println!("create store");
    let compiler = Singlepass::new();
    let mut store = Store::new(compiler);

    // We then use our store and Wasm bytes to compile a `Module`.
    // A `Module` is a compiled WebAssembly module that isn't ready to execute yet.
    println!("create module");
    let mut module = Module::new(&store, &wasm).unwrap();
    module.set_name("magic system");

    // create an EmEnv with default global
    println!("create function env");
    let env = FunctionEnv::new(&mut store, em_env);

    println!("create emscripten globals");
    let mut emscripten_globals =
        EmscriptenGlobals::new(&mut store, &env, &module).map_err(|e| anyhow::anyhow!("{}", e))?;
    env.as_mut(&mut store)
        .set_data(&emscripten_globals.data, Default::default());

    println!("generate emscripten env");
    let import_object = generate_emscripten_env(&mut store, &env, &mut emscripten_globals);

    // We then use the `Module` and the import object to create an `Instance`.
    //
    // An `Instance` is a compiled WebAssembly module that has been set up
    // and is ready to execute.
    println!("create instance");
    let instance = Instance::new(&mut store, &module, &import_object)?;

    // We get the `TypedFunction` with no parameters and no results from the instance.
    //
    // Recall that the Wasm module exported a function named "run", this is getting
    // that exported function from the `Instance`.
    println!("get typed function");
    let _magic_compute_func: TypedFunction<(), ()> = instance
        .exports
        .get_typed_function(&mut store, "magic_compute")?;

    // Finally, we call our exported Wasm function which will call our "magic_compute"
    // function and return.
    // run_func.call(&mut store)?;

    Ok(())
}
