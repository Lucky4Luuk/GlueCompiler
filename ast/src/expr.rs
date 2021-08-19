use syntax::{SyntaxNode, SyntaxToken, SyntaxElement, SyntaxKind};
use common::LiteralType;

#[derive(Debug)]
pub enum Expr {
    BinaryExpr(BinaryExpr),
    Literal(Literal),
    ParenExpr(ParenExpr),
    UnaryExpr(UnaryExpr),
    VariableRef(VariableRef),
    FunctionCall(FunctionCall),
}

impl Expr {
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        let result = match node.kind() {
            SyntaxKind::InfixExpr => Self::BinaryExpr(BinaryExpr(node)),
            SyntaxKind::Literal => Self::Literal(Literal(node)),
            SyntaxKind::ParenExpr => Self::ParenExpr(ParenExpr(node)),
            SyntaxKind::PrefixExpr => Self::UnaryExpr(UnaryExpr(node)),
            SyntaxKind::VariableRef => Self::VariableRef(VariableRef(node)),
            SyntaxKind::FunctionCall => Self::FunctionCall(FunctionCall(node)),
            _ => return None,
        };

        Some(result)
    }
}

#[derive(Debug)]
pub struct BinaryExpr(SyntaxNode);

impl BinaryExpr {
    pub fn lhs(&self) -> Option<Expr> {
        self.0.children().find_map(Expr::cast)
    }

    pub fn rhs(&self) -> Option<Expr> {
        self.0.children().filter_map(Expr::cast).nth(1)
    }

    pub fn op(&self) -> Option<SyntaxToken> {
        self.0
            .children_with_tokens()
            .filter_map(SyntaxElement::into_token)
            .find(|token| {
                matches!(
                    token.kind(),
                    SyntaxKind::Plus | SyntaxKind::Minus | SyntaxKind::Asterix | SyntaxKind::Slash,
                )
            })
    }
}

#[derive(Debug)]
pub struct Literal(SyntaxNode);

impl Literal {
    pub fn parse(&self) -> LiteralType {
        let first_token = self.0.first_token().unwrap();
        let as_text = first_token.text().replace("_", "");
        if as_text.contains("u64") {
            LiteralType::U64(as_text.to_string().replace("u64", "").parse::<u64>().expect("Literal is not u64!"))
        } else if as_text.contains("u32") {
            LiteralType::U32(as_text.to_string().replace("u32", "").parse::<u32>().expect("Literal is not u32!"))
        } else if as_text.contains("u16") {
            LiteralType::U16(as_text.to_string().replace("u16", "").parse::<u16>().expect("Literal is not u16!"))
        } else if as_text.contains("u8") {
            LiteralType::U8(as_text.to_string().replace("u8", "").parse::<u8>().expect("Literal is not u8!"))
        } else if as_text.contains("i64") {
            LiteralType::I64(as_text.to_string().replace("i64", "").parse::<i64>().expect("Literal is not i64!"))
        } else if as_text.contains("i32") {
            LiteralType::I32(as_text.to_string().replace("i32", "").parse::<i32>().expect("Literal is not i32!"))
        } else if as_text.contains("i16") {
            LiteralType::I16(as_text.to_string().replace("i16", "").parse::<i16>().expect("Literal is not i16!"))
        } else if as_text.contains("i8") {
            LiteralType::I8(as_text.to_string().replace("i8", "").parse::<i8>().expect("Literal is not i8!"))
        } else if as_text.contains("f64") {
            LiteralType::F64(as_text.to_string().replace("f64", "").parse::<f64>().expect("Literal is not f64!"))
        } else if as_text.contains("f32") {
            LiteralType::F32(as_text.to_string().replace("f32", "").parse::<f32>().expect("Literal is not f32!"))
        } else {
            LiteralType::Missing
        }
    }
}

#[derive(Debug)]
pub struct ParenExpr(SyntaxNode);

impl ParenExpr {
    pub fn expr(&self) -> Option<Expr> {
        self.0.children().find_map(Expr::cast)
    }
}

#[derive(Debug)]
pub struct UnaryExpr(SyntaxNode);

impl UnaryExpr {
    pub fn expr(&self) -> Option<Expr> {
        self.0.children().find_map(Expr::cast)
    }

    pub fn op(&self) -> Option<SyntaxToken> {
        self.0
            .children_with_tokens()
            .filter_map(SyntaxElement::into_token)
            .find(|token| token.kind() == SyntaxKind::Minus)
    }
}

#[derive(Debug)]
pub struct VariableRef(SyntaxNode);

impl VariableRef {
    pub fn name(&self) -> String {
        self.0.first_token().unwrap().text().to_string()
    }
}

#[derive(Debug)]
pub struct FunctionCall(SyntaxNode);

impl FunctionCall {
    pub fn name(&self) -> String {
        self.0.first_token().unwrap().text().to_string()
    }
}
