mod ast;
mod symbol_table;
mod semantic_analyzer;
mod types;
mod parser; // New module for our manual parser
mod interpreter; // New module for the AST interpreter

use std::fs;

use semantic_analyzer::{SemanticAnalyzer};
use parser::{parse_program};
use interpreter::{Interpreter, InterpretResult}; // Use the new interpreter

// --- CLI Dependencies ---
use clap::{Parser as ClapParser, Subcommand};

#[derive(ClapParser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Interprets a .snask file directly from its AST
    Interpret { file: String },
}

// --- MAIN ---
fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Interpret { file } => {
            if !file.ends_with(".snask") {
                eprintln!("Erro: O arquivo de entrada deve ter a extensão '.snask'.");
                return;
            }
            match interpret_file(file) {
                Ok(_) => println!("Execução concluída."),
                Err(e) => eprintln!("Erro durante a execução: {}", e),
            }
        }
    }
}

fn interpret_file(file_path: &str) -> Result<(), String> {
    let source = fs::read_to_string(file_path)
        .map_err(|e| format!("Não foi possível ler o arquivo {}: {}", file_path, e))?;

    // 2. Parsing (Using our manual parser)
    let program: ast::Program = parse_program(&source)
        .map_err(|e| format!("Erro de parsing: {}", e))?;
    
    // 3. Análise Semântica
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program);

    if !analyzer.errors.is_empty() {
        let mut error_msg = String::from("Análise semântica encontrou erros:\n");
        for error in analyzer.errors {
            error_msg.push_str(&format!("{:?}\n", error));
        }
        return Err(error_msg);
    }

    // 4. Interpretação da AST
    let mut interpreter = Interpreter::new();
    match interpreter.interpret(program) {
        InterpretResult::Ok => Ok(()),
        InterpretResult::RuntimeError => Err(String::from("Erro de execução do interpretador.")),
    }
}
