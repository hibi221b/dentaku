#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
    Number(f64),
    Plus,
    Minus,
    Asterisk,
    Slash
}

#[derive(Debug)]
pub struct Lexer<'a> {
    input: &'a [u8],
    pos: usize
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a [u8]) -> Self {
        Self {
            input,
            pos: 0
        }
    }

    pub fn token(&mut self) -> Option<Token> {

        let current = self.curr()?;
        //現在解析中の値が数値またはドットの時
        let token = if Self::is_number(*current) {

            //1.5+10の場合 String::from("1")になる
            //&[u8] -> char
            let mut numerical_figures = String::new();
            numerical_figures.push(*current as char);  

            //次に調べる値がある and その値が数値かまたはドットの時
            while self.peek().is_some() && Self::is_number(*self.peek().unwrap()) {
                //解析を一つ進める
                self.next();

                //1.5+10を解析していた場合
                //*self.curr().unwrap() as charは self.next()で解析する文字を右に一つ進んだため .になる
                //その次の回では5が追加され String::from("1.5")ができる
                numerical_figures.push(*self.curr().unwrap() as char);
            }
            
            //String::from("1.5")
            numerical_figures
                .parse::<f64>()  //String::from("1.5").parse::<f64>()
                .ok() //成功なら Option<f64>  Some(1.5)
                .and_then(|n| Some(Token::Number(n)))

        } else {
            match current {
                b'+' => Some(Token::Plus),
                b'-' => Some(Token::Minus),
                b'*' => Some(Token::Asterisk),
                b'/' => Some(Token::Slash),
                _ => None
            }
        };

        self.next();
        token
    }

    //インデックスを一つ進める
    fn next(&mut self) {
        self.pos += 1;
    }

    //現在の解析中の文字
    fn curr(&mut self) -> Option<&u8> {
        self.input.get(self.pos)
    }

    //次に解析する文字
    fn peek(&mut self) -> Option<&u8> {
        self.input.get(self.pos + 1)
    }

    //数字 or ドット
    fn is_number(c: u8) -> bool {
        c.is_ascii_digit() || c == b'.'
    }
}

#[test]
fn test_lexer() {
    let mut lexer = Lexer::new("15-2.0/4.56-3.14+100".as_bytes());
    assert_eq!(lexer.token(), Some(Token::Number(15.0_f64)));
    assert_eq!(lexer.token(), Some(Token::Minus));
    assert_eq!(lexer.token(), Some(Token::Number(2.0_f64)));
    assert_eq!(lexer.token(), Some(Token::Slash));
    assert_eq!(lexer.token(), Some(Token::Number(4.56_f64)));
    assert_eq!(lexer.token(), Some(Token::Minus));
    assert_eq!(lexer.token(), Some(Token::Number(3.14_f64)));
    assert_eq!(lexer.token(), Some(Token::Plus));
    assert_eq!(lexer.token(), Some(Token::Number(100.0_f64)));
    assert_eq!(lexer.token(), None);
}