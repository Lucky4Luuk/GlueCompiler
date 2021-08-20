/// A list of supported tools to compile the resulting C with
pub enum Tool {
    GCC,
    // Clang,
}

pub(crate) mod grammar;
mod gcc;

pub fn compile(triple: String, tool: Tool, hir: hir::Root) {
    match tool {
        GCC => gcc::compile(triple, hir),
        _ => unimplemented!(),
    }
}
