use console_error_panic_hook;
use console_log;
use log;
use std::io;
use tokay::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn enable_logging() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).expect("logger init failed");
}

#[wasm_bindgen]
pub fn run(code: &str, input: &str, print_fn: Option<js_sys::Function>) -> Option<String> {
    let code = code.to_string();
    let mut input = Reader::new(None, Box::new(io::Cursor::new(input.to_string())));

    let mut compiler = Compiler::new();
    compiler.constant(
        "print",
        RefValue::from(tokay::value::Func {
            name: "print",
            func: Box::new(move |context, args, _nargs| {
                let mut output = if args.len() == 0 && context.is_some() {
                    let context = context.unwrap();

                    if let Some(mut capture) = context.get_capture(0) {
                        let value = capture.extract(context.thread.reader);
                        value.to_string()
                    }
                    else {
                        String::new()
                    }
                } else {
                    let mut output = String::new();

                    for i in 0..args.len() {
                        if i > 0 {
                            output.push(' ');
                        }

                        output.push_str(&args[i].to_string());
                    }

                    output
                };

                output.push('\n');

                let output = JsValue::from_str(&output);
                match &print_fn {
                    Some(f) => {
                        let _ = f.call1(&JsValue::NULL, &output);
                    }
                    None => {
                        web_sys::console::log_1(&output);
                    }
                }

                Value::Void.into() // need to push a void with high severity
            }),
        }),
    );

    match compiler.compile(Reader::new(None, Box::new(io::Cursor::new(code)))) {
        Ok(None) => None,
        Ok(Some(program)) => {
            let mut thread = vm::Thread::new(&program, vec![&mut input]);
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
