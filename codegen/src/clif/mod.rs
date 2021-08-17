pub(crate) mod func;
pub(crate) mod stmt;
pub(crate) mod variable;

pub fn gen(target: crate::CompileTarget, hir_root: hir::Root) {
    for hir_func in hir_root.functions {
        func::build_func(target.call_conv, &hir_func);
    }
}
