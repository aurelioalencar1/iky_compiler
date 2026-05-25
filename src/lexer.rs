#[derive(Debug)]
pub enum Token {
    // –– Palavras chaves —
    Let,
    Fn,
    Return,
    Arena,
    Pub,
    Struct,
    Extend,
    For,
    If,
    Use,
    Try,

    // --- Operadores ---
    DeclaraAtrib,
    IgualIgual,
    Diferente,
    Mais,
    Menos,
    Exclamacao,
    Maior,
    Menor,

    // ---- Identificador ----
    Identificador(String),

    // ---- Literais -----
    LitInt(i64),

    LitFloat(f64),

    LitString(String),

    LitBool(bool),

    // --- Tipos Primitivos ---
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    Bool,
    Str,

    // –– Pontuação —–
    ParenEsq,
    ParenDir,
    ChaveEsq,
    ChaveDir,
    ColcheteEsq,
    ColcheteDir,
    PontoVirgula,
    DoisPontos,
    Virgula,
    Ponto,
    Arroba,

    // — Especial —
    EOF,
}

// Struct lexer
#[derive(Debug)]
pub struct Lexer {
    fonte: Vec<char>,
    posicao: usize,
    tokens: Vec<Token>,
}

// --- implementação ---

impl Lexer {
    pub fn new(source: String) -> Lexer {
        Lexer {
            fonte: source.chars().collect(),
            posicao: 0,
            tokens: Vec::new(),
        }
    }
    fn char_atual(&self) -> Option<char> {
        self.fonte.get(self.posicao).copied()
    }

    fn avancar(&mut self) {
        self.posicao += 1;
    }

    pub fn tokenizar(&mut self){
        while self.posicao < self.fonte.len() {
            let c = self.char_atual();

            match c {
                Some(' ') | Some('\t') | Some('\n') => {
                    self.avancar();
                },// Ignorar espaços em branco

                Some('(') => self.tokens.push(Token::ParenEsq),
                Some(')') => self.tokens.push(Token::ParenDir),
                Some('{') => self.tokens.push(Token::ChaveEsq),
                Some('}') => self.tokens.push(Token::ChaveDir),
                Some('[') => self.tokens.push(Token::ColcheteEsq),
                Some(']') => self.tokens.push(Token::ColcheteDir),
                Some(';') => self.tokens.push(Token::PontoVirgula),
                Some(':') => self.tokens.push(Token::DoisPontos),
                Some(',') => self.tokens.push(Token::Virgula),
                Some('.') => self.tokens.push(Token::Ponto),
                Some('@') => self.tokens.push(Token::Arroba),
                _ => {}
            
                    
            }

            self.avancar();
        }
        self.tokens.push(Token::EOF);
    }
}
