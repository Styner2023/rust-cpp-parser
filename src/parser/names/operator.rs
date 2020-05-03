// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use crate::lexer::preprocessor::context::PreprocContext;
use crate::lexer::{Lexer, LocToken, Token};
use crate::parser::declarations::{decl::DeclSpecifierParser, pointer::PointerDeclaratorParser};
use crate::parser::expressions;
use crate::parser::types::r#type::Type;

#[derive(Clone, Debug, PartialEq)]
pub enum Operator {
    Op(expressions::Operator),
    UD(String),
    Conv(Type),
}

impl Operator {
    pub fn is_conv(&self) -> bool {
        match self {
            Operator::Op(_) => false,
            _ => true,
        }
    }
}

pub(crate) struct OperatorParser<'a, 'b, PC: PreprocContext> {
    lexer: &'b mut Lexer<'a, PC>,
}

impl<'a, 'b, PC: PreprocContext> OperatorParser<'a, 'b, PC> {
    pub(crate) fn new(lexer: &'b mut Lexer<'a, PC>) -> Self {
        Self { lexer }
    }

    pub(crate) fn parse(self, tok: Option<LocToken>) -> (Option<LocToken>, Option<Operator>) {
        let tok = tok.unwrap_or_else(|| self.lexer.next_useful());
        if tok.tok != Token::Operator {
            return (Some(tok), None);
        }

        let tok = self.lexer.next_useful();
        match tok.tok {
            Token::LiteralString(_) => {
                let tok = self.lexer.next_useful();
                if let Token::Identifier(id) = tok.tok {
                    (None, Some(Operator::UD(id)))
                } else {
                    unreachable!("Invalid token in operator name: {:?}", tok);
                }
            }
            Token::LiteralStringUD(s_ud) => {
                let (_, ud) = *s_ud;
                (None, Some(Operator::UD(ud)))
            }
            Token::New => {
                let tok = self.lexer.next_useful();
                if tok.tok == Token::LeftBrack {
                    let tok = self.lexer.next_useful();
                    if tok.tok == Token::RightBrack {
                        (None, Some(Operator::Op(expressions::Operator::NewArray)))
                    } else {
                        unreachable!("Invalid token in operator name: {:?}", tok);
                    }
                } else {
                    (Some(tok), Some(Operator::Op(expressions::Operator::New)))
                }
            }
            Token::Delete => {
                let tok = self.lexer.next_useful();
                if tok.tok == Token::LeftBrack {
                    let tok = self.lexer.next_useful();
                    if tok.tok == Token::RightBrack {
                        (None, Some(Operator::Op(expressions::Operator::DeleteArray)))
                    } else {
                        unreachable!("Invalid token in operator name: {:?}", tok);
                    }
                } else {
                    (Some(tok), Some(Operator::Op(expressions::Operator::Delete)))
                }
            }
            Token::CoAwait => (None, Some(Operator::Op(expressions::Operator::CoAwait))),
            Token::LeftParen => {
                let tok = self.lexer.next_useful();
                if tok.tok == Token::RightParen {
                    (None, Some(Operator::Op(expressions::Operator::Call)))
                } else {
                    unreachable!("Invalid token in operator name: {:?}", tok);
                }
            }
            Token::LeftBrack => {
                let tok = self.lexer.next_useful();
                if tok.tok == Token::RightBrack {
                    (None, Some(Operator::Op(expressions::Operator::Subscript)))
                } else {
                    unreachable!("Invalid token in operator name: {:?}", tok);
                }
            }
            Token::Arrow => (None, Some(Operator::Op(expressions::Operator::Arrow))),
            Token::ArrowStar => (
                None,
                Some(Operator::Op(expressions::Operator::ArrowIndirection)),
            ),
            Token::Tilde => (None, Some(Operator::Op(expressions::Operator::BitNeg))),
            Token::Not => (None, Some(Operator::Op(expressions::Operator::Not))),
            Token::Plus => (None, Some(Operator::Op(expressions::Operator::Plus))),
            Token::Minus => (None, Some(Operator::Op(expressions::Operator::Minus))),
            Token::Star => (None, Some(Operator::Op(expressions::Operator::Indirection))),
            Token::Divide => (None, Some(Operator::Op(expressions::Operator::Div))),
            Token::Modulo => (None, Some(Operator::Op(expressions::Operator::Mod))),
            Token::Xor => (None, Some(Operator::Op(expressions::Operator::BitXor))),
            Token::And => (None, Some(Operator::Op(expressions::Operator::AddressOf))),
            Token::Or => (None, Some(Operator::Op(expressions::Operator::BitOr))),
            Token::Equal => (None, Some(Operator::Op(expressions::Operator::Assign))),
            Token::PlusEqual => (None, Some(Operator::Op(expressions::Operator::AddAssign))),
            Token::MinusEqual => (None, Some(Operator::Op(expressions::Operator::SubAssign))),
            Token::StarEqual => (None, Some(Operator::Op(expressions::Operator::MulAssign))),
            Token::DivideEqual => (None, Some(Operator::Op(expressions::Operator::DivAssign))),
            Token::ModuloEqual => (None, Some(Operator::Op(expressions::Operator::ModAssign))),
            Token::XorEqual => (None, Some(Operator::Op(expressions::Operator::XorAssign))),
            Token::AndEqual => (None, Some(Operator::Op(expressions::Operator::AndAssign))),
            Token::OrEqual => (None, Some(Operator::Op(expressions::Operator::OrAssign))),
            Token::EqualEqual => (None, Some(Operator::Op(expressions::Operator::Eq))),
            Token::NotEqual => (None, Some(Operator::Op(expressions::Operator::Neq))),
            Token::Lower => (None, Some(Operator::Op(expressions::Operator::Lt))),
            Token::Greater => (None, Some(Operator::Op(expressions::Operator::Gt))),
            Token::LowerEqual => (None, Some(Operator::Op(expressions::Operator::Leq))),
            Token::GreaterEqual => (None, Some(Operator::Op(expressions::Operator::Geq))),
            Token::LowerEqualGreater => {
                (None, Some(Operator::Op(expressions::Operator::ThreeWayComp)))
            }
            Token::AndAnd => (None, Some(Operator::Op(expressions::Operator::And))),
            Token::OrOr => (None, Some(Operator::Op(expressions::Operator::Or))),
            Token::LeftShift => (None, Some(Operator::Op(expressions::Operator::LShift))),
            Token::RightShift => (None, Some(Operator::Op(expressions::Operator::RShift))),
            Token::LeftShiftEqual => (None, Some(Operator::Op(expressions::Operator::LShiftAssign))),
            Token::RightShiftEqual => {
                (None, Some(Operator::Op(expressions::Operator::RShiftAssign)))
            }
            Token::PlusPlus => (None, Some(Operator::Op(expressions::Operator::PreInc))),
            Token::MinusMinus => (None, Some(Operator::Op(expressions::Operator::PreDec))),
            Token::Comma => (None, Some(Operator::Op(expressions::Operator::Comma))),
            _ => {
                let ctp = ConversionTypeParser::new(self.lexer);
                let (tok, typ) = ctp.parse(Some(tok));

                if let Some(typ) = typ {
                    (tok, Some(Operator::Conv(typ)))
                } else {
                    unreachable!("Invalid token in operator name: {:?}", tok);
                }

                // TODO: add operator literal: http://eel.is/c++draft/over.literal#nt:literal-operator-id
            }
        }
    }
}

pub struct ConversionTypeParser<'a, 'b, PC: PreprocContext> {
    lexer: &'b mut Lexer<'a, PC>,
}

impl<'a, 'b, PC: PreprocContext> ConversionTypeParser<'a, 'b, PC> {
    pub(crate) fn new(lexer: &'b mut Lexer<'a, PC>) -> Self {
        Self { lexer }
    }

    pub(crate) fn parse(self, tok: Option<LocToken>) -> (Option<LocToken>, Option<Type>) {
        let dsp = DeclSpecifierParser::new(self.lexer);
        let (tok, (_, typ, _)) = dsp.parse(tok, None);

        let mut typ = if let Some(typ) = typ {
            typ
        } else {
            return (tok, None);
        };

        // Pointer: *, &, &&
        let pdp = PointerDeclaratorParser::new(self.lexer);
        let (tok, ptrs) = pdp.parse(tok, None);
        typ.pointers = ptrs;

        (tok, Some(typ))
    }
}
