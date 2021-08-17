use crate::code_block::CodeBlock;

#[derive(Debug)]
pub struct FuncArgs {
    pub args: Vec<(String, String)>
}

impl FuncArgs {
    pub fn lower(ast: ast::FuncArgs) -> Option<Self> {
        let args = ast.args();

        if args.len() % 2 != 0 { return None; }

        let mut mapped_args = Vec::new();
        for i in (0..args.len()).step_by(2) {
            let var_type = args[i].text().to_string();
            let var_name = args[i+1].text().to_string();
            mapped_args.push((var_type, var_name));
        }

        Some(Self {
            args: mapped_args
        })
    }
}

#[derive(Debug)]
pub enum FuncReturnArgs {
    SingleReturn(String),
    MultiReturn(Vec<(String, String)>)
}

impl FuncReturnArgs {
    pub fn lower(ast: ast::FuncReturnArgs) -> Option<Self> {
        let args = ast.args();
        if args.len() < 1 {
            None
        } else if args.len() < 2 {
            Some(Self::SingleReturn(args[0].text().to_string()))
        } else {
            if args.len() % 2 != 0 { return None; }
            let mut mapped_args = Vec::new();
            for i in (0..args.len()).step_by(2) {
                let var_type = args[i].text().to_string();
                let var_name = args[i+1].text().to_string();
                mapped_args.push((var_type, var_name));
            }
            Some(Self::MultiReturn(mapped_args))
        }
    }
}

#[derive(Debug)]
pub struct Func {
    pub name: String,
    pub args: Option<FuncArgs>,
    pub ret_args: Option<FuncReturnArgs>,
    pub code_block: CodeBlock,
}

impl Func {
    pub fn lower(ast: ast::Func) -> Option<Self> {
        let name = ast.name()?.text().to_string();
        let args = ast.args();
        let args = if args.is_some() {
            let unwrapped_args = args?;
            if unwrapped_args.args().len() > 0 {
                FuncArgs::lower(unwrapped_args)
            } else {
                None
            }
        } else {
            None
        };
        let ret_args = ast.ret_args();
        let ret_args = if ret_args.is_some() { FuncReturnArgs::lower(ret_args?) } else { None };
        let code_block = CodeBlock::lower(ast.code_block()?);

        Some(Self {
            name: name,
            args: args,
            ret_args: ret_args,
            code_block: code_block,
        })
    }
}
