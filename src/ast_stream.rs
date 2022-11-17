
use std::{ collections::HashMap, iter};

use crate::{util::{NodeType, Stream, TokenType, BaseType}, token_stream::TokenStream};

#[derive(Debug)]
pub struct Node{
    pub node_type: NodeType,
    pub identity: String,
    pub val_type: BaseType,
    pub str_val: String,
    pub num_val: f64,
    pub l_children: Vec<Node>,
    pub r_children: Vec<Node>,
    pub extra: Vec<Node>
}

impl Clone for Node{
    fn clone(&self) -> Node {
        Node { node_type: self.node_type.clone(), identity: self.identity.clone(), val_type: self.val_type.clone(), str_val: self.str_val.clone(), num_val: self.num_val.clone(), l_children: self.l_children.clone(), r_children: self.r_children.clone(), extra: self.extra.clone() }
    }
}

impl Node{
    pub fn new(node_type: NodeType, id: String) -> Node{
        Node { node_type, identity: id, val_type: BaseType::VOID, str_val: String::from(""), num_val: 0.0, l_children: vec![], r_children: vec![], extra: vec![Node::new_eof()] }
    }
    pub fn new_eof() -> Node{
        Node { node_type: NodeType::EOF, identity: String::from(""), val_type: BaseType::VOID, str_val: String::from(""), num_val: 0.0, l_children: vec![], r_children: vec![], extra: vec![] }
    }
    pub fn append_l(&mut self, node: Node){
        if self.l_children.len() == 1 && self.l_children[0].node_type.is_eof() {
            self.l_children.pop();
        }
        self.l_children.push(node);
    }
    pub fn append_e(&mut self, node: Node){
        if self.extra.len() == 1 && self.extra[0].node_type.is_eof() {
            self.extra.pop();
        }
        self.extra.push(node);
    }
    pub fn append_r(&mut self, node: Node){
        if self.r_children.len() == 1 && self.r_children[0].node_type.is_eof() {
            self.r_children.pop();
        }
        self.r_children.push(node);
    }
}
//var register
pub struct ASTStream{
    pub in_stream: TokenStream,
    cur: Node,
    next: Node,
    registered_var: HashMap<String, BaseType>
}


impl ASTStream{
    /// cur points to the last one that matched     
    /// parse next move to a new line start
    /// parse others point to the first one that matches
    pub fn new(path: &str) -> ASTStream{
        let mut stream = ASTStream { in_stream: TokenStream::new(path), cur: Node::new_eof(), next: Node::new_eof(), registered_var: HashMap::new() };
        stream.next = stream.parse_next();
        stream
    }

    fn register_var(&mut self, var_type: BaseType, var_name: String){
        self.registered_var.extend(iter::once((var_name, var_type)));
    }

    fn parse_val(&mut self) -> Node{
        let mut node = Node::new(NodeType::VAL, String::from(""));
        match self.in_stream.current().token_type {
            TokenType::STRING => {
                node.val_type = BaseType::STRING;
                node.str_val = self.in_stream.current().value;
            }
            TokenType::NUMBER => {
                node.val_type = BaseType::NUMBER;
                node.num_val = self.in_stream.current().value.parse::<f64>().unwrap();
            }
            _ => {
                self.err("prase val type err");
            }
        }
        node
    }

    fn parse_expression(&mut self) -> Node{
        let mut node = Node::new(NodeType::EXPRESSION, String::from(""));
        match self.in_stream.current().token_type {
            TokenType::STRING => {
                node.val_type = BaseType::STRING;
                match self.in_stream.peek().token_type {
                    TokenType::CAT => {
                        node.str_val = String::from("|");
                        node.append_l(self.parse_val());
                        self.in_stream.next();
                        self.in_stream.next();
                        node.append_r(self.parse_expression());
                    }
                    _ => {
                        node = self.parse_val();
                    }
                }
            }
            TokenType::LBRAC => {
                self.in_stream.next();
                let tmp = self.parse_expression();
                self.in_stream.next();
                if self.in_stream.peek().token_type.is_num_op() || self.in_stream.peek().token_type.is_cat() {
                    if self.in_stream.peek().token_type.is_num_op() {
                        node.val_type = BaseType::NUMBER;
                    }
                    else if self.in_stream.peek().token_type.is_cat() {
                        node.val_type = BaseType::STRING;
                    }
                    node.append_l(tmp);
                    node.str_val = self.in_stream.next().value;
                    self.in_stream.next();
                    node.append_r(self.parse_expression());
                }
                else {
                    node = tmp;
                }
            }
            TokenType::VAR => {
                let mut tmp = Node::new(NodeType::VAL, self.in_stream.current().value);
                tmp.val_type = self.registered_var[&tmp.identity].clone();
                if self.in_stream.peek().token_type.is_num_op() ||  self.in_stream.peek().token_type.is_cat() {
                    node.val_type = self.registered_var[&tmp.identity].clone();
                    node.str_val = self.in_stream.peek().value;
                    node.append_l(tmp);
                    self.in_stream.next();
                    self.in_stream.next();
                    node.append_r(self.parse_expression());
                }
                else{
                    node = tmp;
                }
            }
            TokenType::NOT => {
                node.val_type = BaseType::NUMBER;
                node.str_val = String::from("!");
                self.in_stream.next();
                node.append_r(self.parse_expression());
            }
            TokenType::NUMBER =>{
                node.val_type = BaseType::NUMBER;
                let next_token = self.in_stream.peek();
                if next_token.token_type.is_num_op() {
                    node.str_val = next_token.value;
                    node.append_l(self.parse_val());
                    self.in_stream.next();
                    self.in_stream.next();
                    node.append_r(self.parse_expression());
                }
                else {
                    node = self.parse_val();
                }
            }
            _ => {
                self.err("Illegal ch in expression");
            }
        }
        node
    }

    fn parse_declare(&mut self) -> Node {
        let name = self.in_stream.current().value;
        let mut node = Node::new(NodeType::DECLARE, self.in_stream.current().value);
        //skip :
        self.in_stream.next();
        let type_token = self.in_stream.next();
        //now cur points to the type token
        match type_token.token_type {
            TokenType::TYPESTRING => {
                node.val_type = BaseType::STRING;
                self.register_var(BaseType::STRING, name);
            }
            TokenType::TYPENUMBER => {
                node.val_type = BaseType::NUMBER;
                self.register_var(BaseType::NUMBER, name);
            }
            _ => {
                self.err("WRONG parsing TYPE");
            }
        }
        //skip =
        self.in_stream.next();
        //now cur points to the =
        self.in_stream.next();
        node.append_l(self.parse_expression());
        node
    }

    fn parse_put(&mut self) -> Node{
        let mut node = Node::new(NodeType::PUT, String::from(""));
        self.in_stream.next();
        node.append_l(self.parse_expression());
        node
    }

    fn parse_if(&mut self) -> Node {
        let mut node = Node::new(NodeType::IF, String::from(""));
        self.in_stream.next();
        node.append_e(self.parse_expression());
        self.in_stream.next();
        loop{
            match self.in_stream.peek().token_type {
                TokenType::END => {
                    break;
                }
                _ => {
                    node.append_l(self.parse_next());
                }
            }
        }
        self.in_stream.next();
        self.in_stream.next();
        match self.in_stream.current().token_type {
            TokenType::ELSE => {
                self.in_stream.next();
                loop {
                    match self.in_stream.peek().token_type {
                        TokenType::END => {
                            break;
                        }
                        _ => {
                            node.append_r(self.parse_next());
                        }
                    }
                }
            }
            TokenType::SEMICOLON => {
                return node;
            }
            _ => {
                self.err("parse if, there should be a ; after }");
            }
        }
        self.in_stream.next();
        node
    }

    fn parse_change(&mut self) -> Node {
        let mut node = Node::new(NodeType::CHANGE, self.in_stream.current().value);
        self.in_stream.next();
        self.in_stream.next();
        node.append_l(self.parse_expression());
        node
    }

    fn parse_while(&mut self) -> Node {
        let mut node = Node::new(NodeType::WHILE, String::from(""));
        self.in_stream.next();
        node.append_e(self.parse_expression());
        self.in_stream.next();
        loop {
            match self.in_stream.peek().token_type {
                TokenType::END => {
                    break;
                }
                _ => {
                    node.append_l(self.parse_next());
                }
            }
        }
        self.in_stream.next();
        self.in_stream.next();
        node
    }

    fn parse_next(&mut self) -> Node{
        let cur_token = self.in_stream.next();
        match cur_token.token_type {
            TokenType::EOF => {
                Node::new_eof()
            }
            TokenType::SEMICOLON => {
                Node::new(NodeType::EOL, String::from(""))
            }
            TokenType::WHILE => {
                self.parse_while()
            }
            TokenType::IF => {
                self.parse_if()
            }
            TokenType::VAR => {
                let next_token = self.in_stream.peek();
                match next_token.token_type {
                    TokenType::COLON => {
                        self.parse_declare()
                    }
                    TokenType::ASSIGN => {
                        self.parse_change()
                    }
                    _ => {
                        self.err("VAR FALLOWED AN WRONG CH");
                        Node::new_eof()
                    }
                }
            }
            TokenType::PUT => {
                self.parse_put()
            }
            _ => {
                println!("cur token: {:?}", cur_token);
                self.err("WRONG START OF LINE");
                Node::new_eof()
            }
        }
    }
}


impl Stream<Node> for ASTStream{
    fn next(&mut self) -> Node {
        self.cur = self.next.clone();
        self.next = self.parse_next();
        self.cur.clone()
    }
    fn peek(&self) -> Node {
        self.next.clone()
    }
    fn current(&self) -> Node {
        self.cur.clone()
    }
    fn reset(&mut self) {
        self.in_stream.reset();
        self.cur = Node::new_eof();
        self.next = self.parse_next();
    }
    fn eof(&self) -> bool {
        match self.next.node_type {
            NodeType::EOF => {
                true
            }
            _ => {
                false
            }
        }
    }
    fn err(&self, msg: &str) {
        self.in_stream.err(msg);
    }
}