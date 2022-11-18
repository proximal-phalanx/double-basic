use std::io::stdin;

mod util;
mod text_stream;
mod token_stream;
mod ast_stream;
mod parser;

fn main() {
    println!("Type in the file path (type in !number to run sample code)");
    let mut buf = String::from("");
    stdin().read_line(&mut buf).unwrap();
    if buf.starts_with('!'){
        buf = buf[1..].to_string();
        buf = buf.trim_end().to_string();
        buf = format!("./samples/sample{}.bb", buf);
    }
    else{
        buf = buf.trim_end().to_string();
    }
    println!("{}", buf);
    let mut parser = parser::Parser::open_file(&buf);
    parser.run();
    // let debug = false;
    // while !stream.eof() {
    //     let tmp = stream.next();
    //     match tmp.node_type {
    //         NodeType::EOL => {
                // println!("EOL");
    //         }
    //         _ => {
    //             if debug {
    //                 for _ in 0..2{
    //                     print!("-------------------------------------");
    //                 }
    //                 println!("---------------------------");
    //                 println!("{:#?}", tmp);
    //             }
    //             parser.run(tmp);
    //         }
    //     }
    // }
}
