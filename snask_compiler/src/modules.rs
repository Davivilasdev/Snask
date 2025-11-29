use crate::parser::parse_program;
use crate::ast::Program;
use std::fs;
use std::path::{Path, PathBuf};
use crate::packages; // Importa o novo módulo de pacotes

pub fn load_module(path_str: &str) -> Result<Program, String> {
    // Tenta resolver o módulo de diferentes maneiras.
    // 1. Como um pacote instalado.
    // 2. Como um caminho de arquivo relativo.

    let mut tried_paths: Vec<String> = Vec::new();

    // --- TENTATIVA 1: Resolver como um pacote ---
    if let Some(packages_dir) = packages::get_user_packages_dir() {
        let module_path = Path::new(path_str);
        
        // Caminho 1: <packages_dir>/<module>/main.snask
        let mut path_candidate = packages_dir.join(module_path);
        path_candidate.push("main.snask");
        
        if path_candidate.exists() {
            return read_and_parse_module(&path_candidate);
        }
        tried_paths.push(path_candidate.to_string_lossy().into_owned());

        // Caminho 2: <packages_dir>/<module>.snask
        let mut path_candidate = packages_dir.join(module_path);
        if !path_candidate.to_string_lossy().ends_with(".snask") {
            path_candidate.set_extension("snask");
        }

        if path_candidate.exists() {
            return read_and_parse_module(&path_candidate);
        }
        tried_paths.push(path_candidate.to_string_lossy().into_owned());
    }

    // --- TENTATIVA 2: Resolver como caminho relativo ---
    let relative_path = Path::new(path_str);
    if relative_path.exists() {
        return read_and_parse_module(relative_path);
    }
    tried_paths.push(relative_path.to_string_lossy().into_owned());


    // Se todas as tentativas falharem
    Err(format!(
        "Não foi possível encontrar o módulo '{}'. Caminhos tentados:\n - {}",
        path_str,
        tried_paths.join("\n - ")
    ))
}

/// Função auxiliar para ler e parsear o código de um módulo
fn read_and_parse_module(path: &Path) -> Result<Program, String> {
    let source = fs::read_to_string(path)
        .map_err(|e| format!("Não foi possível ler o módulo {}: {}", path.display(), e))?;
    
    parse_program(&source)
        .map_err(|e| format!("Erro de parsing no módulo {}: {}", path.display(), e))
}
