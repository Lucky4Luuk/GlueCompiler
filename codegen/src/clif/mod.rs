pub(crate) mod func;
pub(crate) mod stmt;
pub(crate) mod variable;

use cranelift::codegen::Context;
use cranelift::codegen::settings;
use cranelift::codegen::binemit::{NullRelocSink, NullTrapSink, NullStackMapSink};

use cranelift_object::*;
use cranelift_module::{Module, Linkage};

pub fn gen(flags: settings::Flags, target: crate::CompileTarget, hir_root: hir::Root) {
    let obj_builder = ObjectBuilder::new(target.isa, "result", cranelift_module::default_libcall_names()).expect("Failed to construct object builder!");
    let mut obj_module = ObjectModule::new(obj_builder);
    for hir_func in hir_root.functions {
        let func = func::build_func(flags.clone(), target.call_conv, &hir_func);
        let mut context = Context::for_function(func.clone());
        // let mut relocs_sink = NullRelocSink {};
        let mut traps_sink = NullTrapSink {};
        let mut stack_maps_sink = NullStackMapSink {};
        // let mut mem = Vec::new();
        // context.compile_and_emit(&*target.isa, &mut mem, &mut relocs_sink, &mut traps_sink, &mut stack_maps_sink);
        // println!("mem len: {}", mem.len());
        println!("Function name: `{}`", &hir_func.name);
        let func_id = obj_module.declare_function(&hir_func.name, Linkage::Local, &func.signature).expect("Failed to declare function!");
        obj_module.define_function(func_id, &mut context, &mut traps_sink, &mut stack_maps_sink).expect("Failed to compile function");
    }
    let obj_product = obj_module.finish();
    let mem = obj_product.emit().expect("Failed to emit to memory!");
    dbg!(mem.len());
    println!("Writing {} bytes to {}.o!", mem.len(), "result");
    std::fs::write("result.o", &mem[..]);
}
