use crate::{util::{TokenType, Stream}, text_stream::TextStream};

#[derive(Debug)]
pub struct Token{
    pub token_type: TokenType,
    pub value: String,
}

impl Token{
    pub fn new(token_type: TokenType, value: String) -> Token{
        Token { token_type, value }
    }
    pub fn new_eof() -> Token{
        Token { token_type: TokenType::EOF, value: String::from("") }
    }
}

impl Clone for Token{
    fn clone(&self) -> Token {
        Token { token_type: self.token_type.clone(), value: self.value.clone() }
    }
}
pub struct TokenStream{
    in_stream: TextStream,
    cur: Token,
    next: Token,
}


impl TokenStream{
    fn read_until_f(&mut self, test: fn(ch: u8) -> bool, cur: u8) -> String{
        let mut buf: Vec<u8> = vec![];
        buf.push(cur);
        let mut ret: Vec<u8> = vec![];
        while !self.in_stream.eof(){
            if test(self.in_stream.peek()) {
                buf.push(self.in_stream.next());
            }    
            else{
                ret = buf;
                break;
            }
        }
        String::from_utf8(ret).unwrap()
    }
    fn read_until(&mut self, test: fn(ch: u8) -> bool) -> String{
        let mut buf: Vec<u8> = vec![];
        let mut ret: Vec<u8> = vec![];
        while !self.in_stream.eof(){
            if test(self.in_stream.peek()) {
                buf.push(self.in_stream.next());
            }    
            else{
                ret = buf;
                break;
            }
        }
        String::from_utf8(ret).unwrap()
    }

    pub fn prase_next(&mut self) -> Token{
        let mut ch = self.in_stream.next();
        while ch == ' ' as u8 || ch == '\n' as u8 || ch == '\t' as u8{
            ch = self.in_stream.next();
        }
        let mut ret = Token::new_eof();
        if ch.is_ascii_digit() {
            let buf = self.read_until_f(|ch| {ch.is_ascii_digit() || ch == '.' as u8}, ch);
            ret = Token::new(TokenType::NUMBER, buf);
        }
        if ch == '\"' as u8 { 
            let buf = self.read_until(|ch: u8| {ch != '\"' as u8});
            self.in_stream.next();
            ret = Token::new(TokenType::STRING, buf);
        }
        if ch.is_ascii_alphabetic(){
            let word = self.read_until_f(|ch: u8| {ch.is_ascii_alphabetic() || ch.is_ascii_digit() || ch == '_' as u8}, ch);
            if word == "NUMBER" {
                ret = Token::new(TokenType::TYPENUMBER, word);
            }
            else if word == "CALL"{
                ret = Token::new(TokenType::CALL, word);
            }
            else if word == "STRING" {
                ret = Token::new(TokenType::TYPESTRING, word);
            }
            else if word == "WHILE" {
                ret = Token::new(TokenType::WHILE, word);
            }
            else if word == "IF"{
                ret = Token::new(TokenType::IF, word);
            }
            else if word == "ELSE"{
                ret = Token::new(TokenType::ELSE, word);
            }
            else if word == "RET"{
                ret = Token::new(TokenType::RET, word);
            }
            else if word == "FUNCTION"{
                ret = Token::new(TokenType::FUNCTION, word);
            }
            else if word == "PUT"{
                ret = Token::new(TokenType::PUT, word);
            }
            else{
                ret = Token::new(TokenType::VAR, word);
            }
        }
        if ch == '{' as u8{
            ret = Token::new(TokenType::BEGIN, String::from_utf8(vec![ch]).unwrap());
        }
        else if ch == '}' as u8{
            ret = Token::new(TokenType::END, String::from_utf8(vec![ch]).unwrap());
        }
        else if ch == '(' as u8{
            ret = Token::new(TokenType::LBRAC, String::from_utf8(vec![ch]).unwrap());
        }
        else if ch == ')' as u8{
            ret = Token::new(TokenType::RBRAC, String::from_utf8(vec![ch]).unwrap());
        }
        else if ch == '=' as u8{
            if self.in_stream.peek() == '=' as u8{
                self.in_stream.next();
                ret = Token::new(TokenType::EQUAL, String::from("=="));
            }
            else{
                ret = Token::new(TokenType::ASSIGN, String::from_utf8(vec![ch]).unwrap());
            }
        }

        else if ch == '+' as u8{
            ret = Token::new(TokenType::ADD, String::from_utf8(vec![ch]).unwrap());
        }
        else if ch == '-' as u8{
            if self.in_stream.peek().is_ascii_digit() {
                let buf = self.read_until_f(|ch| {ch.is_ascii_digit() || ch == '.' as u8}, ch);
                ret = Token::new(TokenType::NUMBER, buf);
            }
            else{
                ret = Token::new(TokenType::SUB, String::from_utf8(vec![ch]).unwrap());
            }
        }
        else if ch == '*' as u8{
            ret = Token::new(TokenType::MUL, String::from_utf8(vec![ch]).unwrap());
        }
        else if ch == '/' as u8{
            ret = Token::new(TokenType::DIV, String::from_utf8(vec![ch]).unwrap());
        }
        else if ch == '%' as u8{
            ret = Token::new(TokenType::MOD, String::from_utf8(vec![ch]).unwrap());
        }
        else if ch == '&' as u8{
            if self.in_stream.peek() == '&' as u8{
                self.in_stream.next();
                ret = Token::new(TokenType::AND, String::from("&&"));
            }
        }
        else if ch == '|' as u8{
            if self.in_stream.peek() == '|' as u8{
                self.in_stream.next();
                ret = Token::new(TokenType::OR, String::from("||"));
            }
            else{
                ret = Token::new(TokenType::CAT, String::from_utf8(vec![ch]).unwrap());
            }
        }
        else if ch == ':' as u8{
            ret = Token::new(TokenType::COLON, String::from_utf8(vec![ch]).unwrap());
        }
        else if ch == ',' as u8{
            ret = Token::new(TokenType::COMMA, String::from_utf8(vec![ch]).unwrap());
        }
        else if ch == '!' as u8{
            ret = Token::new(TokenType::NOT, String::from_utf8(vec![ch]).unwrap());
        }
        else if ch == '>' as u8{
            if self.in_stream.peek() == '=' as u8{
                self.in_stream.next();
                ret = Token::new(TokenType::GOE, String::from(">="));
            }
            else{
                ret = Token::new(TokenType::GREATER, String::from_utf8(vec![ch]).unwrap());
            }
        }
        else if ch == '<' as u8{
            if self.in_stream.peek() == '=' as u8{
                self.in_stream.next();
                ret = Token::new(TokenType::LOE, String::from("<="));
            }
            else{
                ret = Token::new(TokenType::LESS, String::from_utf8(vec![ch]).unwrap());
            }
        }
        else if ch == ';' as u8{
            ret = Token::new(TokenType::SEMICOLON, String::from_utf8(vec![ch]).unwrap());
        }
        else if ch == '^' as u8{
            ret = Token::new(TokenType::POW, String::from_utf8(vec![ch]).unwrap());
        }
        ret
    }

    pub fn new(path: &str) -> TokenStream{
        let in_stream = TextStream::new(path);
        let mut stream = TokenStream{
            in_stream,
            cur: Token::new_eof(),
            next: Token::new_eof(),
        };
        stream.next();
        stream
    }
}
impl Stream<Token> for TokenStream{
    fn next(&mut self) -> Token{
        self.cur = self.next.clone();
        self.next = self.prase_next();
        self.cur.clone()
    }
    fn current(&self) -> Token {
        self.cur.clone()
    }
    fn peek(&self) -> Token {
        self.next.clone()
    }
    fn reset(&mut self) {
        self.in_stream.reset();
        self.cur = Token::new_eof();
        self.next = Token::new_eof();
    }
    fn err(&self, msg: &str) {
        self.in_stream.err(msg);
    }
    fn eof(&self) -> bool {
        self.in_stream.eof()
    }
}