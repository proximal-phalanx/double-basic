use core::panic;
use std::{collections::HashMap, iter};

use crate::{ast_stream::{ASTStream, Node}, util::{NodeType, BaseType, Stream}};

pub struct Parser {
    in_stream: ASTStream,
    num_var: HashMap<String, f64>,
    str_var: HashMap<String, String>
}

impl Parser {
    pub fn open_file(file_name: &str) -> Parser {
        Parser {
            in_stream: ASTStream::new(file_name),
            num_var: HashMap::new(),
            str_var: HashMap::new()
        }
    }

    fn get_val_num(&self, root: Node) -> f64 {
        match root.node_type {
            NodeType::VAL => {
                if root.identity == "" {
                    return root.num_val;
                }
                else {
                    return self.num_var.get(&root.identity).unwrap().clone();
                }
            }
            NodeType::EXPRESSION => {
                let op = root.str_val.clone();
                match op.as_str() {
                    "+" => {
                        return self.get_val_num(root.l_children[0].clone()) + self.get_val_num(root.r_children[0].clone());
                    }
                    "-" => {
                        return self.get_val_num(root.l_children[0].clone()) - self.get_val_num(root.r_children[0].clone());
                    }
                    "*" => {
                        return self.get_val_num(root.l_children[0].clone()) * self.get_val_num(root.r_children[0].clone());
                    }
                    "/" => {
                        return self.get_val_num(root.l_children[0].clone()) / self.get_val_num(root.r_children[0].clone());
                    }
                    "%" => {
                        return self.get_val_num(root.l_children[0].clone()) % self.get_val_num(root.r_children[0].clone());
                    }
                    "^" => {
                        return self.get_val_num(root.l_children[0].clone()).powf(self.get_val_num(root.r_children[0].clone()));
                    }
                    ">" => {
                        return if self.get_val_num(root.l_children[0].clone()) > self.get_val_num(root.r_children[0].clone()) { 1.0 } else { 0.0 };
                    }
                    "<" => {
                        return if self.get_val_num(root.l_children[0].clone()) < self.get_val_num(root.r_children[0].clone()) { 1.0 } else { 0.0 };
                    }
                    "<=" => {
                        return if self.get_val_num(root.l_children[0].clone()) <= self.get_val_num(root.r_children[0].clone()) { 1.0 } else { 0.0 };
                    }
                    ">=" => {
                        return if self.get_val_num(root.l_children[0].clone()) >= self.get_val_num(root.r_children[0].clone()) { 1.0 } else { 0.0 };
                    }
                    "==" => {
                        return if self.get_val_num(root.l_children[0].clone()) == self.get_val_num(root.r_children[0].clone()) { 1.0 } else { 0.0 };
                    }
                    "!=" => {
                        return if self.get_val_num(root.l_children[0].clone()) != self.get_val_num(root.r_children[0].clone()) { 1.0 } else { 0.0 };
                    }
                    "&&" => {
                        return if self.get_val_num(root.l_children[0].clone()) != 0.0 && self.get_val_num(root.r_children[0].clone()) != 0.0 { 1.0 } else { 0.0 };
                    }
                    "||" => {
                        return if self.get_val_num(root.l_children[0].clone()) != 0.0 || self.get_val_num(root.r_children[0].clone()) != 0.0 { 1.0 } else { 0.0 };
                    }
                    "!" => {
                        return if self.get_val_num(root.r_children[0].clone()) == 0.0 { 1.0 } else { 0.0 };
                    }
                    _ => {
                        panic!("Not a valid operator");
                    }
                }
            }
            _ => {
                panic!("ERROR: not a number or expression");
            }
        }
    }

    fn get_val_str(&self, root: Node) -> String {
        match root.node_type {
            NodeType::VAL => {
                if root.identity == "" {
                    return root.str_val;
                }
                else {
                    return self.str_var.get(&root.identity).unwrap().clone();
                }
            }
            NodeType::EXPRESSION => {
                let op = root.str_val.clone();
                match op.as_str() {
                    "|" => {
                        return self.get_val_str(root.l_children[0].clone()) + &self.get_val_str(root.r_children[0].clone());
                    }
                    _ => {
                        panic!("Not a valid operator");
                    }
                }
            }
            _ => {
                panic!("ERROR: not a number or expression");
            }
        }
    }

    fn replace_num_var(&mut self, name: &str, val: f64) {
        self.num_var.insert(name.to_string(), val);
    }

    fn replace_str_var(&mut self, name: &str, val: String) {
        self.str_var.insert(name.to_string(), val);
    }

    fn reg_var(&mut self, root: Node) {
        match root.val_type {
            BaseType::NUMBER => {
                self.num_var.extend(iter::once((root.identity.clone(), self.get_val_num(root.l_children[0].clone()))));
            }
            BaseType::STRING => {
                self.str_var.extend(iter::once((root.identity.clone(), self.get_val_str(root.l_children[0].clone()))));
            }
            BaseType::VOID => {
                panic!("ERROR: void type");
            }
        }
    }

    pub fn run_node(&mut self, n: Node) {
        match n.node_type {
            NodeType::EOL | NodeType::EOF => {
                return;
            }
            NodeType::DECLARE => {
                self.reg_var(n);
            }
            NodeType::PUT => {
                match n.l_children[0].val_type {
                    BaseType::NUMBER => {
                        println!("{}", self.get_val_num(n.l_children[0].clone()));
                    }
                    BaseType::STRING => {
                        println!("{}", self.get_val_str(n.l_children[0].clone()));
                    }
                    BaseType::VOID => {
                        panic!("ERROR: void type in put");
                    }
                }
            }
            NodeType::CHANGE => {
                match n.l_children[0].val_type {
                    BaseType::NUMBER => {
                        self.replace_num_var(&n.identity, self.get_val_num(n.l_children[0].clone()));
                    }
                    BaseType::STRING => {
                        self.replace_str_var(&n.identity, self.get_val_str(n.l_children[0].clone()));
                    }
                    BaseType::VOID => {
                        panic!("ERROR: void type in change");
                    }
                }
            }
            NodeType::IF => {
                let cond = self.get_val_num(n.extra[0].clone());
                if cond != 0.00 {
                    for each in n.l_children {
                        self.run_node(each.clone());
                    }
                }
                else{
                    for each in n.r_children {
                        self.run_node(each.clone());
                    }
                }
            }
            NodeType::WHILE => {
                loop{
                    let cond = self.get_val_num(n.extra[0].clone());
                    if cond == 0.00 {
                        break;
                    }
                    else{
                        for each in n.l_children.clone() {
                            self.run_node(each.clone());
                        }
                    }
                }
            }
            _ => {

            }
        }
    }

    pub fn run(&mut self) {
        while !self.in_stream.eof() {
            let tmp = self.in_stream.next();
            match tmp.node_type {
                NodeType::EOL => {

                }
                _ => {
                    self.run_node(tmp);
                }
            }
        }
    }
}