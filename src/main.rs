use std::io;

use brainfuck_rust::VM;

fn main() {
    // initialize vm
    let mut vm = VM::new();

    // line buffer
    let mut line_buffer = String::new();

    // read until eof
    loop {
        match io::stdin().read_line(&mut line_buffer) {
            Ok(n) => {
                // EOF when n == 0
                if n == 0 {
                    break;
                }
            }
            Err(e) => panic!("{}", e),
        }

        vm.load_program(line_buffer.as_bytes());
        line_buffer.clear();
    }

    // step vm until it reaches the end of the program buffer
    while vm.step() {}

    // flush the output
    let mut o: Vec<u8> = Vec::new();
    loop {
        match vm.output() {
            Some(c) => o.push(c),
            None => break,
        }
    }

    // write output to stdout
    let s: String;
    match String::from_utf8(o) {
        Ok(i) => s = i,
        Err(e) => panic!("{}", e),
    }

    print!("{}", s);
}
