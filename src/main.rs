// 参考文献:  Rust でつくるインタプリタ 
// 著者:     https://qiita.com/nirasan
// url:     https://qiita.com/nirasan/items/f7a232af3372ea370f4b

mod lexer;
mod parser;
use parser::parser_input_value;

fn main() {
    let result = match parser_input_value(b"2+8/4") {
        Some(p) => p,
        None => {
            eprintln!("error");
            std::process::exit(1);
        }
    };

    println!("result: {}", result);
}