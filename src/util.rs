
pub trait Stream<T>{
    fn next(&mut self) -> T;
    fn peek(&self) -> T;
    fn current(&self) -> T;
    fn reset(&mut self);
    fn eof(&self) -> bool;
    fn err(&self, msg: &str);
}

#[derive(Debug)]
pub enum NodeType{
    PUT,
    DECLARE,
    CHANGE,
    IF,
    WHILE,
    EXPRESSION,
    VAL,
    EOF,
    EOL
}
impl NodeType {
    pub fn is_eof(&self) -> bool{
        match self {
            NodeType::EOF => true,
            _ => false,
        }
    }
}
#[derive(Debug)]
pub enum BaseType{
    NUMBER,
    STRING,
    VOID
}
impl Clone for BaseType{
    fn clone(&self) -> BaseType {
        match self{
            BaseType::NUMBER => BaseType::NUMBER,
            BaseType::STRING => BaseType::STRING,
            BaseType::VOID => BaseType::VOID
        }
    }
}

impl Clone for NodeType{
    fn clone(&self) -> NodeType {
        match self {
            NodeType::EOL => NodeType::EOL,
            NodeType::VAL => NodeType::VAL,
            NodeType::PUT => NodeType::PUT,
            NodeType::EXPRESSION => NodeType::EXPRESSION,
            NodeType::DECLARE => NodeType::DECLARE,
            NodeType::CHANGE => NodeType::CHANGE,
            NodeType::IF => NodeType::IF,
            NodeType::WHILE => NodeType::WHILE,
            NodeType::EOF => NodeType::EOF
        }
    }
}

#[derive(Debug)]
//op 8-19
pub enum TokenType{
    ASSIGN,
    COMMA,
    VAR,
    NUMBER,
    STRING,
    TYPENUMBER,
    TYPESTRING,
    WHILE,
    EQUAL,
    ADD,
    SUB,
    MUL,
    DIV,
    MOD,
    AND,
    OR,
    NOT,
    GREATER,
    LESS,
    GOE,
    LOE,
    CAT,
    BEGIN,
    END,
    LBRAC,
    RBRAC,
    IF,
    ELSE,
    PUT,
    COLON,
    EOF,
    CALL,
    FUNCTION,
    RET,
    SEMICOLON,
    POW
}
impl TokenType {
    pub fn is_cat(&self) -> bool{
        match self {
            TokenType::CAT => {
                true
            }
            _ => {
                false
            }
        }
    }

    pub fn is_num_op(&self) -> bool{
        match self {
            TokenType::GOE => {
                true
            }
            TokenType::LOE => {
                true
            }
            TokenType::EQUAL => {
                true
            }
            TokenType::ADD => {
                true
            }
            TokenType::SUB => {
                true
            }
            TokenType::MOD => {
                true
            }
            TokenType::MUL => {
                true
            }
            TokenType::DIV => {
                true
            }
            TokenType::GREATER => {
                true
            }
            TokenType::LESS => {
                true
            }
            TokenType::AND => {
                true
            }
            TokenType::OR => {
                true
            }
            TokenType::NOT => {
                true
            }
            TokenType::POW => {
                true
            }
            _ => {
                false
            }
        }
    }
}
impl Copy for TokenType{}
impl Clone for TokenType{
    fn clone(&self) -> Self{
        match self{
            TokenType::POW => TokenType::POW,
            TokenType::GOE => TokenType::GOE,
            TokenType::LOE => TokenType::LOE,
            TokenType::SEMICOLON => TokenType::SEMICOLON,
            TokenType::CALL => TokenType::CALL,
            TokenType::ASSIGN => TokenType::ASSIGN,
            TokenType::VAR => TokenType::VAR,
            TokenType::NUMBER => TokenType::NUMBER,
            TokenType::STRING => TokenType::STRING,
            TokenType::TYPENUMBER => TokenType::TYPENUMBER,
            TokenType::TYPESTRING => TokenType::TYPESTRING,
            TokenType::WHILE => TokenType::WHILE,
            TokenType::EQUAL => TokenType::EQUAL,
            TokenType::ADD => TokenType::ADD,
            TokenType::SUB => TokenType::SUB,
            TokenType::MUL => TokenType::MUL,
            TokenType::DIV => TokenType::DIV,
            TokenType::MOD => TokenType::MOD,
            TokenType::AND => TokenType::AND,
            TokenType::OR => TokenType::OR,
            TokenType::NOT => TokenType::NOT,
            TokenType::GREATER => TokenType::GREATER,
            TokenType::LESS => TokenType::LESS,
            TokenType::CAT => TokenType::CAT,
            TokenType::BEGIN => TokenType::BEGIN,
            TokenType::END => TokenType::END,
            TokenType::LBRAC => TokenType::LBRAC,
            TokenType::RBRAC => TokenType::RBRAC,
            TokenType::IF => TokenType::IF,
            TokenType::ELSE => TokenType::ELSE,
            TokenType::EOF => TokenType::EOF,
            TokenType::PUT => TokenType::PUT,
            TokenType::COLON => TokenType::COLON,
            TokenType::FUNCTION => TokenType::FUNCTION,
            TokenType::RET => TokenType::RET,
            TokenType::COMMA => TokenType::COMMA
        }
    }
}
