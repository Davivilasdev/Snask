use std::path::PathBuf;
use std::fs;
use url::Url;
use directories::ProjectDirs;

const SNASK_PACKAGES_BASE_URL: &str = "https://raw.githubusercontent.com/Davivilasdev/SnaskPackages/main/";

pub fn get_user_packages_dir() -> Option<PathBuf> {
    if let Some(proj_dirs) = ProjectDirs::from("com", "Snask", "Snask") {
        let dir = proj_dirs.data_dir().join("packages");
        if !dir.exists() {
            let _ = fs::create_dir_all(&dir);
        }
        Some(dir)
    } else {
        None
    }
}

pub fn install_package(name_or_url: &str) -> Result<(), String> {
    // Verifica se é uma URL (repositório Git)
    if Url::parse(name_or_url).is_ok() && (name_or_url.starts_with("http") || name_or_url.starts_with("git")) {
        install_git_repository(name_or_url)
    } else {
        // Assume que é o nome de um pacote .snask para baixar do GitHub
        install_snask_package(name_or_url)
    }
}

fn install_git_repository(url: &str) -> Result<(), String> {
    let packages_dir = get_user_packages_dir()
        .ok_or_else(|| "Não foi possível determinar o diretório de pacotes.".to_string())?;
    
    let repo_name = url.split('/').last()
        .and_then(|n| n.strip_suffix(".git"))
        .ok_or("URL de repositório inválida.")?;
    
    let target_dir = packages_dir.join(repo_name);
    
    if target_dir.exists() {
        println!("Projeto '{}' já existe. Pulando a instalação.", repo_name);
        return Ok(());
    }
    
    println!("Clonando projeto '{}' em {}...", url, target_dir.display());
    git2::Repository::clone(url, &target_dir)
        .map_err(|e| format!("Falha ao clonar: {}", e))?;
    println!("Projeto '{}' clonado com sucesso.", repo_name);
    Ok(())
}

fn install_snask_package(name: &str) -> Result<(), String> {
    let package_filename = format!("{}.rs", name);
    let package_url = format!("{}{}", SNASK_PACKAGES_BASE_URL, package_filename);
    
    println!("📦 Baixando módulo Rust '{}' de {}...", name, package_url);
    
    // Fazer requisição HTTP
    let response = reqwest::blocking::get(&package_url)
        .map_err(|e| format!("Falha ao fazer requisição para {}: {}", package_url, e))?;
    
    if !response.status().is_success() {
        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Err(format!("Módulo '{}' não encontrado no registro oficial.", name));
        }
        return Err(format!("Erro ao baixar módulo '{}': HTTP {}", name, response.status()));
    }
    
    let content = response.text()
        .map_err(|e| format!("Falha ao ler conteúdo do módulo: {}", e))?;
    
    // Determinar o diretório src/stdlib/ do projeto
    let current_dir = std::env::current_dir()
        .map_err(|e| format!("Não foi possível obter diretório atual: {}", e))?;
    
    let stdlib_dir = current_dir.join("src").join("stdlib");
    
    if !stdlib_dir.exists() {
        return Err(format!("Diretório src/stdlib não encontrado. Execute este comando da raiz do projeto Snask."));
    }
    
    let dest_path = stdlib_dir.join(&package_filename);
    let is_new_module = !dest_path.exists();
    
    if !is_new_module {
        println!("⚠️  Módulo '{}' já existe em src/stdlib/", name);
        print!("Deseja sobrescrever? (s/N): ");
        use std::io::{self, Write};
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        if !input.trim().eq_ignore_ascii_case("s") {
            println!("Instalação cancelada.");
            return Ok(());
        }
    }
    
    // Salvar o módulo
    fs::write(&dest_path, content)
        .map_err(|e| format!("Falha ao salvar módulo em {}: {}", dest_path.display(), e))?;
    
    println!("✓ Módulo '{}' baixado para {}", name, dest_path.display());
    
    // Se for um módulo novo, integrar automaticamente
    if is_new_module {
        println!("\n🔧 Integrando módulo automaticamente...");
        
        // 1. Adicionar declaração do módulo em src/stdlib.rs
        integrate_module_declaration(name)?;
        
        // 2. Adicionar registro na função register_stdlib()
        integrate_module_registration(name)?;
        
        println!("✓ Módulo integrado em src/stdlib.rs");
        
        // 3. Recompilar o projeto
        println!("\n🔨 Recompilando Snask...");
        recompile_snask()?;
        
        println!("\n✅ INSTALAÇÃO COMPLETA!");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("O módulo '{}' está pronto para uso!", name);
        println!("As funções do módulo estão disponíveis globalmente em seus programas Snask.");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    } else {
        println!("\n⚠️  Módulo sobrescrito. Você pode precisar recompilar:");
        println!("    cargo build --release\n");
    }
    
    Ok(())
}

/// Adiciona a declaração `pub mod <nome>;` em src/stdlib.rs
fn integrate_module_declaration(module_name: &str) -> Result<(), String> {
    let current_dir = std::env::current_dir()
        .map_err(|e| format!("Não foi possível obter diretório atual: {}", e))?;
    
    let stdlib_rs_path = current_dir.join("src").join("stdlib.rs");
    
    let content = fs::read_to_string(&stdlib_rs_path)
        .map_err(|e| format!("Falha ao ler src/stdlib.rs: {}", e))?;
    
    // Verificar se já existe a declaração
    let declaration = format!("pub mod {};", module_name);
    if content.contains(&declaration) {
        return Ok(()); // Já existe
    }
    
    // Encontrar a última linha de declaração de módulo
    let lines: Vec<&str> = content.lines().collect();
    let mut insert_index = 0;
    
    for (i, line) in lines.iter().enumerate() {
        if line.trim().starts_with("pub mod ") && line.trim().ends_with(";") {
            insert_index = i + 1;
        }
    }
    
    // Inserir a nova declaração
    let mut new_lines = lines.clone();
    new_lines.insert(insert_index, &declaration);
    
    let new_content = new_lines.join("\n");
    fs::write(&stdlib_rs_path, new_content)
        .map_err(|e| format!("Falha ao escrever src/stdlib.rs: {}", e))?;
    
    Ok(())
}

/// Adiciona a chamada `<nome>::register(globals);` na função register_stdlib()
fn integrate_module_registration(module_name: &str) -> Result<(), String> {
    let current_dir = std::env::current_dir()
        .map_err(|e| format!("Não foi possível obter diretório atual: {}", e))?;
    
    let stdlib_rs_path = current_dir.join("src").join("stdlib.rs");
    
    let content = fs::read_to_string(&stdlib_rs_path)
        .map_err(|e| format!("Falha ao ler src/stdlib.rs: {}", e))?;
    
    // Verificar se já existe o registro
    let registration = format!("globals.define(\"{}\".to_string(), {}::create_module(), false, false);", module_name, module_name);
    if content.contains(&format!("{}::create_module()", module_name)) {
        return Ok(()); // Já existe
    }
    
    // Encontrar a função register_stdlib e adicionar o registro
    let lines: Vec<&str> = content.lines().collect();
    let mut new_lines = Vec::new();
    let mut in_register_function = false;
    let mut last_define_index = 0;
    
    for (i, line) in lines.iter().enumerate() {
        new_lines.push(line.to_string());
        
        if line.contains("pub fn register_stdlib") {
            in_register_function = true;
        }
        
        if in_register_function && line.trim().starts_with("globals.define(") {
            last_define_index = new_lines.len();
        }
    }
    
    // Inserir o novo registro após o último globals.define
    if last_define_index > 0 {
        new_lines.insert(last_define_index, format!("    {}", registration));
    }
    
    let new_content = new_lines.join("\n");
    fs::write(&stdlib_rs_path, new_content)
        .map_err(|e| format!("Falha ao escrever src/stdlib.rs: {}", e))?;
    
    Ok(())
}

/// Recompila o projeto Snask
fn recompile_snask() -> Result<(), String> {
    use std::process::Command;
    
    let output = Command::new("cargo")
        .args(&["build", "--release"])
        .output()
        .map_err(|e| format!("Falha ao executar cargo build: {}", e))?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Falha na compilação:\n{}", stderr));
    }
    
    println!("✓ Compilação concluída com sucesso!");
    Ok(())
}