use syntax::{SyntaxNode, SyntaxToken, SyntaxElement, SyntaxKind};

use crate::code_block::CodeBlock;

#[derive(Debug)]
pub struct FuncArgs(SyntaxNode);

impl FuncArgs {
    //TODO: This function is not very neat.
    fn tokens_of_type(&self, kind: SyntaxKind) -> Vec<SyntaxToken> {
        let mut children: Vec<SyntaxToken> = Vec::new();
        for element in self.0.children_with_tokens() {
            if element.kind() == kind {
                children.push(element.as_token().expect("Unreachable!").clone());
            }
        }
        children
    }

    pub fn cast(node: SyntaxNode) -> Option<Self> {
        if node.kind() == SyntaxKind::FunctionDeclarationArgs {
            Some(Self(node))
        } else {
            None
        }
    }

    ///Has to return an even length list of identifiers, because
    ///all arguments have to be named.
    pub fn args(&self) -> Vec<SyntaxToken> {
        let idents = self.tokens_of_type(SyntaxKind::Identifier);
        if idents.len() < 2 {
            return Vec::new();
        }
        idents
    }
}

#[derive(Debug)]
pub struct FuncReturnArgs(SyntaxNode);

impl FuncReturnArgs {
    fn tokens_of_type(&self, kind: SyntaxKind) -> Vec<SyntaxToken> {
        let mut children: Vec<SyntaxToken> = Vec::new();
        for element in self.0.children_with_tokens() {
            if element.kind() == kind {
                children.push(element.as_token().expect("Unreachable!").clone());
            }
        }
        children
    }

    pub fn cast(node: SyntaxNode) -> Option<Self> {
        if node.kind() == SyntaxKind::FunctionDeclarationReturnArgs {
            Some(Self(node))
        } else {
            None
        }
    }

    ///If you only receive 1 identifier, it means its a single, unnamed return.
    ///The other correct option is getting an even length list of identifiers
    pub fn args(&self) -> Vec<SyntaxToken> {
        let idents = self.tokens_of_type(SyntaxKind::Identifier);
        if idents.len() < 1 {
            return Vec::new();
        }
        idents
    }
}

#[derive(Debug)]
pub struct Func(SyntaxNode);

impl Func {
    //TODO: This function is not very neat.
    fn tokens_of_type(&self, kind: SyntaxKind) -> Vec<SyntaxToken> {
        let mut children: Vec<SyntaxToken> = Vec::new();
        for element in self.0.children_with_tokens() {
            if element.kind() == kind {
                children.push(element.as_token().expect("Unreachable!").clone());
            }
        }
        children
    }

    pub fn cast(node: SyntaxNode) -> Option<Self> {
        if node.kind() == SyntaxKind::FunctionDeclaration {
            Some(Self(node))
        } else {
            None
        }
    }

    pub fn name(&self) -> Option<SyntaxToken> {
        let idents = self.tokens_of_type(SyntaxKind::Identifier);
        let ident_count = idents.len();
        if ident_count > 1 {
            Some(idents[1].clone())
        } else if ident_count > 0 {
            Some(idents[0].clone())
        } else {
            None
        }
    }

    pub fn code_block(&self) -> Option<CodeBlock> {
        self.0.children().filter_map(CodeBlock::cast).next()
    }

    pub fn args(&self) -> Option<FuncArgs> {
        self.0.children().filter_map(FuncArgs::cast).next()
    }

    pub fn ret_args(&self) -> Option<FuncReturnArgs> {
        self.0.children().filter_map(FuncReturnArgs::cast).next()
    }
}
