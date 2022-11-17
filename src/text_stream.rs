use std::{fs::File, io::Read};

use crate::util::Stream;


pub struct TextStream{
    //the pos here is the next pos
    pos: usize,
    row: usize,
    col: usize,
    content: Vec<u8>,
    cur: u8,
    next: u8
}

impl TextStream{
    pub fn new(path: &str) -> TextStream{
        let mut f = File::open(path).unwrap();
        let mut buf: Vec<u8> = vec![];
        f.read_to_end(&mut buf).unwrap();
        let first = buf[0];
        TextStream { pos: 0, row: 1, col: 1, content: buf, cur: 0, next: first}
    }
}

impl Stream<u8> for TextStream{
    fn next(&mut self) -> u8{
        self.pos += 1;
        self.col += 1;
        if self.cur == '\n' as u8 {
            self.col = 0;
            self.row += 1;
        }
        self.cur = self.next;
        if self.pos < self.content.len() {
            self.next = self.content[self.pos];
        }
        else{
            self.next = 0;
        }
        self.cur
    }

    fn peek(&self) -> u8{
        self.next
    }

    fn current(&self) -> u8{
        self.cur
    }

    fn reset(&mut self){
        self.pos = 0;
        self.row = 1;
        self.col = 1;
        self.cur = 0;
        self.next = self.content[self.pos];
    }

    fn eof(&self) -> bool{
        self.next == 0
    }

    fn err(&self, msg: &str) {
        panic!("{} row: {} col: {}", msg, self.row, self.col);
    }
}