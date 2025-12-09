use crate::value::Value;
use crate::symbol_table::SymbolTable;

pub mod string;
pub mod math;
pub mod blaze;
pub mod blaze_auth;
pub mod collections;
pub mod blaze_db; // Adicione esta linha

/// Registra todas as funções da biblioteca padrão
pub fn register_stdlib(globals: &mut SymbolTable) {
    // Registra o módulo string (necessário para interpolação)
    globals.define("string".to_string(), string::create_module(), false, false);

    // Registra a função 'format' globalmente para compatibilidade com interpolação de strings
    globals.define("format".to_string(), string::get_global_format_function(), false, false);
    
    // Registra outros módulos
    globals.define("math".to_string(), math::create_module(), false, false);
    globals.define("blaze".to_string(), blaze::create_module(), false, false);
    globals.define("blaze_auth".to_string(), blaze_auth::create_module(), false, false);
    globals.define("collections".to_string(), collections::create_module(), false, false);
    globals.define("blaze_db".to_string(), blaze_db::create_module(), false, false); // Adicione esta linha
}