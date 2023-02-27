mod analyzer;
mod parser;
mod symbol_table;
fn main() {
    let mut args = std::env::args();
    args.next();
    let source_path = args.next();

    if source_path.is_none() {
        return eprintln!("Need one argument : <source>.calc file");
    }
    proceess_file(&source_path.unwrap());
}

fn proceess_file(source_path: &str) {
    const CALC_PREFIX: &str = ".calc";

    if !source_path.ends_with(CALC_PREFIX) {
        return eprintln!(
            "Invalid argument {}, file must end with {}",
            source_path, CALC_PREFIX
        );
    }
    let source_code = std::fs::read_to_string(source_path);
    if source_code.is_err() {
        return eprintln!(
            "Error reading file {}\n {}",
            source_path,
            source_code.unwrap_err()
        );
    }
    let source_code = source_code.unwrap();
    let parsed_program;
    match parser::parse_program(&source_code) {
        Ok((rest, syntax_tree)) => {
            let trimmed_rest = rest.trim();
            if trimmed_rest.len() > 0 {
                eprintln!(
                    "Invalid remaining code in '{}': {}",
                    source_path, trimmed_rest
                );
                return;
            }
            parsed_program = syntax_tree;
        }
        Err(err) => {
            eprintln!("Invalid code in '{}': {:?}", source_path, err);
            return;
        }
    }

    let analyzed_program;
    let mut variables = symbol_table::SymbolTable::new();
    match analyzer::analyze_program(&mut variables, &parsed_program) {
        Ok(analyzed_tree) => {
            analyzed_program = analyzed_tree;
        }
        Err(err) => {
            eprintln!("Invalid code in '{}': {}", source_path, err);
            return;
        }
    }

    println!("Symbol table: {:#?}", variables);
    println!("Analyzed program: {:#?}", analyzed_program);
}
