pub(crate) mod func;
pub(crate) mod stmt;
pub(crate) mod variable;

use cranelift::codegen::Context;

pub fn gen(target: crate::CompileTarget, hir_root: hir::Root) {
    for hir_func in hir_root.functions {
        let func = func::build_func(target.call_conv, &hir_func);
        let context = Context::for_function(func);
    }
}
