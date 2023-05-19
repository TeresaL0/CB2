use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum C1Token {
    // TODO: Define variants and their token/regex
    
    // Schlüsselwörter
    #[token("bool")]
    KW_BOOLEAN,

    #[token("do")]
    KW_DO,

    #[token("else")]
    KW_ELSE,

    #[token("float")]
    KW_FLOAT,

    #[token("for")]
    KW_FOR,

    #[token("if")]
    KW_IF,

    #[token("int")]
    KW_INT,

    #[token("printf")]
    KW_PRINTF,

    #[token("return")]
    KW_RETURN,

    #[token("void")]
    KW_VOID,

    #[token("while")]
    KW_WHILE,


    // Operatoren
    #[token("+")]
    PLUS,

    #[token("-")]
    MINUS,

    #[token("*")]
    ASTERISK,

    #[token("/")]
    SLASH,

    #[token("=")]
    ASSIGN,

    #[token("==")]
    EQ,

    #[token("!=")]
    NEQ,

    #[token("<")]
    LSS,

    #[token(">")]
    GRT,

    #[token("<=")]
    LEQ,

    #[token(">=")]
    GEQ,

    #[token("&&")]
    AND,

    #[token("||")]
    OR,


    // Sonstige Token
    #[token(",")]
    COMMA,

    #[token(";")]
    SEMICOLON,

    #[token("(")]
    LPAREN,

    #[token(")")]
    RPAREN,

    #[token("{")]
    LBRACE,

    #[token("}")]
    RBRACE,


    // Pseudotoken
    #[regex("[0-9]")]
    DIGIT,
    
    #[regex("{DIGIT}+")]                            // ?
    INTEGER,

    #[regex("{INTEGER}.{INTEGER}|.{INTEGER}")]      // ?
    FLOAT,

     #[regex("[a-zA-Z]")]
    LETTER, 	


    // Termvariablen
    #[regex("{INTEGER}")]
    CONST_INT,

    #[regex("{FLOAT}([eE]([-+])?{INTEGER})?|{INTEGER}[eE]([-+])?{INTEGER}")]
    CONST_FLOAT,

    #[regex("true|false")]
    CONST_BOOLEAN,

    #[regex("")]                        // Term einfügen                                           
    CONST_STRING,

     #[regex("({LETTER})+ ({DIGIT} | {LETTER})*")]
    ID,


    // Kommentare
    #[token("/*")]    
    BEGIN_C_COMMENT,

    #[token("*/")]  
    END_C_COMMENT,

    #[token("//")]  
    CPLUS_COMMENT,
               
    // "/*" <comment> "*/",                
    // "//" <comment> "\n"


    // Logos requires one token variant to handle errors,
    // it can be named anything you wish.
    #[error]
    Error,
}

// Token::FLOAT ... bspw.
