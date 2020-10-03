use crate::lexer::{Lexer, Token};

#[derive(Debug)]
enum Expr {
    Number(f64),
    PrefixExpr {
        operator: String,
        right: Box<Expr>
    },
    InfixExpr {
        left: Box<Expr>,
        operator: String,
        right: Box<Expr>
    }
}

#[derive(Debug)]
struct Parser<'a> {
    lexer: Lexer<'a>,
    curr: Option<Token>,
    peek: Option<Token>
}

impl<'a> Parser<'a> {
    fn new(mut lexer: Lexer<'a>) -> Self {
        let curr = lexer.token(); 
        let peek = lexer.token(); 

        Self { lexer, curr, peek }
    }

    fn next(&mut self) {
        self.curr = self.peek;
        self.peek = self.lexer.token(); 
    }
}

#[derive(PartialEq, PartialOrd, Debug)]
enum Precedence {
    LOWEST,
    SUM, //+ -
    PRODUCT, // * /
    PREFIX //前置演算子 -
}

impl<'a> Parser<'a> {
    fn token_operator(token: &Token) -> String {
        match token {
            Token::Plus => "Plus".to_string(),
            Token::Minus => "Minus".to_string(),
            Token::Asterisk => "Asterisk".to_string(),
            Token::Slash => "Slash".to_string(),
            _ => "".to_string()
        }
    }

    fn token_precedence(token: &Token) -> Precedence {
        match token {
            Token::Plus | Token::Minus => Precedence::SUM,
            Token::Asterisk | Token::Slash => Precedence::PRODUCT,
            _ => Precedence::LOWEST
        }
    }
}

impl<'a> Parser<'a> {
    fn parse(&mut self) -> Option<Box<Expr>> {
        self.parse_expression(Precedence::LOWEST)
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Option<Box<Expr>> {
        let mut left = self.parse_prefix()?;
        while self.peek.is_some() && precedence < Self::token_precedence(&self.peek?) {
            self.next();
            
            left = match self.curr? {
                Token::Plus | Token::Minus | Token::Asterisk | Token::Slash => {
                    self.parse_infix(left)?
                },
                _ => left
            };
        }

        Some(left)
    }

    fn parse_prefix(&mut self) -> Option<Box<Expr>> {
        match self.curr? {
            Token::Number(n) => {
                Some(Box::new(Expr::Number(n)))
            },
            Token::Minus => {
                self.next();
                let number = self.parse_expression(Precedence::PREFIX)?;

                Some(Box::new(Expr::PrefixExpr { operator: "Minus".to_string(), right: number }))
            },
            _ => None
        }
    }

    fn parse_infix(&mut self, left: Box<Expr>) -> Option<Box<Expr>> {
        let operator = Self::token_operator(&self.curr?);
        let precedence = Self::token_precedence(&self.curr?);

        self.next();
        let right = self.parse_expression(precedence)?;
        Some(Box::new(Expr::InfixExpr { left, operator, right }))
    }
}

fn eval(expr: &Expr) -> f64 {
    match expr {
        Expr::Number(n) => *n,
        Expr::PrefixExpr { operator: _, right } => -eval(right),
        Expr::InfixExpr { left, operator, right } => {
            let left = eval(left);
            let right = eval(right);
            match operator.as_str() {
                "Plus" => left + right,
                "Minus" => left - right,
                "Asterisk" => left * right,
                "Slash" => left / right,
                _ => 0_f64
            }
        }
    }
}

pub fn parser_input_value(input: &[u8]) -> Option<f64> {
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let expr = parser.parse();

    match expr {
        Some(expr) => Some(eval(&expr)),
        None => None
    }
}