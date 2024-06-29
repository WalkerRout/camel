use std::fmt;
use std::rc::Rc;

/// Nodes in the Abstract Syntax Tree
///
/// Application: t1 t2
/// Abstraction: λx. t1
/// Identifier:  x
#[derive(Debug, PartialEq)]
pub enum Node<'inp> {
  Abstraction(Abstraction<'inp>),
  Application(Application<'inp>),
  Identifier(Identifier<'inp>),
}

/// An abstraction of a lambda function, containing a parameter and a body
#[derive(Debug, PartialEq)]
pub struct Abstraction<'inp> {
  pub param: &'inp str,
  pub body: Rc<Node<'inp>>,
}

#[derive(Debug, PartialEq)]
pub struct Application<'inp> {
  pub lhs: Rc<Node<'inp>>,
  pub rhs: Rc<Node<'inp>>,
}

#[derive(Debug, PartialEq)]
pub struct Identifier<'inp> {
  pub name: &'inp str,
}

impl fmt::Display for Node<'_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Node::Abstraction(abs) => write!(f, "(λ{}. {})", abs.param, abs.body),
      Node::Application(app) => write!(f, "{} {}", app.lhs, app.rhs),
      Node::Identifier(id) => write!(f, "{}", id.name),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use rstest::rstest;

  #[rstest]
  #[case(
    Node::Application(Application {
      lhs: Rc::new(Node::Abstraction(Abstraction {
        param: "x",
        body: Rc::new(Node::Identifier(Identifier {
          name: "x",
        })),
      })),
      rhs: Rc::new(Node::Abstraction(Abstraction {
        param: "y",
        body: Rc::new(Node::Identifier(Identifier {
          name: "y",
        })),
      })),
    }),
    "(λx. x) (λy. y)"
  )]
  fn simple_ast(#[case] ast: Node, #[case] expected_str: &str) {
    assert_eq!(ast.to_string(), expected_str);
  }
}
