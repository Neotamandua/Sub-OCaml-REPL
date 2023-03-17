use std::collections::HashMap;

use anyhow::Result;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use Sub_OCaml::{
    error::Error::UtilsError, run_code, run_code_with_persistent_environment, ty, type_check, value,
};

fn main() -> Result<()> {
    // `()` can be used when no completer is required
    let mut rl = DefaultEditor::new()?;
    let mut global_type_environment: HashMap<String, ty> = HashMap::new();
    let mut global_value_environment: HashMap<String, Box<value>> = HashMap::new();
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(code) => {
                match run_code_with_persistent_environment(
                    &mut global_type_environment,
                    &mut global_value_environment,
                    &code,
                ) {
                    Ok(r) => {
                        println!("Type: {:?}", r.2);
                        println!("Result: {:?}", r.3);
                    }
                    Err(err) => {
                        println!("{:?}", err);
                    }
                };
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}
