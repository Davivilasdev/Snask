pub mod ast;
pub mod value;
pub mod symbol_table;
pub mod semantic_analyzer;
pub mod types;
pub mod parser;
pub mod interpreter;
pub mod modules;
pub mod stdlib;
pub mod span;
pub mod diagnostics;
pub mod repl;
pub mod packages; // Módulo para o gerenciador de pacotes


use std::fs;

use clap::{Parser as ClapParser, Subcommand};
use interpreter::{Interpreter, InterpretResult};
use parser::parse_program;
use semantic_analyzer::SemanticAnalyzer;
use stdlib::register_stdlib;

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
    /// Starts the interactive REPL
    Repl,
    /// Installs a package from the official Snask package registry
    Install { name: String },
}

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
        Commands::Repl => {
            let mut repl = repl::Repl::new();
            repl.run();
        }
        Commands::Install { name } => {
            if let Err(e) = packages::install_package(name) {
                eprintln!("Erro ao instalar pacote: {}", e);
            }
        }
    }
}

fn interpret_file(file_path: &str) -> Result<(), String> {
    let source = fs::read_to_string(file_path)
        .map_err(|e| format!("Não foi possível ler o arquivo {}: {}", file_path, e))?;

    let program = match parse_program(&source) {
        Ok(p) => p,
        Err(e) => {
            let diagnostic = convert_parser_error(&e);
            return Err(diagnostic.render(file_path, &source));
        }
    };

    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program);

    if !analyzer.errors.is_empty() {
        let mut error_msg = String::from("Análise semântica encontrou erros:\n");
        for error in analyzer.errors {
            error_msg.push_str(&format!("{:?}\n", error));
        }
        return Err(error_msg);
    }

    let mut interpreter = Interpreter::new();
    register_stdlib(interpreter.get_globals_mut());
    match interpreter.interpret(program) {
        InterpretResult::Ok => Ok(()),
        InterpretResult::RuntimeError(msg) => Err(format!("Erro de execução: {}", msg)),
    }
}

fn convert_parser_error(error_msg: &str) -> diagnostics::Diagnostic {
    use diagnostics::{Diagnostic, Annotation};
    use span::{Span, Position};

    if let Some(idx) = error_msg.rfind("na linha ") {
        let suffix = &error_msg[idx..];
        let parts: Vec<&str> = suffix.split_whitespace().collect();
        
        if parts.len() >= 4 {
            let line_str = parts[2].trim_matches(',');
            let col_str = parts[4];
            
            if let (Ok(line), Ok(col)) = (line_str.parse::<usize>(), col_str.parse::<usize>()) {
                let pos = Position::new(line, col, 0);
                let span = Span::new(pos, pos);
                
                let clean_msg = error_msg[..idx].trim().to_string();
                
                return Diagnostic::error(clean_msg)
                    .with_code("P001".to_string())
                    .with_annotation(Annotation::primary(span, "erro aqui".to_string()));
            }
        }
    }

    Diagnostic::error(error_msg.to_string())
}
