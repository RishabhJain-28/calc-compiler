mod analyzer;
mod executor;
mod parser;
mod symbol_table;

fn main() {
    run_interpreter()
}

fn run_interpreter() {
    eprintln!("* Calc interactive interpreter *");
    let mut variables = symbol_table::SymbolTable::new();
    loop {
        let cmd: String = input_command();
        // if cmd == "exit" {
        //     break;
        // }

        match cmd.trim() {
            "q" | "exit" | "quit" => break,
            "c" | "clear" => {
                variables = symbol_table::SymbolTable::new();
                eprintln!("Cleared Variables");
            }
            "v" | "variables" => {
                eprintln!("Variables:");
                for v in variables.iter() {
                    eprintln!("{} : {}", v.0, v.1)
                }
            }

            input => {
                let parsed_program;
                match parser::parse_program(&input) {
                    Ok((rest, syntax_tree)) => {
                        let trimmed_rest = rest.trim();
                        if trimmed_rest.len() > 0 {
                            eprintln!("Unparsed input: `{}`.", trimmed_rest);
                            return;
                        }
                        parsed_program = syntax_tree;
                    }
                    Err(err) => {
                        eprintln!("Error: {:?}", err);
                        return;
                    }
                }

                let analyzed_program;
                match analyzer::analyze_program(&mut variables, &parsed_program) {
                    Ok(analyzed_tree) => {
                        analyzed_program = analyzed_tree;
                        executor::execute_program(&mut variables, &analyzed_program)
                    }
                    Err(err) => {
                        eprintln!("Error: {:?}", err);
                        return;
                    }
                }
            }
        }
    }
}

fn input_command() -> String {
    let mut text = String::new();
    eprint!("> ");
    std::io::stdin()
        .read_line(&mut text)
        .expect("Cannot read line.");
    text
}
