pub use cranelift::codegen::isa::CallConv; //Re-export for use in the frontend

mod clif;

#[derive(Debug)]
pub struct CompileTarget {
    pub call_conv: CallConv, //TODO: Can also be chosen with CallConv::triple_default (see https://docs.rs/target-lexicon/0.12.1/target_lexicon/struct.Triple.html)
}

impl CompileTarget {
    pub fn from_triple() -> Self {
        todo!();
    }
}

// impl TargetIsa for CompileTarget {}

pub fn compile(target: CompileTarget, hir: hir::Root) {
    let clif_code = clif::gen(target, hir);
}
