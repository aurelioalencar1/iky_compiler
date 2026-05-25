mod lexer;
use lexer::Lexer;
fn main() {
    let mut lex = Lexer::new(String::from("fn main() {}"));
    lex.tokenizar();
    println!("posicção inicial: {:?}", lex);

}