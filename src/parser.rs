use anyhow::anyhow;
use thiserror::Error;

use crate::ast::{Abstraction, Application, Identifier, Node};
use crate::lexer::Lexer;
use crate::token::{Token, TokenError, TokenKind};

#[derive(Debug, Error, PartialEq)]
pub enum ParserError {
  #[error("Unexpected token: {0:?}")]
  UnexpectedToken(TokenError),

  #[error("Unexpected end of input")]
  UnexpectedEndOfInput,
}

pub struct Parser<'inp> {
  lexer: Lexer<'inp>,
  current_token: Option<Token<'inp>>,
}

impl<'inp> Parser<'inp> {
  pub fn new(input: &'inp str) -> Self {
    let mut lexer = Lexer::new(input);
    let current_token = lexer.next_token();
    Parser {
      lexer,
      current_token,
    }
  }

  /// Parse a term, which is either a lambda, or an application
  ///
  /// term ::= application
  ///        | LAMBDA LCID DOT term
  pub fn parse_term(&mut self) -> Result<Node<'inp>, anyhow::Error> {
    match self.current_kind() {
      Some(TokenKind::Lambda) => self.parse_abstraction(),
      _ => self.parse_application(),
    }
  }

  fn parse_abstraction(&mut self) -> Result<Node<'inp>, anyhow::Error> {
    self.advance();
    let param = match &self.current_token {
      Some(Token {
        kind: TokenKind::LowercaseId,
        text,
      }) => *text,
      Some(..) => {
        return Err(anyhow!(ParserError::UnexpectedToken(
          self
            .current_token
            .clone()
            .map(Into::into)
            .expect("not an eof error")
        )))
      }
      None => return Err(anyhow!(ParserError::UnexpectedEndOfInput)),
    };
    self.advance();
    self.expect(TokenKind::Dot)?;
    let body = self.parse_term()?;
    Ok(Node::Abstraction(Box::new(Abstraction {
      param: param,
      body,
    })))
  }

  /// Parse an application, which is an application applied left-associatively to itself
  ///
  /// Originally represented as:
  /// application ::= application atom
  ///
  /// The left recursion was removed with:
  /// application  ::= atom application'
  /// application' ::= atom application'
  ///                | ε
  fn parse_application(&mut self) -> Result<Node<'inp>, anyhow::Error> {
    let mut lhs = self.parse_atom()?;
    while matches!(
      self.current_kind(),
      Some(TokenKind::LowercaseId | TokenKind::LeftParen)
    ) {
      let rhs = self.parse_atom()?;
      lhs = Node::Application(Box::new(Application { lhs, rhs }));
    }
    Ok(lhs)
  }

  /// Parse an atom, which is any term between brackets, or a lowercase ID
  ///
  /// atom ::= LPAREN term RPAREN
  ///        | LCID
  fn parse_atom(&mut self) -> Result<Node<'inp>, anyhow::Error> {
    match self.current_kind() {
      Some(TokenKind::LeftParen) => self.parse_parenthesized(),
      Some(TokenKind::LowercaseId) => self.parse_identifier(),
      Some(..) => Err(anyhow!(ParserError::UnexpectedToken(
        self
          .current_token
          .clone()
          .map(Into::into)
          .expect("not an eof error")
      ))),
      None => Err(anyhow!(ParserError::UnexpectedEndOfInput)),
    }
  }

  fn parse_parenthesized(&mut self) -> Result<Node<'inp>, anyhow::Error> {
    self.advance();
    let term = self.parse_term()?;
    self.expect(TokenKind::RightParen)?;
    Ok(term)
  }

  fn parse_identifier(&mut self) -> Result<Node<'inp>, anyhow::Error> {
    let id = match &self.current_token {
      Some(Token { text, .. }) => *text,
      None => return Err(anyhow!(ParserError::UnexpectedEndOfInput)),
    };
    self.advance();
    Ok(Node::Identifier(Identifier { name: id }))
  }

  fn advance(&mut self) {
    self.current_token = self.lexer.next_token();
  }

  fn expect(&mut self, kind: TokenKind) -> Result<(), anyhow::Error> {
    match self.current_kind() {
      Some(k) if k == kind => {
        self.advance();
        Ok(())
      }
      Some(..) => Err(anyhow!(ParserError::UnexpectedToken(
        self
          .current_token
          .clone()
          .map(Into::into)
          .expect("not an eof error")
      ))),
      None => Err(anyhow!(ParserError::UnexpectedEndOfInput)),
    }
  }

  fn current_kind(&self) -> Option<TokenKind> {
    self.current_token.as_ref().map(|t| t.kind)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use rstest::rstest;

  #[rstest]
  #[case(
    "(λx.x)(λy.(λa.a))",
    Node::Application(Box::new(Application {
      lhs: Node::Abstraction(Box::new(Abstraction {
        param: "x",
        body: Node::Identifier(Identifier {
          name: "x",
        }),
      })),
      rhs: Node::Abstraction(Box::new(Abstraction {
        param: "y",
        body: Node::Abstraction(Box::new(Abstraction {
          param: "a",
          body: Node::Identifier(Identifier {
            name: "a",
          })
        }))
      })),
    })),
    "(λx. x) (λy. (λa. a))"
  )]
  fn single_application(
    #[case] input: &str,
    #[case] expected_ast: Node,
    #[case] expected_str: &str,
  ) -> Result<(), anyhow::Error> {
    let mut parser = Parser::new(input);
    let ast = parser.parse_term()?;
    assert_eq!(ast, expected_ast);
    assert_eq!(ast.to_string(), expected_str);
    Ok(())
  }

  #[rstest]
  #[case(
    "(λx.x)(λy.y)(λabc.abc)",
    Node::Application(Box::new(Application {
      // left associative
      lhs: Node::Application(Box::new(Application {
        lhs: Node::Abstraction(Box::new(Abstraction {
          param: "x",
          body: Node::Identifier(Identifier {
            name: "x",
          }),
        })),
        rhs: Node::Abstraction(Box::new(Abstraction {
          param: "y",
          body: Node::Identifier(Identifier {
            name: "y",
          }),
        })),
      })),
      rhs: Node::Abstraction(Box::new(Abstraction {
        param: "abc",
        body: Node::Identifier(Identifier {
          name: "abc",
        }),
      })),
    })),
    "(λx. x) (λy. y) (λabc. abc)"
  )]
  fn double_application(
    #[case] input: &str,
    #[case] expected_ast: Node,
    #[case] expected_str: &str,
  ) -> Result<(), anyhow::Error> {
    let mut parser = Parser::new(input);
    let ast = parser.parse_term()?;
    assert_eq!(ast, expected_ast);
    assert_eq!(ast.to_string(), expected_str);
    Ok(())
  }

  #[rstest]
  #[case("(λx.1)", None, "1")]
  #[case("(λA.a)", None, "A")]
  #[case("(λAbc.Abc)", None, "A")]
  #[case("(3 λx.x)", None, "3")]
  #[case(")λx.x)", Some(TokenKind::RightParen), ")")]
  #[case("(.x.x)", Some(TokenKind::Dot), ".")]
  #[case("(x .)", Some(TokenKind::Dot), ".")]
  #[should_panic]
  #[case("(λaBC.aBC)", None, "")] // first letter must be lower, others are ok
  fn unexpected_token_error(
    #[case] input: &str,
    #[case] expected_kind: Option<TokenKind>,
    #[case] expected_repr: &str,
  ) {
    let mut parser = Parser::new(input);
    let result = parser.parse_term();
    eprintln!("{:?}", &result);
    assert!(matches!(
      result,
      Err(err) if err.downcast_ref::<ParserError>().unwrap() == &ParserError::UnexpectedToken(
        TokenError { kind: expected_kind.unwrap_or(TokenKind::Unknown), text: expected_repr.to_string() }
      )
    ));
  }

  #[rstest]
  #[case("")]
  #[case("(")]
  #[case("(λ")]
  #[case("(λx")]
  #[case("(λx.")]
  #[case("(λx.x")]
  #[case("(λx.x)(")]
  fn unexpected_end_of_input_error(#[case] input: &str) {
    let mut parser = Parser::new(input);
    let result = parser.parse_term();
    assert!(
      matches!(result, Err(err) if err.downcast_ref::<ParserError>().unwrap() == &ParserError::UnexpectedEndOfInput)
    );
  }
}
