use crate::code_block::CodeBlock;

#[derive(Debug)]
pub struct FuncArgs {
    
}

#[derive(Debug)]
pub enum FuncReturn {

}

#[derive(Debug)]
pub struct Func {
    pub name: String,
    pub args: Option<FuncArgs>,
    pub return_args: Option<FuncReturn>,
    pub code_block: CodeBlock,
}

impl Func {
    pub fn lower(ast: ast::Func) -> Option<Self> {
        let name = ast.name()?.text().to_string();
        let code_block = CodeBlock::lower(ast.code_block()?)?;

        Some(Self {
            name: name,
            code_block: code_block,
        })
    }
}
