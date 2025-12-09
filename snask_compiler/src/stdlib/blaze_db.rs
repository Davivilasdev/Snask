use crate::value::Value;
use std::collections::HashMap;
use rusqlite::{Connection, params, Result as SqlResult};
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;

lazy_static! {
    static ref DB_CONNECTION: Arc<Mutex<Option<Connection>>> = Arc::new(Mutex::new(None));
}

/// Módulo Blaze_DB - ORM simples para SQLite
pub fn create_module() -> Value {
    let mut module = HashMap::new();

    // Conectar ao banco de dados
    module.insert(
        "conectar".to_string(),
        Value::NativeFunction(|args, _interpreter| {
            if args.is_empty() {
                return Err("blaze_db.conectar espera 1 argumento: (caminho_db)".to_string());
            }

            let db_path = match &args[0] {
                Value::String(s) => s.clone(),
                _ => return Err("Caminho do banco deve ser string".to_string()),
            };

            match Connection::open(&db_path) {
                Ok(conn) => {
                    *DB_CONNECTION.lock().unwrap() = Some(conn);
                    let mut result = HashMap::new();
                    result.insert(Value::String("sucesso".to_string()), Value::Boolean(true));
                    result.insert(Value::String("mensagem".to_string()), 
                        Value::String(format!("Conectado a {}", db_path)));
                    Ok(Value::Dict(result))
                }
                Err(e) => {
                    let mut result = HashMap::new();
                    result.insert(Value::String("sucesso".to_string()), Value::Boolean(false));
                    result.insert(Value::String("erro".to_string()), 
                        Value::String(format!("Erro ao conectar: {}", e)));
                    Ok(Value::Dict(result))
                }
            }
        }),
    );

    // Executar SQL direto
    module.insert(
        "executar".to_string(),
        Value::NativeFunction(|args, _interpreter| {
            if args.is_empty() {
                return Err("blaze_db.executar espera 1 argumento: (sql)".to_string());
            }

            let sql = match &args[0] {
                Value::String(s) => s.clone(),
                _ => return Err("SQL deve ser string".to_string()),
            };

            let conn_guard = DB_CONNECTION.lock().unwrap();
            if let Some(ref conn) = *conn_guard {
                match conn.execute(&sql, []) {
                    Ok(rows_affected) => {
                        let mut result = HashMap::new();
                        result.insert(Value::String("sucesso".to_string()), Value::Boolean(true));
                        result.insert(Value::String("linhas_afetadas".to_string()), 
                            Value::Number(rows_affected as f64));
                        Ok(Value::Dict(result))
                    }
                    Err(e) => {
                        let mut result = HashMap::new();
                        result.insert(Value::String("sucesso".to_string()), Value::Boolean(false));
                        result.insert(Value::String("erro".to_string()), 
                            Value::String(format!("Erro SQL: {}", e)));
                        Ok(Value::Dict(result))
                    }
                }
            } else {
                Err("Banco de dados não conectado. Use blaze_db.conectar() primeiro.".to_string())
            }
        }),
    );

    // Criar tabela
    module.insert(
        "criar_tabela".to_string(),
        Value::NativeFunction(|args, _interpreter| {
            if args.len() < 2 {
                return Err("blaze_db.criar_tabela espera 2 argumentos: (nome, colunas)".to_string());
            }

            let table_name = match &args[0] {
                Value::String(s) => s.clone(),
                _ => return Err("Nome da tabela deve ser string".to_string()),
            };

            let columns = match &args[1] {
                Value::Dict(d) => d,
                _ => return Err("Colunas devem ser um dicionário".to_string()),
            };

            let mut column_defs = Vec::new();
            for (key, value) in columns {
                if let (Value::String(col_name), Value::String(col_type)) = (key, value) {
                    column_defs.push(format!("{} {}", col_name, col_type));
                }
            }

            let sql = format!(
                "CREATE TABLE IF NOT EXISTS {} ({})",
                table_name,
                column_defs.join(", ")
            );

            let conn_guard = DB_CONNECTION.lock().unwrap();
            if let Some(ref conn) = *conn_guard {
                match conn.execute(&sql, []) {
                    Ok(_) => {
                        let mut result = HashMap::new();
                        result.insert(Value::String("sucesso".to_string()), Value::Boolean(true));
                        result.insert(Value::String("mensagem".to_string()), 
                            Value::String(format!("Tabela '{}' criada", table_name)));
                        Ok(Value::Dict(result))
                    }
                    Err(e) => {
                        let mut result = HashMap::new();
                        result.insert(Value::String("sucesso".to_string()), Value::Boolean(false));
                        result.insert(Value::String("erro".to_string()), 
                            Value::String(format!("Erro ao criar tabela: {}", e)));
                        Ok(Value::Dict(result))
                    }
                }
            } else {
                Err("Banco de dados não conectado".to_string())
            }
        }),
    );

    // Inserir dados
    module.insert(
        "inserir".to_string(),
        Value::NativeFunction(|args, _interpreter| {
            if args.len() < 2 {
                return Err("blaze_db.inserir espera 2 argumentos: (tabela, dados)".to_string());
            }

            let table_name = match &args[0] {
                Value::String(s) => s.clone(),
                _ => return Err("Nome da tabela deve ser string".to_string()),
            };

            let data = match &args[1] {
                Value::Dict(d) => d,
                _ => return Err("Dados devem ser um dicionário".to_string()),
            };

            let mut columns = Vec::new();
            let mut placeholders = Vec::new();
            let mut values: Vec<String> = Vec::new();

            for (key, value) in data {
                if let Value::String(col_name) = key {
                    columns.push(col_name.clone());
                    placeholders.push("?".to_string());
                    
                    let val_str = match value {
                        Value::String(s) => s.clone(),
                        Value::Number(n) => n.to_string(),
                        Value::Boolean(b) => if *b { "1" } else { "0" }.to_string(),
                        _ => value.to_string(),
                    };
                    values.push(val_str);
                }
            }

            let sql = format!(
                "INSERT INTO {} ({}) VALUES ({})",
                table_name,
                columns.join(", "),
                placeholders.join(", ")
            );

            let conn_guard = DB_CONNECTION.lock().unwrap();
            if let Some(ref conn) = *conn_guard {
                let params_refs: Vec<&dyn rusqlite::ToSql> = values.iter()
                    .map(|v| v as &dyn rusqlite::ToSql)
                    .collect();

                match conn.execute(&sql, params_refs.as_slice()) {
                    Ok(rows) => {
                        let last_id = conn.last_insert_rowid();
                        let mut result = HashMap::new();
                        result.insert(Value::String("sucesso".to_string()), Value::Boolean(true));
                        result.insert(Value::String("id".to_string()), Value::Number(last_id as f64));
                        result.insert(Value::String("linhas_afetadas".to_string()), Value::Number(rows as f64));
                        Ok(Value::Dict(result))
                    }
                    Err(e) => {
                        let mut result = HashMap::new();
                        result.insert(Value::String("sucesso".to_string()), Value::Boolean(false));
                        result.insert(Value::String("erro".to_string()), 
                            Value::String(format!("Erro ao inserir: {}", e)));
                        Ok(Value::Dict(result))
                    }
                }
            } else {
                Err("Banco de dados não conectado".to_string())
            }
        }),
    );

    // Consultar dados
    module.insert(
        "consultar".to_string(),
        Value::NativeFunction(|args, _interpreter| {
            if args.is_empty() {
                return Err("blaze_db.consultar espera 1-2 argumentos: (tabela, [condicao])".to_string());
            }

            let table_name = match &args[0] {
                Value::String(s) => s.clone(),
                _ => return Err("Nome da tabela deve ser string".to_string()),
            };

            let condition = if args.len() > 1 {
                match &args[1] {
                    Value::String(s) => format!(" WHERE {}", s),
                    _ => String::new(),
                }
            } else {
                String::new()
            };

            let sql = format!("SELECT * FROM {}{}", table_name, condition);

            let conn_guard = DB_CONNECTION.lock().unwrap();
            if let Some(ref conn) = *conn_guard {
                match conn.prepare(&sql) {
                    Ok(mut stmt) => {
                        let column_count = stmt.column_count();
                        let column_names: Vec<String> = (0..column_count)
                            .map(|i| stmt.column_name(i).unwrap_or("").to_string())
                            .collect();

                        match stmt.query_map([], |row| {
                            let mut row_dict = HashMap::new();
                            for (i, col_name) in column_names.iter().enumerate() {
                                let value: Result<String, _> = row.get(i);
                                let val = match value {
                                    Ok(s) => Value::String(s),
                                    Err(_) => {
                                        // Tentar como número
                                        match row.get::<_, f64>(i) {
                                            Ok(n) => Value::Number(n),
                                            Err(_) => Value::Nil,
                                        }
                                    }
                                };
                                row_dict.insert(Value::String(col_name.clone()), val);
                            }
                            Ok(Value::Dict(row_dict))
                        }) {
                            Ok(rows) => {
                                let results: Vec<Value> = rows.filter_map(|r| r.ok()).collect();
                                Ok(Value::List(results))
                            }
                            Err(e) => {
                                let mut result = HashMap::new();
                                result.insert(Value::String("sucesso".to_string()), Value::Boolean(false));
                                result.insert(Value::String("erro".to_string()), 
                                    Value::String(format!("Erro na consulta: {}", e)));
                                Ok(Value::Dict(result))
                            }
                        }
                    }
                    Err(e) => {
                        let mut result = HashMap::new();
                        result.insert(Value::String("sucesso".to_string()), Value::Boolean(false));
                        result.insert(Value::String("erro".to_string()), 
                            Value::String(format!("Erro ao preparar consulta: {}", e)));
                        Ok(Value::Dict(result))
                    }
                }
            } else {
                Err("Banco de dados não conectado".to_string())
            }
        }),
    );

    // Atualizar dados
    module.insert(
        "atualizar".to_string(),
        Value::NativeFunction(|args, _interpreter| {
            if args.len() < 3 {
                return Err("blaze_db.atualizar espera 3 argumentos: (tabela, dados, condicao)".to_string());
            }

            let table_name = match &args[0] {
                Value::String(s) => s.clone(),
                _ => return Err("Nome da tabela deve ser string".to_string()),
            };

            let data = match &args[1] {
                Value::Dict(d) => d,
                _ => return Err("Dados devem ser um dicionário".to_string()),
            };

            let condition = match &args[2] {
                Value::String(s) => s.clone(),
                _ => return Err("Condição deve ser string".to_string()),
            };

            let mut set_clauses = Vec::new();
            let mut values: Vec<String> = Vec::new();

            for (key, value) in data {
                if let Value::String(col_name) = key {
                    set_clauses.push(format!("{} = ?", col_name));
                    
                    let val_str = match value {
                        Value::String(s) => s.clone(),
                        Value::Number(n) => n.to_string(),
                        Value::Boolean(b) => if *b { "1" } else { "0" }.to_string(),
                        _ => value.to_string(),
                    };
                    values.push(val_str);
                }
            }

            let sql = format!(
                "UPDATE {} SET {} WHERE {}",
                table_name,
                set_clauses.join(", "),
                condition
            );

            let conn_guard = DB_CONNECTION.lock().unwrap();
            if let Some(ref conn) = *conn_guard {
                let params_refs: Vec<&dyn rusqlite::ToSql> = values.iter()
                    .map(|v| v as &dyn rusqlite::ToSql)
                    .collect();

                match conn.execute(&sql, params_refs.as_slice()) {
                    Ok(rows) => {
                        let mut result = HashMap::new();
                        result.insert(Value::String("sucesso".to_string()), Value::Boolean(true));
                        result.insert(Value::String("linhas_afetadas".to_string()), Value::Number(rows as f64));
                        Ok(Value::Dict(result))
                    }
                    Err(e) => {
                        let mut result = HashMap::new();
                        result.insert(Value::String("sucesso".to_string()), Value::Boolean(false));
                        result.insert(Value::String("erro".to_string()), 
                            Value::String(format!("Erro ao atualizar: {}", e)));
                        Ok(Value::Dict(result))
                    }
                }
            } else {
                Err("Banco de dados não conectado".to_string())
            }
        }),
    );

    // Deletar dados
    module.insert(
        "deletar".to_string(),
        Value::NativeFunction(|args, _interpreter| {
            if args.len() < 2 {
                return Err("blaze_db.deletar espera 2 argumentos: (tabela, condicao)".to_string());
            }

            let table_name = match &args[0] {
                Value::String(s) => s.clone(),
                _ => return Err("Nome da tabela deve ser string".to_string()),
            };

            let condition = match &args[1] {
                Value::String(s) => s.clone(),
                _ => return Err("Condição deve ser string".to_string()),
            };

            let sql = format!("DELETE FROM {} WHERE {}", table_name, condition);

            let conn_guard = DB_CONNECTION.lock().unwrap();
            if let Some(ref conn) = *conn_guard {
                match conn.execute(&sql, []) {
                    Ok(rows) => {
                        let mut result = HashMap::new();
                        result.insert(Value::String("sucesso".to_string()), Value::Boolean(true));
                        result.insert(Value::String("linhas_afetadas".to_string()), Value::Number(rows as f64));
                        Ok(Value::Dict(result))
                    }
                    Err(e) => {
                        let mut result = HashMap::new();
                        result.insert(Value::String("sucesso".to_string()), Value::Boolean(false));
                        result.insert(Value::String("erro".to_string()), 
                            Value::String(format!("Erro ao deletar: {}", e)));
                        Ok(Value::Dict(result))
                    }
                }
            } else {
                Err("Banco de dados não conectado".to_string())
            }
        }),
    );

    // Contar registros
    module.insert(
        "contar".to_string(),
        Value::NativeFunction(|args, _interpreter| {
            if args.is_empty() {
                return Err("blaze_db.contar espera 1-2 argumentos: (tabela, [condicao])".to_string());
            }

            let table_name = match &args[0] {
                Value::String(s) => s.clone(),
                _ => return Err("Nome da tabela deve ser string".to_string()),
            };

            let condition = if args.len() > 1 {
                match &args[1] {
                    Value::String(s) => format!(" WHERE {}", s),
                    _ => String::new(),
                }
            } else {
                String::new()
            };

            let sql = format!("SELECT COUNT(*) FROM {}{}", table_name, condition);

            let conn_guard = DB_CONNECTION.lock().unwrap();
            if let Some(ref conn) = *conn_guard {
                match conn.query_row(&sql, [], |row| row.get::<_, i64>(0)) {
                    Ok(count) => Ok(Value::Number(count as f64)),
                    Err(e) => {
                        let mut result = HashMap::new();
                        result.insert(Value::String("sucesso".to_string()), Value::Boolean(false));
                        result.insert(Value::String("erro".to_string()), 
                            Value::String(format!("Erro ao contar: {}", e)));
                        Ok(Value::Dict(result))
                    }
                }
            } else {
                Err("Banco de dados não conectado".to_string())
            }
        }),
    );

    let dict_map = module
        .into_iter()
        .map(|(k, v)| (Value::String(k), v))
        .collect();

    Value::Dict(dict_map)
}
