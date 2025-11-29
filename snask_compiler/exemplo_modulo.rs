// Exemplo de módulo Snask compatível com instalação automática
// Este arquivo demonstra a estrutura correta para módulos do SnaskPackages

use crate::value::Value;
use std::collections::HashMap;

/// Função principal de registro do módulo - OBRIGATÓRIA
/// Esta função cria e retorna o módulo com todas as suas funções
pub fn create_module() -> Value {
    let mut module = HashMap::new();
    
    // Registrar funções individuais no módulo
    module.insert("exemplo_funcao".to_string(), Value::NativeFunction(exemplo_funcao));
    module.insert("saudacao".to_string(), Value::NativeFunction(saudacao));
    module.insert("calcular".to_string(), Value::NativeFunction(calcular));
    
    // Retornar o módulo como um objeto Value
    Value::Object(module)
}

/// Função de exemplo que retorna uma string formatada
fn exemplo_funcao(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("exemplo_funcao() espera 1 argumento".to_string());
    }
    
    match &args[0] {
        Value::String(s) => Ok(Value::String(format!("Você disse: {}", s))),
        _ => Err("exemplo_funcao() espera uma string".to_string()),
    }
}

/// Função de saudação personalizada
fn saudacao(args: Vec<Value>) -> Result<Value, String> {
    if args.is_empty() {
        return Ok(Value::String("Olá, mundo!".to_string()));
    }
    
    match &args[0] {
        Value::String(nome) => Ok(Value::String(format!("Olá, {}!", nome))),
        _ => Err("saudacao() espera uma string como nome".to_string()),
    }
}

/// Função de cálculo matemático
fn calcular(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("calcular() espera 2 argumentos numéricos".to_string());
    }
    
    match (&args[0], &args[1]) {
        (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
        _ => Err("calcular() espera dois números".to_string()),
    }
}
