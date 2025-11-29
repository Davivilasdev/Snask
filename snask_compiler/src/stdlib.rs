use crate::value::Value;
use crate::symbol_table::SymbolTable;

pub mod string;
pub mod math;

/// Registra todas as funções da biblioteca padrão
pub fn register_stdlib(globals: &mut SymbolTable) {
    // Registra o módulo string (necessário para interpolação)
    globals.define("string".to_string(), string::create_module(), false, false);

    // Registra a função 'format' globalmente para compatibilidade com interpolação de strings
    globals.define("format".to_string(), string::get_global_format_function(), false, false);
    globals.define("math".to_string(), math::create_module(), false, false);
}