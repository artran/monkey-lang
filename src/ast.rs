pub(crate) enum Node {
    Statement(String),
    Expression(String),
    LetStatement { name: String, value: String },
    Identifier(String),
}

pub(crate) struct Program {
    pub(crate) statements: Vec<Node>,
}

impl Program {
    pub(crate) fn new() -> Program {
        Program {
            statements: Vec::new(),
        }
    }

    pub(crate) fn literal(&self) -> String {
        let mut out = String::new();
        for statement in &self.statements {
            match statement {
                Node::Statement(s) => out.push_str(s),
                Node::Expression(e) => out.push_str(e),
                Node::LetStatement { name, value } => {
                    out.push_str("let ");
                    out.push_str(name);
                    out.push_str(" = ");
                    out.push_str(value);
                    out.push_str(";");
                }
                Node::Identifier(i) => out.push_str(i),
            }
        }
        out
    }
}
