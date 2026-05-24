#[derive(Debug)]
pub enum Token{

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
PontoVirgula,
DoisPontos,
Virgula,
Ponto,
Arroba,

// — Especial —
EOF,






}

// Struct lexer

pub struct Lexer{
    fonte: String,
    posicao: usize,
    tokens: Vec<Token>,
}


// --- implementação ---

impl Lexer{
    pub fn new(sourcer: String) -> Lexer{
        Lexer {
            fonte: sourcer.chars().collect(),
            posicao: 0,
            tokens: Vec::new(),
        }

        }
fn char_atual(&self) -> Option<char>{
    self.fonte.get(self.posicao).copied()

}

}
