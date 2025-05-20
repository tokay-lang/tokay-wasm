use std::io;
use tokay::{Compiler, Object, Reader, vm::Thread};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn tokay(code: &str, input: &str) -> Option<String> {
    let code = code.to_string();
    let mut input = Reader::new(None, Box::new(io::Cursor::new(input.to_string())));

    let mut compiler = Compiler::new();

    match compiler.compile(Reader::new(None, Box::new(io::Cursor::new(code)))) {
        Ok(None) => None,
        Ok(Some(program)) => {
            let mut thread = Thread::new(&program, vec![&mut input]);
            //thread.debug = compiler.debug;
            //thread.globals = globals;

            match thread.run() {
                Ok(Some(value)) => Some(value.repr()),
                Err(error) => Some(error.to_string()),
                _ => None,
            }

            //globals = thread.globals;
        }
        Err(errors) => Some(format!("{:?}", errors)),
    }
}
