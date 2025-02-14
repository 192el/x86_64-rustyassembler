pub enum Tokens {
    Mnemonic(String),
    Register(String),
    Number(i64),
    Symbol(String),
    Label(String),
}

pub struct Lexer(input: String, position: usize) {

}
