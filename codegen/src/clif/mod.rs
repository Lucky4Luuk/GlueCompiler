pub(crate) mod func;
pub(crate) mod stmt;
pub(crate) mod variable;

use cranelift::codegen::Context;
use cranelift::codegen::settings;
use cranelift::codegen::binemit::{NullRelocSink, NullTrapSink, NullStackMapSink};

pub fn gen(flags: settings::Flags, target: crate::CompileTarget, hir_root: hir::Root) {
    for hir_func in hir_root.functions {
        let func = func::build_func(flags.clone(), target.call_conv, &hir_func);
        let mut context = Context::for_function(func);
        let mut relocs_sink = NullRelocSink {};
        let mut traps_sink = NullTrapSink {};
        let mut stack_maps_sink = NullStackMapSink {};
        let mut mem = Vec::new();
        context.compile_and_emit(&*target.isa, &mut mem, &mut relocs_sink, &mut traps_sink, &mut stack_maps_sink);
        println!("mem len: {}", mem.len());
    }
}
