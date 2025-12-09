use crate::value::Value;
use rouille::{Request, Response};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use chrono::Local;
use std::fs;
use lazy_static::lazy_static;

use bcrypt::{hash, verify, DEFAULT_COST};
use uuid::Uuid;

use std::time::{SystemTime, UNIX_EPOCH};

lazy_static! {
    pub static ref TEMPLATE_DIR: RwLock<Option<String>> = RwLock::new(None);
    pub static ref STATIC_DIR: RwLock<Option<String>> = RwLock::new(None);
    pub static ref USERS: RwLock<HashMap<String, User>> = RwLock::new(HashMap::new());
    pub static ref SESSIONS: RwLock<HashMap<String, Session>> = RwLock::new(HashMap::new());
}

#[derive(Debug, Clone)]
pub struct User {
    pub id: String,
    pub username: String,
    pub hashed_password: String,
}

#[derive(Debug, Clone)]
pub struct Session {
    pub id: String,
    pub user_id: String,
    pub expiry: u64,
}

/// Módulo Blaze - Framework Web para Snask v1.1
/// Alimentado por `rouille`.
pub fn create_module() -> Value {
    let mut module = HashMap::new();

    // Função de registro de usuário
    module.insert(
        "register".to_string(),
        Value::NativeFunction(|args, _interpreter| {
            if args.len() < 2 {
                return Err("blaze.register espera 2 argumentos: (username, password)".to_string());
            }

            let username = match &args[0] {
                Value::String(s) => s.clone(),
                _ => return Err("Primeiro argumento (username) deve ser uma string".to_string()),
            };

            let password = match &args[1] {
                Value::String(s) => s.clone(),
                _ => return Err("Segundo argumento (password) deve ser uma string".to_string()),
            };

            // Verificar se o usuário já existe
            if USERS.read().unwrap().contains_key(&username) {
                return Ok(Value::Boolean(false)); // Usuário já existe
            }

            // Hash da senha
            let hashed_password = match hash(password, DEFAULT_COST) {
                Ok(h) => h,
                Err(_) => return Err("Erro ao fazer hash da senha".to_string()),
            };

            let user = User {
                id: Uuid::new_v4().to_string(),
                username: username.clone(),
                hashed_password,
            };

            USERS.write().unwrap().insert(username, user);
            Ok(Value::Boolean(true))
        }),
    );

    // Função para converter qualquer valor para booleano
    module.insert(
        "boolean".to_string(),
        Value::NativeFunction(|args, _interpreter| {
            if args.len() != 1 {
                return Err("blaze.boolean espera 1 argumento".to_string());
            }
            Ok(Value::Boolean(args[0].is_truthy()))
        }),
    );

    module.insert(
        "create".to_string(),
        Value::NativeFunction(|_args, _interpreter| {
            let mut app = HashMap::new();
            app.insert(
                Value::String("routes".to_string()),
                Value::List(Vec::new()),
            );
            Ok(Value::Dict(app))
        }),
    );

    // Função de login de usuário
    module.insert(
        "login".to_string(),
        Value::NativeFunction(|args, _interpreter| {
            if args.len() < 2 {
                return Err("blaze.login espera 2 argumentos: (username, password)".to_string());
            }

            let username = match &args[0] {
                Value::String(s) => s.clone(),
                _ => return Err("Primeiro argumento (username) deve ser uma string".to_string()),
            };

            let password = match &args[1] {
                Value::String(s) => s.clone(),
                _ => return Err("Segundo argumento (password) deve ser uma string".to_string()),
            };

            let users = USERS.read().unwrap();
            if let Some(user) = users.get(&username) {
                if verify(password, &user.hashed_password).unwrap_or(false) {
                    // Login bem-sucedido, criar sessão
                    let session_id = Uuid::new_v4().to_string();
                    let expiry = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .expect("Time went backwards")
                        .as_secs()
                        + (3600 * 24); // Sessão válida por 24 horas

                    let session = Session {
                        id: session_id.clone(),
                        user_id: user.id.clone(),
                        expiry,
                    };

                    SESSIONS.write().unwrap().insert(session_id.clone(), session);
                    return Ok(Value::String(session_id)); // Retorna o session_id diretamente
                }
            }
            Ok(Value::Nil) // Login falhou
        }),
    );

    // Função de logout de usuário
    module.insert(
        "logout".to_string(),
        Value::NativeFunction(|args, _interpreter| {
            if args.len() < 1 {
                return Err("blaze.logout espera 1 argumento: (session_id)".to_string());
            }

            let session_id = match &args[0] {
                Value::String(s) => s.clone(),
                _ => return Err("Primeiro argumento (session_id) deve ser uma string".to_string()),
            };

            if SESSIONS.write().unwrap().remove(&session_id).is_some() {
                let mut res_dict = HashMap::new();
                res_dict.insert(Value::String("success".to_string()), Value::Boolean(true));
                res_dict.insert(Value::String("logout".to_string()), Value::Boolean(true));
                Ok(Value::Dict(res_dict))
            } else {
                let mut res_dict = HashMap::new();
                res_dict.insert(Value::String("success".to_string()), Value::Boolean(false));
                res_dict.insert(Value::String("logout".to_string()), Value::Boolean(false));
                Ok(Value::Dict(res_dict))
            }
        }),
    );

    // Função para redirecionar
    module.insert(
        "redirect".to_string(),
        Value::NativeFunction(|args, _interpreter| {
            if args.len() != 2 {
                return Err("blaze.redirect espera 2 argumentos: (response, url)".to_string());
            }

            let res_dict = match &args[0] {
                Value::Dict(d) => d.clone(),
                _ => return Err("Primeiro argumento (response) deve ser um objeto Response".to_string()),
            };

            let url = match &args[1] {
                Value::String(s) => s.clone(),
                _ => return Err("Segundo argumento (url) deve ser uma string".to_string()),
            };

            let mut new_res = res_dict.clone();
            new_res.insert(Value::String("status".to_string()), Value::Number(302.0));
            
            let mut headers = new_res.get(&Value::String("headers".to_string()))
                .and_then(|v| if let Value::Dict(h) = v { Some(h.clone()) } else { None })
                .unwrap_or_else(HashMap::new);
            
            headers.insert(Value::String("Location".to_string()), Value::String(url));
            new_res.insert(Value::String("headers".to_string()), Value::Dict(headers));

            Ok(Value::Dict(new_res))
        }),
    );

        // Sub-módulo para manipulação de resposta

        let mut response_module: HashMap<Value, Value> = HashMap::new();

    

        response_module.insert(

            Value::String("set".to_string()),

            Value::NativeFunction(|args, _interpreter| {

                if args.len() != 3 {

                    return Err("blaze.response.set espera 3 argumentos: (response, key, value)".to_string());

                }

                let res_dict = match &args[0] {

                    Value::Dict(d) => d.clone(),

                    _ => return Err("Primeiro argumento (response) deve ser um objeto Response".to_string()),

                };

                let key = match &args[1] {

                    Value::String(s) => s.clone(),

                    _ => return Err("Segundo argumento (key) deve ser uma string".to_string()),

                };

                let value = args[2].clone();

    

                let mut new_res = res_dict.clone();

                let mut headers = new_res.get(&Value::String("headers".to_string()))

                    .and_then(|v| if let Value::Dict(h) = v { Some(h.clone()) } else { None })

                    .unwrap_or_else(HashMap::new);

    

                if key == "session_id" {

                    if let Value::String(s_id) = value {

                        let expiry = SystemTime::now()

                            .duration_since(UNIX_EPOCH)

                            .expect("Time went backwards")

                            .as_secs()

                            + (3600 * 24); // 24 horas

    

                        headers.insert(

                            Value::String("Set-Cookie".to_string()),

                            Value::String(format!(

                                "session_id={}; Max-Age={}; Path=/; HttpOnly",

                                s_id, expiry

                            )),

                        );

                    } else {

                        return Err("Valor para 'session_id' deve ser uma string".to_string());

                    }

                } else {

                    headers.insert(Value::String(key), value);

                }

                new_res.insert(Value::String("headers".to_string()), Value::Dict(headers));

                Ok(Value::Dict(new_res))

            }),

        );

    

        response_module.insert(

            Value::String("status".to_string()),

            Value::NativeFunction(|args, _interpreter| {

                if args.len() != 2 {

                    return Err("blaze.response.status espera 2 argumentos: (response, code)".to_string());

                }

                let res_dict = match &args[0] {

                    Value::Dict(d) => d.clone(),

                    _ => return Err("Primeiro argumento (response) deve ser um objeto Response".to_string()),

                };

                let code = match &args[1] {

                    Value::Number(n) => *n as f64,

                    _ => return Err("Segundo argumento (code) deve ser um número".to_string()),

                };

    

                let mut new_res = res_dict.clone();

                new_res.insert(Value::String("status".to_string()), Value::Number(code));

                Ok(Value::Dict(new_res))

            }),

        );

    

        response_module.insert(

            Value::String("clear_cookie".to_string()),

            Value::NativeFunction(|args, _interpreter| {

                if args.len() != 2 {

                    return Err("blaze.response.clear_cookie espera 2 argumentos: (response, key)".to_string());

                }

                let res_dict = match &args[0] {

                    Value::Dict(d) => d.clone(),

                    _ => return Err("Primeiro argumento (response) deve ser um objeto Response".to_string()),

                };

                let key = match &args[1] {

                    Value::String(s) => s.clone(),

                    _ => return Err("Segundo argumento (key) deve ser uma string".to_string()),

                };

    

                let mut new_res = res_dict.clone();

                let mut headers = new_res.get(&Value::String("headers".to_string()))

                    .and_then(|v| if let Value::Dict(h) = v { Some(h.clone()) } else { None })

                    .unwrap_or_else(HashMap::new);

                

                headers.insert(

                    Value::String("Set-Cookie".to_string()),

                    Value::String(format!(

                        "{}=; Max-Age=0; Path=/; HttpOnly",

                        key

                    )),

                );

                new_res.insert(Value::String("headers".to_string()), Value::Dict(headers));

                Ok(Value::Dict(new_res))

            }),

        );

    

        module.insert("response".to_string(), Value::Dict(response_module));

    // Registrar rota
    module.insert(
        "route".to_string(),
        Value::NativeFunction(|args, _interpreter| {
            if args.len() < 4 || args.len() > 5 {
                return Err("blaze.route espera 4 ou 5 argumentos: (app, method, path, handler, [is_protected])".to_string());
            }

            let app = match &args[0] {
                Value::Dict(d) => d.clone(),
                _ => return Err("Primeiro argumento deve ser uma aplicação Blaze".to_string()),
            };

            let method = match &args[1] {
                Value::String(s) => s.to_uppercase(),
                _ => return Err("Segundo argumento (method) deve ser uma string".to_string()),
            };

            let path = match &args[2] {
                Value::String(s) => s.clone(),
                _ => return Err("Terceiro argumento (path) deve ser uma string".to_string()),
            };

            let handler = args[3].clone();
            if !matches!(handler, Value::Function(_)) {
                return Err("Quarto argumento (handler) deve ser uma função".to_string());
            }

            let is_protected = if args.len() == 5 {
                match &args[4] {
                    Value::Boolean(b) => *b,
                    _ => return Err("Quinto argumento (is_protected) deve ser um booleano".to_string()),
                }
            } else {
                false // Padrão é não protegido
            };

            // Criar objeto de rota
            let mut route = HashMap::new();
            route.insert(Value::String("path".to_string()), Value::String(path));
            route.insert(Value::String("method".to_string()), Value::String(method));
            route.insert(Value::String("handler".to_string()), handler);
            route.insert(Value::String("protected".to_string()), Value::Boolean(is_protected));

            // Adicionar rota à lista
            let mut new_app = app.clone();
            if let Some(Value::List(ref mut routes)) = 
                new_app.get_mut(&Value::String("routes".to_string()))
            {
                routes.push(Value::Dict(route));
            }

            Ok(Value::Dict(new_app))
        }),
    );

    // Definir diretório de templates
    module.insert(
        "set_templates_dir".to_string(),
        Value::NativeFunction(|args, _interpreter| {
            if args.len() < 1 {
                return Err(
                    "blaze.set_templates_dir espera 1 argumento: (path)".to_string(),
                );
            }
            let path = match &args[0] {
                Value::String(s) => s.clone(),
                _ => return Err("Primeiro argumento (path) deve ser uma string".to_string()),
            };

            *TEMPLATE_DIR.write().unwrap() = Some(path);
            Ok(Value::Nil)
        }),
    );

    // Definir diretório de arquivos estáticos (CSS, JS, imagens)
    module.insert(
        "set_static_dir".to_string(),
        Value::NativeFunction(|args, _interpreter| {
            if args.len() < 1 {
                return Err(
                    "blaze.set_static_dir espera 1 argumento: (path)".to_string(),
                );
            }
            let path = match &args[0] {
                Value::String(s) => s.clone(),
                _ => return Err("Primeiro argumento (path) deve ser uma string".to_string()),
            };

            *STATIC_DIR.write().unwrap() = Some(path);
            Ok(Value::Nil)
        }),
    );

    // Renderizar template HTML
    module.insert(
        "render_template".to_string(),
        Value::NativeFunction(|args, _interpreter| {
            if args.len() < 2 {
                return Err(
                    "blaze.render_template espera 2-4 argumentos: (response, template_name, [data], [css_files])"
                        .to_string(),
                );
            }
            
            // Se apenas 2 argumentos, criar dados e CSS vazios
            let data_arg = if args.len() >= 3 { &args[2] } else { &Value::Dict(HashMap::new()) };
            let css_arg = if args.len() >= 4 { &args[3] } else { &Value::List(Vec::new()) };

            let res = match &args[0] {
                Value::Dict(d) => d.clone(),
                _ => return Err("Primeiro argumento (response) deve ser um objeto Response".to_string()),
            };
            let template_name = match &args[1] {
                Value::String(s) => s.clone(),
                _ => return Err("Segundo argumento (template_name) deve ser uma string".to_string()),
            };

            let template_dir = TEMPLATE_DIR.read().unwrap().clone().ok_or_else(|| {
                "Diretório de templates não configurado. Use blaze.set_templates_dir().".to_string()
            })?;
            
            let template_path = format!("{}/{}", template_dir, template_name);

            let mut template_content = fs::read_to_string(&template_path)
                .map_err(|e| format!("Erro ao ler arquivo de template '{}': {}", template_path, e))?;

            // Processar dados para substituição (aceita Dict ou Nil)
            if !matches!(data_arg, Value::Nil) {
                if let Value::Dict(data) = data_arg {
                    for (key, value) in data {
                        if let Value::String(k) = key {
                            // Converter qualquer valor para string
                            let v_str = match value.clone() {
                                Value::String(s) => s,
                                Value::Number(n) => n.to_string(),
                                Value::Boolean(b) => b.to_string(),
                                Value::Nil => "nil".to_string(),
                                _ => value.to_string(),
                            };
                            let k_name = k.to_string();
                            let placeholder = "{{{".repeat(4) + &k_name + "}}}".repeat(4).as_str();
                            template_content = template_content.replace(&placeholder, &v_str);
                        }
                    }
                }
            }

            // Processar inclusão de arquivos CSS (aceita List ou Nil)
            if !matches!(css_arg, Value::Nil) && !matches!(css_arg, Value::Dict(_)) {
                if let Value::List(css_files) = css_arg {
                    if !css_files.is_empty() {
                        let mut css_links = String::new();
                        let static_dir = STATIC_DIR.read().unwrap().clone();
                        
                        for css_file_val in css_files {
                            if let Value::String(css_file) = css_file_val {
                                // Se o diretório estático está configurado, usar caminho relativo
                                if let Some(ref static_path) = static_dir {
                                    let css_path = format!("{}/{}", static_path, css_file);
                                    if std::path::Path::new(&css_path).exists() {
                                        css_links.push_str(&format!("<link rel=\"stylesheet\" href=\"/static/{}\">
", css_file));
                                    }
                                }
                            }
                        }
                        
                        // Substituir placeholder {{CSS}} ou inserir antes de </head>
                        if template_content.contains("{{CSS}}") {
                            template_content = template_content.replace("{{CSS}}", &css_links);
                        } else if template_content.contains("</head>") {
                            template_content = template_content.replace("</head>", &format!("{}\n</head>", css_links));
                        } else {
                            // Se não há tag </head>, inserir no início
                            template_content = format!("{}\n{}", css_links, template_content);
                        }
                    }
                }
            }
            
            let mut new_res = res.clone();
            new_res.insert(Value::String("body".to_string()), Value::String(template_content));
            
            // Definir Content-Type como text/html
            if let Some(Value::Dict(mut headers)) = new_res.get(&Value::String("headers".to_string())).cloned() {
                headers.insert(
                    Value::String("Content-Type".to_string()), 
                    Value::String("text/html".to_string())
                );
                new_res.insert(Value::String("headers".to_string()), Value::Dict(headers));
            }

            Ok(Value::Dict(new_res))
        }),
    );

    // Função helper para criar um dicionário com valores
    module.insert(
        "create_dict".to_string(),
        Value::NativeFunction(|args, _interpreter| {
            let mut dict = HashMap::new();
            // Aceita pares key-value: create_dict("key1", "value1", "key2", "value2", ...)
            if args.len() % 2 != 0 {
                return Err("blaze.create_dict espera um número par de argumentos (pares key-value)".to_string());
            }
            for i in (0..args.len()).step_by(2) {
                if let (Value::String(k), v) = (&args[i], &args[i + 1]) {
                    if let Value::String(s) = v {
                        dict.insert(Value::String(k.clone()), Value::String(s.clone()));
                    } else {
                        dict.insert(Value::String(k.clone()), v.clone());
                    }
                } else {
                    return Err("blaze.create_dict espera strings como chaves".to_string());
                }
            }
            Ok(Value::Dict(dict))
        }),
    );

    // Função helper para criar uma lista com valores
    module.insert(
        "create_list".to_string(),
        Value::NativeFunction(|args, _interpreter| {
            Ok(Value::List(args.clone()))
        }),
    );

    // Função helper para incluir CSS inline ou externo
    module.insert(
        "include_css".to_string(),
        Value::NativeFunction(|args, _interpreter| {
            if args.len() < 1 {
                return Err("blaze.include_css espera 1-2 argumentos: (css_file, [inline])".to_string());
            }

            let css_file = match &args[0] {
                Value::String(s) => s.clone(),
                _ => return Err("Primeiro argumento (css_file) deve ser uma string".to_string()),
            };

            let inline = args.len() >= 2 && match &args[1] {
                Value::Boolean(b) => *b,
                _ => false,
            };

            let static_dir = STATIC_DIR.read().unwrap().clone();
            
            if inline {
                // Incluir CSS inline
                if let Some(ref static_path) = static_dir {
                    let css_path = format!("{}/{}", static_path, css_file);
                    match fs::read_to_string(&css_path) {
                        Ok(css_content) => {
                            Ok(Value::String(format!("<style>\n{}\n</style>", css_content)))
                        },
                        Err(_) => Ok(Value::String(format!("<!-- CSS file '{}' not found -->", css_file)))
                    }
                } else {
                    Err("Diretório de arquivos estáticos não configurado. Use blaze.set_static_dir().".to_string())
                }
            } else {
                // Retornar tag <link> para CSS externo
                Ok(Value::String(format!("<link rel=\"stylesheet\" href=\"/static/{}\">", css_file)))
            }
        }),
    );

    // Servir arquivos estáticos
    module.insert(
        "serve_static".to_string(),
        Value::NativeFunction(|args, _interpreter| {
            if args.len() < 3 {
                return Err(
                    "blaze.serve_static espera 3 argumentos: (app, url_path_prefix, fs_directory)"
                        .to_string(),
                );
            }
            let app = match &args[0] {
                Value::Dict(d) => d.clone(),
                _ => return Err("Primeiro argumento (app) deve ser uma aplicação Blaze".to_string()),
            };
            let url_path_prefix = match &args[1] {
                Value::String(s) => s.clone(),
                _ => return Err("Segundo argumento (url_path_prefix) deve ser uma string".to_string()),
            };
            let fs_directory = match &args[2] {
                Value::String(s) => s.clone(),
                _ => return Err("Terceiro argumento (fs_directory) deve ser uma string".to_string()),
            };

            // Criar objeto de rota estática especial
            let mut static_route = HashMap::new();
            static_route.insert(Value::String("path".to_string()), Value::String(url_path_prefix));
            static_route.insert(Value::String("method".to_string()), Value::String("_STATIC_ROUTE_".to_string()));
            static_route.insert(Value::String("handler".to_string()), Value::String(fs_directory));

            let mut new_app = app.clone();
            if let Some(Value::List(ref mut routes)) = 
                new_app.get_mut(&Value::String("routes".to_string()))
            {
                routes.push(Value::Dict(static_route));
            }
            Ok(Value::Dict(new_app))
        }),
    );

    // Criar objeto Response
    module.insert(
        "response".to_string(),
        Value::NativeFunction(|args, _interpreter| {
            let mut res = HashMap::new();
            res.insert(Value::String("status".to_string()), Value::Number(200.0));
            res.insert(
                Value::String("headers".to_string()),
                Value::Dict(HashMap::new()),
            );
            
            let body = if args.is_empty() {
                Value::String("".to_string())
            } else {
                args[0].clone()
            };
            res.insert(Value::String("body".to_string()), body);

            Ok(Value::Dict(res))
        }),
    );

    // Iniciar o servidor
    module.insert(
        "listen".to_string(),
        Value::NativeFunction(|args, interpreter| {
            if args.len() < 2 {
                return Err("blaze.listen espera 2 argumentos: (app, port_or_address)".to_string());
            }

            let app = match &args[0] {
                Value::Dict(d) => d,
                _ => return Err("Primeiro argumento deve ser uma aplicação Blaze".to_string()),
            };

            let addr = match &args[1] {
                Value::String(s) => s.clone(),
                Value::Number(n) => format!("127.0.0.1:{}", *n as u16),
                _ => return Err("O segundo argumento para listen deve ser uma string de endereço (ex: '127.0.0.1:8000') ou um número de porta.".to_string()),
            };

            let routes_value = app
                .get(&Value::String("routes".to_string()))
                .cloned()
                .unwrap_or(Value::List(Vec::new()));

            let routes = match routes_value {
                Value::List(r) => r,
                _ => return Err("Estrutura de rotas inválida na aplicação.".to_string()),
            };

            println!("\n🔥 Blaze v1.1 - Servidor iniciado em http://{}...", addr);
            println!("   Pressione Ctrl+C para parar.\n");

            let interpreter = Arc::new(Mutex::new((*interpreter).clone()));
            let routes = Arc::new(routes);

            rouille::start_server(addr, move |request| {
                let mut interp = interpreter.lock().unwrap();
                let routes_clone = routes.clone();

                // Ler cookie de sessão
                let mut session_id_from_cookie: Option<String> = None;
                let mut cookies_iter = rouille::input::cookies(&request);
                if let Some((name, value)) = cookies_iter.find(|(name, _)| *name == "session_id") {
                    session_id_from_cookie = Some(value.to_string());
                }

                // Validar sessão
                let mut authenticated_user_id: Option<String> = None;
                if let Some(s_id) = &session_id_from_cookie {
                    authenticated_user_id = is_authenticated(s_id);
                }

                let mut new_session_id_from_handler: Option<String> = None;
                let mut logout_from_handler: bool = false;

                let mut response = {
                    let mut response_to_return = Response::empty_404();
                    let mut route_found = false;

                    // 1. Tenta servir arquivos estáticos
                    for route_value in routes_clone.iter() {
                        if let Value::Dict(route_map) = route_value {
                            let path_val = route_map.get(&Value::String("path".to_string()));
                            let method_val = route_map.get(&Value::String("method".to_string()));
                            let handler_val = route_map.get(&Value::String("handler".to_string()));

                            if let ( 
                                Some(Value::String(path)), 
                                Some(Value::String(method)), 
                                Some(handler),
                            ) = (path_val, method_val, handler_val) {
                                if method == "_STATIC_ROUTE_" {
                                    if let Some(suffix) = request.url().strip_prefix(path) {
                                        if let Value::String(fs_dir) = handler {
                                            let mut asset_path = std::path::PathBuf::from(fs_dir);
                                            for component in std::path::Path::new(suffix).components() {
                                                if let std::path::Component::Normal(part) = component {
                                                    asset_path.push(part);
                                                }
                                            }
                                            if asset_path.is_file() {
                                                if let Ok(file) = std::fs::File::open(&asset_path) {
                                                    let mime = rouille::extension_to_mime(asset_path.extension().and_then(|s| s.to_str()).unwrap_or(""));
                                                    response_to_return = Response::from_file(mime, file);
                                                    route_found = true;
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    if route_found {
                        response_to_return
                    } else {
                        // 2. Se nenhum arquivo estático foi servido, tenta rotas dinâmicas
                        for route_value in routes_clone.iter() {
                            if let Value::Dict(route_map) = route_value {
                                let path_val = route_map.get(&Value::String("path".to_string()));
                                let method_val = route_map.get(&Value::String("method".to_string()));
                                let handler_val = route_map.get(&Value::String("handler".to_string()));
                                let protected_val = route_map.get(&Value::String("protected".to_string()));

                                let is_protected = if let Some(Value::Boolean(b)) = protected_val { *b } else { false };

                                if let ( 
                                    Some(Value::String(path)), 
                                    Some(Value::String(method)), 
                                    Some(handler),
                                ) = (path_val, method_val, handler_val) {
                                    if let Some(params) = match_route(path, &request.url()) {
                                        if request.method() == *method {
                                            // Checar proteção da rota
                                            if is_protected && authenticated_user_id.is_none() {
                                                // Redirecionar para login ou erro
                                                response_to_return = Response::redirect_302("/login"); // Exemplo de redirecionamento
                                                route_found = true;
                                                break; // Sair do loop de rotas
                                            }

                                            let req_obj = build_request_object(
                                                request,
                                                params,
                                                authenticated_user_id.is_some(),
                                                authenticated_user_id.clone(),
                                                session_id_from_cookie.clone(),
                                            );
                                            if let Value::Function(_) = handler {
                                                let res_obj = Value::Dict(HashMap::new()); // Default response object
                                                let mut handler_res_val = match interp.call_function_by_value(handler.clone(), vec![req_obj, res_obj]) {
                                                    Ok(res_val) => res_val,
                                                    Err(e) => {
                                                        eprintln!("[Erro no Handler da Rota] {}", e);
                                                        response_to_return = Response::text("Erro interno no servidor").with_status_code(500);
                                                        route_found = true; // Considere a rota encontrada para evitar 404
                                                        break; // Sair do loop de rotas
                                                    }
                                                };

                                                let mut new_session_id_from_handler: Option<String> = None;
                                                let mut logout_from_handler: bool = false;

                                                if let Value::Dict(ref mut res_map) = handler_res_val {
                                                    if let Some(session_id_val) = res_map.remove(&Value::String("session_id".to_string())) {
                                                        if let Value::String(s_id) = session_id_val {
                                                            new_session_id_from_handler = Some(s_id);
                                                        }
                                                    }
                                                    if let Some(logout_val) = res_map.remove(&Value::String("logout".to_string())) {
                                                        if let Value::Boolean(b) = logout_val {
                                                            logout_from_handler = b;
                                                        }
                                                    }
                                                }
                                                
                                                response_to_return = to_rouille_response(handler_res_val);
                                            }
                                            route_found = true;
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                        response_to_return
                    }
                };

                // Manipular cookies de sessão
                if let Some(new_session_id) = new_session_id_from_handler {
                    let cookie_header = format!(
                        "session_id={}; Max-Age={}; Path=/; HttpOnly",
                        new_session_id,
                        3600 * 24 // 24 horas
                    );
                    response = response.with_additional_header("Set-Cookie", cookie_header);
                } else if logout_from_handler {
                    let cookie_header = format!(
                        "session_id=; Max-Age=0; Path=/; HttpOnly" // Clear cookie
                    );
                    response = response.with_additional_header("Set-Cookie", cookie_header);
                }
                
                let now = Local::now();
                println!(
                    "{} \"{} {}\" {} {}",
                    now.format("[%d/%b/%Y %H:%M:%S]"),
                    request.method(),
                    request.url(),
                    response.status_code,
                    request.remote_addr()
                );

                response
            } );
        }),
    );

    let dict_map = module
        .into_iter()
        .map(|(k, v)| (Value::String(k), v))
        .collect();

    Value::Dict(dict_map)
}

fn is_authenticated(session_id: &str) -> Option<String> {
    let mut sessions = SESSIONS.write().unwrap();
    if let Some(session) = sessions.get(session_id) {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        if current_time < session.expiry {
            Some(session.user_id.clone())
        } else {
            // Sessão expirada, remover
            sessions.remove(session_id);
            None
        }
    } else {
        None
    }
}

fn match_route(route_path: &str, request_url: &str) -> Option<HashMap<String, String>> {
    let route_parts: Vec<&str> = route_path.split('/').filter(|s| !s.is_empty()).collect();
    let request_parts: Vec<&str> = request_url.split('/').filter(|s| !s.is_empty()).collect();

    if route_parts.len() != request_parts.len() {
        return None;
    }

    let mut params = HashMap::new();

    for (route_part, request_part) in route_parts.iter().zip(request_parts.iter()) {
        if let Some(param_name) = route_part.strip_prefix(':') {
            params.insert(param_name.to_string(), (*request_part).to_string());
        } else if route_part != request_part {
            return None;
        }
    }

    Some(params)
}

/// Constrói um objeto `Value::Dict` a partir de um `rouille::Request`
fn build_request_object(
    request: &Request,
    params: HashMap<String, String>,
    is_authenticated: bool,
    user_id: Option<String>,
    session_id: Option<String>,
) -> Value {
    let mut req_map = HashMap::new();
    req_map.insert(
        Value::String("method".to_string()),
        Value::String(request.method().to_string()),
    );
    req_map.insert(
        Value::String("path".to_string()),
        Value::String(request.url()),
    );

    let mut headers = HashMap::new();
    for (key, value) in request.headers() {
        headers.insert(Value::String(key.to_string()), Value::String(value.to_string()));
    }
    req_map.insert(Value::String("headers".to_string()), Value::Dict(headers));
    
    let mut params_map = HashMap::new();
    for (k, v) in params {
        params_map.insert(Value::String(k), Value::String(v));
    }
    req_map.insert(Value::String("params".to_string()), Value::Dict(params_map));

    // Adicionar informações de autenticação
    req_map.insert(Value::String("is_authenticated".to_string()), Value::Boolean(is_authenticated));
    req_map.insert(Value::String("user_id".to_string()), user_id.map_or(Value::Nil, Value::String));
    req_map.insert(Value::String("session_id".to_string()), session_id.map_or(Value::Nil, Value::String));

    Value::Dict(req_map)
}

/// Converte um `Value::Dict` de resposta do Snask para um `rouille::Response`
fn to_rouille_response(res_val: Value) -> Response {
    match res_val {
        Value::Dict(res_map) => {
            let status = res_map
                .get(&Value::String("status".to_string()))
                .and_then(|v| match v {
                    Value::Number(n) => Some(*n as u16),
                    _ => None,
                })
                .unwrap_or(200);

            let body = res_map
                .get(&Value::String("body".to_string()))
                .cloned()
                .unwrap_or(Value::String("".to_string()));

            let mut response = Response::text(body.to_string()).with_status_code(status);

            if let Some(Value::Dict(headers)) = res_map.get(&Value::String("headers".to_string())) {
                for (key, value) in headers {
                    if let (Value::String(k), Value::String(v)) = (key, value) {
                        response = response.with_additional_header(k.clone(), v.clone());
                    }
                }
            }
            
            response
        }
        _ => Response::text("Handler não retornou um objeto de resposta válido")
            .with_status_code(500),
    }
}