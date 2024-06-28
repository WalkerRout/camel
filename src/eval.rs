use crate::ast::{Abstraction, Application, Identifier, Node};
use std::rc::Rc;

pub fn eval(node: Rc<Node>) -> Rc<Node> {
    match &*node {
        Node::Application(app) => {
            let lhs = eval(app.lhs.clone());
            let rhs = eval(app.rhs.clone());
            if let Node::Abstraction(abs) = &*lhs {
                substitute(&rhs, &abs.body)
            } else {
                Rc::new(Node::Application(Box::new(Application {
                    lhs: lhs.clone(),
                    rhs: rhs.clone(),
                })))
            }
        }
        _ => node.clone(),
    }
}

fn substitute(value: &Rc<Node>, node: &Rc<Node>) -> Rc<Node> {
    match &**node {
        Node::Identifier(id) => {
            if id.name == "x" {
                value.clone()
            } else {
                node.clone()
            }
        }
        Node::Application(app) => Rc::new(Node::Application(Box::new(Application {
            lhs: substitute(value, &app.lhs),
            rhs: substitute(value, &app.rhs),
        }))),
        Node::Abstraction(abs) => Rc::new(Node::Abstraction(Box::new(Abstraction {
            param: abs.param,
            body: substitute(value, &abs.body),
        }))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Parser;
    use crate::ast::{Node, Application, Abstraction, Identifier};
    use std::rc::Rc;

    #[test]
    fn test_evaluation() {
        let input = "(λx.x)(λy.y)";
        let mut parser = Parser::new(input);
        let ast = parser.parse_term().unwrap();
        let result = eval(Rc::new(ast));
        let expected = Rc::new(Node::Abstraction(Box::new(Abstraction {
            param: "y",
            body: Rc::new(Node::Identifier(Identifier { name: "y" })),
        })));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_nested_evaluation() {
        let input = "(λx.(λy.y))(λz.z)";
        let mut parser = Parser::new(input);
        let ast = parser.parse_term().unwrap();
        let result = eval(Rc::new(ast));
        let expected = Rc::new(Node::Abstraction(Box::new(Abstraction {
            param: "y",
            body: Rc::new(Node::Identifier(Identifier { name: "y" })),
        })));
        assert_eq!(result, expected);
    }
}