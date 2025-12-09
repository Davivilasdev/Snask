use crate::value::Value;
use std::collections::HashMap;
use bcrypt::{hash, verify, DEFAULT_COST};
use uuid::Uuid;
use std::time::{SystemTime, UNIX_EPOCH};
use lazy_static::lazy_static;
use std::sync::RwLock;

lazy_static! {
    pub static ref AUTH_USERS: RwLock<HashMap<String, AuthUser>> = RwLock::new(HashMap::new());
    pub static ref AUTH_SESSIONS: RwLock<HashMap<String, AuthSession>> = RwLock::new(HashMap::new());
}

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub id: String,
    pub username: String,
    pub email: String,
    pub hashed_password: String,
    pub created_at: u64,
    pub is_active: bool,
}

#[derive(Debug, Clone)]
pub struct AuthSession {
    pub id: String,
    pub user_id: String,
    pub created_at: u64,
    pub expires_at: u64,
    pub ip_address: Option<String>,
}

/// Módulo de Autenticação do Blaze
pub fn create_module() -> Value {
    let mut module = HashMap::new();

    // Registrar novo usuário
    module.insert(
        "registrar".to_string(),
        Value::NativeFunction(|args, _interpreter| {
            if args.len() < 3 {
                return Err("blaze.auth.registrar espera 3 argumentos: (username, email, password)".to_string());
            }

            let username = match &args[0] {
                Value::String(s) => s.clone(),
                _ => return Err("Username deve ser string".to_string()),
            };

            let email = match &args[1] {
                Value::String(s) => s.clone(),
                _ => return Err("Email deve ser string".to_string()),
            };

            let password = match &args[2] {
                Value::String(s) => s.clone(),
                _ => return Err("Password deve ser string".to_string()),
            };

            // Validações básicas
            if username.len() < 3 {
                return Ok(Value::Dict({
                    let mut err = HashMap::new();
                    err.insert(Value::String("sucesso".to_string()), Value::Boolean(false));
                    err.insert(Value::String("erro".to_string()), Value::String("Username deve ter pelo menos 3 caracteres".to_string()));
                    err
                }));
            }

            if password.len() < 6 {
                return Ok(Value::Dict({
                    let mut err = HashMap::new();
                    err.insert(Value::String("sucesso".to_string()), Value::Boolean(false));
                    err.insert(Value::String("erro".to_string()), Value::String("Senha deve ter pelo menos 6 caracteres".to_string()));
                    err
                }));
            }

            if !email.contains('@') {
                return Ok(Value::Dict({
                    let mut err = HashMap::new();
                    err.insert(Value::String("sucesso".to_string()), Value::Boolean(false));
                    err.insert(Value::String("erro".to_string()), Value::String("Email inválido".to_string()));
                    err
                }));
            }

            // Verificar se usuário já existe
            let users = AUTH_USERS.read().unwrap();
            if users.contains_key(&username) {
                return Ok(Value::Dict({
                    let mut err = HashMap::new();
                    err.insert(Value::String("sucesso".to_string()), Value::Boolean(false));
                    err.insert(Value::String("erro".to_string()), Value::String("Username já existe".to_string()));
                    err
                }));
            }

            // Verificar se email já existe
            if users.values().any(|u| u.email == email) {
                return Ok(Value::Dict({
                    let mut err = HashMap::new();
                    err.insert(Value::String("sucesso".to_string()), Value::Boolean(false));
                    err.insert(Value::String("erro".to_string()), Value::String("Email já cadastrado".to_string()));
                    err
                }));
            }
            drop(users);

            // Hash da senha
            let hashed_password = match hash(password, DEFAULT_COST) {
                Ok(h) => h,
                Err(_) => return Err("Erro ao processar senha".to_string()),
            };

            let user = AuthUser {
                id: Uuid::new_v4().to_string(),
                username: username.clone(),
                email,
                hashed_password,
                created_at: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                is_active: true,
            };

            AUTH_USERS.write().unwrap().insert(username.clone(), user.clone());

            let mut result = HashMap::new();
            result.insert(Value::String("sucesso".to_string()), Value::Boolean(true));
            result.insert(Value::String("user_id".to_string()), Value::String(user.id));
            result.insert(Value::String("username".to_string()), Value::String(username));
            Ok(Value::Dict(result))
        }),
    );

    // Login de usuário
    module.insert(
        "login".to_string(),
        Value::NativeFunction(|args, _interpreter| {
            if args.len() < 2 {
                return Err("blaze.auth.login espera 2 argumentos: (username, password)".to_string());
            }

            let username = match &args[0] {
                Value::String(s) => s.clone(),
                _ => return Err("Username deve ser string".to_string()),
            };

            let password = match &args[1] {
                Value::String(s) => s.clone(),
                _ => return Err("Password deve ser string".to_string()),
            };

            let users = AUTH_USERS.read().unwrap();
            if let Some(user) = users.get(&username) {
                if !user.is_active {
                    return Ok(Value::Dict({
                        let mut err = HashMap::new();
                        err.insert(Value::String("sucesso".to_string()), Value::Boolean(false));
                        err.insert(Value::String("erro".to_string()), Value::String("Conta desativada".to_string()));
                        err
                    }));
                }

                if verify(password, &user.hashed_password).unwrap_or(false) {
                    // Criar sessão
                    let session_id = Uuid::new_v4().to_string();
                    let now = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs();

                    let session = AuthSession {
                        id: session_id.clone(),
                        user_id: user.id.clone(),
                        created_at: now,
                        expires_at: now + (3600 * 24 * 7), // 7 dias
                        ip_address: None,
                    };

                    AUTH_SESSIONS.write().unwrap().insert(session_id.clone(), session);

                    let mut result = HashMap::new();
                    result.insert(Value::String("sucesso".to_string()), Value::Boolean(true));
                    result.insert(Value::String("session_id".to_string()), Value::String(session_id));
                    result.insert(Value::String("user_id".to_string()), Value::String(user.id.clone()));
                    result.insert(Value::String("username".to_string()), Value::String(username));
                    return Ok(Value::Dict(result));
                }
            }

            let mut err = HashMap::new();
            err.insert(Value::String("sucesso".to_string()), Value::Boolean(false));
            err.insert(Value::String("erro".to_string()), Value::String("Credenciais inválidas".to_string()));
            Ok(Value::Dict(err))
        }),
    );

    // Logout
    module.insert(
        "logout".to_string(),
        Value::NativeFunction(|args, _interpreter| {
            if args.len() < 1 {
                return Err("blaze.auth.logout espera 1 argumento: (session_id)".to_string());
            }

            let session_id = match &args[0] {
                Value::String(s) => s.clone(),
                _ => return Err("Session ID deve ser string".to_string()),
            };

            let removed = AUTH_SESSIONS.write().unwrap().remove(&session_id).is_some();

            let mut result = HashMap::new();
            result.insert(Value::String("sucesso".to_string()), Value::Boolean(removed));
            Ok(Value::Dict(result))
        }),
    );

    // Verificar se sessão é válida
    module.insert(
        "verificar_sessao".to_string(),
        Value::NativeFunction(|args, _interpreter| {
            if args.len() < 1 {
                return Err("blaze.auth.verificar_sessao espera 1 argumento: (session_id)".to_string());
            }

            let session_id = match &args[0] {
                Value::String(s) => s.clone(),
                _ => return Err("Session ID deve ser string".to_string()),
            };

            let mut sessions = AUTH_SESSIONS.write().unwrap();
            if let Some(session) = sessions.get(&session_id) {
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();

                if now < session.expires_at {
                    // Sessão válida
                    let users = AUTH_USERS.read().unwrap();
                    if let Some(user) = users.values().find(|u| u.id == session.user_id) {
                        let mut result = HashMap::new();
                        result.insert(Value::String("valido".to_string()), Value::Boolean(true));
                        result.insert(Value::String("user_id".to_string()), Value::String(user.id.clone()));
                        result.insert(Value::String("username".to_string()), Value::String(user.username.clone()));
                        result.insert(Value::String("email".to_string()), Value::String(user.email.clone()));
                        return Ok(Value::Dict(result));
                    }
                } else {
                    // Sessão expirada
                    sessions.remove(&session_id);
                }
            }

            let mut result = HashMap::new();
            result.insert(Value::String("valido".to_string()), Value::Boolean(false));
            Ok(Value::Dict(result))
        }),
    );

    // Obter usuário atual
    module.insert(
        "usuario_atual".to_string(),
        Value::NativeFunction(|args, _interpreter| {
            if args.len() < 1 {
                return Err("blaze.auth.usuario_atual espera 1 argumento: (session_id)".to_string());
            }

            let session_id = match &args[0] {
                Value::String(s) => s.clone(),
                _ => return Err("Session ID deve ser string".to_string()),
            };

            let sessions = AUTH_SESSIONS.read().unwrap();
            if let Some(session) = sessions.get(&session_id) {
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();

                if now < session.expires_at {
                    let users = AUTH_USERS.read().unwrap();
                    if let Some(user) = users.values().find(|u| u.id == session.user_id) {
                        let mut result = HashMap::new();
                        result.insert(Value::String("id".to_string()), Value::String(user.id.clone()));
                        result.insert(Value::String("username".to_string()), Value::String(user.username.clone()));
                        result.insert(Value::String("email".to_string()), Value::String(user.email.clone()));
                        result.insert(Value::String("ativo".to_string()), Value::Boolean(user.is_active));
                        return Ok(Value::Dict(result));
                    }
                }
            }

            Ok(Value::Nil)
        }),
    );

    // Alterar senha
    module.insert(
        "alterar_senha".to_string(),
        Value::NativeFunction(|args, _interpreter| {
            if args.len() < 3 {
                return Err("blaze.auth.alterar_senha espera 3 argumentos: (username, senha_antiga, senha_nova)".to_string());
            }

            let username = match &args[0] {
                Value::String(s) => s.clone(),
                _ => return Err("Username deve ser string".to_string()),
            };

            let old_password = match &args[1] {
                Value::String(s) => s.clone(),
                _ => return Err("Senha antiga deve ser string".to_string()),
            };

            let new_password = match &args[2] {
                Value::String(s) => s.clone(),
                _ => return Err("Senha nova deve ser string".to_string()),
            };

            if new_password.len() < 6 {
                return Ok(Value::Dict({
                    let mut err = HashMap::new();
                    err.insert(Value::String("sucesso".to_string()), Value::Boolean(false));
                    err.insert(Value::String("erro".to_string()), Value::String("Nova senha deve ter pelo menos 6 caracteres".to_string()));
                    err
                }));
            }

            let mut users = AUTH_USERS.write().unwrap();
            if let Some(user) = users.get_mut(&username) {
                if verify(old_password, &user.hashed_password).unwrap_or(false) {
                    let new_hash = match hash(new_password, DEFAULT_COST) {
                        Ok(h) => h,
                        Err(_) => return Err("Erro ao processar nova senha".to_string()),
                    };

                    user.hashed_password = new_hash;

                    let mut result = HashMap::new();
                    result.insert(Value::String("sucesso".to_string()), Value::Boolean(true));
                    return Ok(Value::Dict(result));
                }
            }

            let mut err = HashMap::new();
            err.insert(Value::String("sucesso".to_string()), Value::Boolean(false));
            err.insert(Value::String("erro".to_string()), Value::String("Credenciais inválidas".to_string()));
            Ok(Value::Dict(err))
        }),
    );

    // Listar todos os usuários (admin)
    module.insert(
        "listar_usuarios".to_string(),
        Value::NativeFunction(|_args, _interpreter| {
            let users = AUTH_USERS.read().unwrap();
            let user_list: Vec<Value> = users
                .values()
                .map(|user| {
                    let mut u = HashMap::new();
                    u.insert(Value::String("id".to_string()), Value::String(user.id.clone()));
                    u.insert(Value::String("username".to_string()), Value::String(user.username.clone()));
                    u.insert(Value::String("email".to_string()), Value::String(user.email.clone()));
                    u.insert(Value::String("ativo".to_string()), Value::Boolean(user.is_active));
                    Value::Dict(u)
                })
                .collect();

            Ok(Value::List(user_list))
        }),
    );

    // Desativar usuário
    module.insert(
        "desativar_usuario".to_string(),
        Value::NativeFunction(|args, _interpreter| {
            if args.len() < 1 {
                return Err("blaze.auth.desativar_usuario espera 1 argumento: (username)".to_string());
            }

            let username = match &args[0] {
                Value::String(s) => s.clone(),
                _ => return Err("Username deve ser string".to_string()),
            };

            let mut users = AUTH_USERS.write().unwrap();
            if let Some(user) = users.get_mut(&username) {
                user.is_active = false;

                let mut result = HashMap::new();
                result.insert(Value::String("sucesso".to_string()), Value::Boolean(true));
                return Ok(Value::Dict(result));
            }

            let mut err = HashMap::new();
            err.insert(Value::String("sucesso".to_string()), Value::Boolean(false));
            err.insert(Value::String("erro".to_string()), Value::String("Usuário não encontrado".to_string()));
            Ok(Value::Dict(err))
        }),
    );

    let dict_map = module
        .into_iter()
        .map(|(k, v)| (Value::String(k), v))
        .collect();

    Value::Dict(dict_map)
}
