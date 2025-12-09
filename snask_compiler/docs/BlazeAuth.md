# Sistema de Autenticação Blaze

## Visão Geral

O módulo `blaze_auth` fornece um sistema completo de autenticação para aplicações Blaze, incluindo:

- ✅ Registro de usuários com validação
- ✅ Login com hash seguro de senhas (bcrypt)
- ✅ Gerenciamento de sessões com expiração
- ✅ Proteção de rotas
- ✅ Alteração de senha
- ✅ Gerenciamento de usuários

## Instalação

O módulo já está incluído na biblioteca padrão do Snask. Basta importar:

```snask
// blaze_auth está disponível globalmente
let resultado = blaze_auth.registrar("usuario", "email@example.com", "senha123")
```

## API de Referência

### `blaze_auth.registrar(username, email, password)`

Registra um novo usuário no sistema.

**Parâmetros:**
- `username` (string): Nome de usuário (mínimo 3 caracteres)
- `email` (string): Email válido
- `password` (string): Senha (mínimo 6 caracteres)

**Retorna:**
```snask
{
    "sucesso": true/false,
    "user_id": "uuid-do-usuario",
    "username": "nome-usuario",
    "erro": "mensagem de erro" // se sucesso = false
}
```

**Exemplo:**
```snask
let resultado = blaze_auth.registrar("joao", "joao@email.com", "senha123")

if blaze.boolean(resultado.sucesso) {
    print("Usuário criado: " + resultado.user_id)
} else {
    print("Erro: " + resultado.erro)
}
```

---

### `blaze_auth.login(username, password)`

Autentica um usuário e cria uma sessão.

**Parâmetros:**
- `username` (string): Nome de usuário
- `password` (string): Senha

**Retorna:**
```snask
{
    "sucesso": true/false,
    "session_id": "uuid-da-sessao",
    "user_id": "uuid-do-usuario",
    "username": "nome-usuario",
    "erro": "mensagem de erro" // se sucesso = false
}
```

**Exemplo:**
```snask
let resultado = blaze_auth.login("joao", "senha123")

if blaze.boolean(resultado.sucesso) {
    // Armazenar session_id em cookie
    res = blaze.response.set(res, "session_id", resultado.session_id)
    print("Login bem-sucedido!")
} else {
    print("Erro: " + resultado.erro)
}
```

---

### `blaze_auth.logout(session_id)`

Encerra uma sessão de usuário.

**Parâmetros:**
- `session_id` (string): ID da sessão a ser encerrada

**Retorna:**
```snask
{
    "sucesso": true/false
}
```

**Exemplo:**
```snask
let resultado = blaze_auth.logout(session_id)
res = blaze.response.clear_cookie(res, "session_id")
```

---

### `blaze_auth.verificar_sessao(session_id)`

Verifica se uma sessão é válida e retorna dados do usuário.

**Parâmetros:**
- `session_id` (string): ID da sessão

**Retorna:**
```snask
{
    "valido": true/false,
    "user_id": "uuid-do-usuario",
    "username": "nome-usuario",
    "email": "email@usuario.com"
}
```

**Exemplo:**
```snask
let sessao = blaze_auth.verificar_sessao(req.session_id)

if blaze.boolean(sessao.valido) {
    print("Usuário autenticado: " + sessao.username)
} else {
    // Redirecionar para login
    res = blaze.redirect(res, "/login")
}
```

---

### `blaze_auth.usuario_atual(session_id)`

Obtém informações do usuário autenticado.

**Parâmetros:**
- `session_id` (string): ID da sessão

**Retorna:**
```snask
{
    "id": "uuid-do-usuario",
    "username": "nome-usuario",
    "email": "email@usuario.com",
    "ativo": true/false
}
// ou nil se sessão inválida
```

**Exemplo:**
```snask
let usuario = blaze_auth.usuario_atual(req.session_id)

if usuario {
    print("Bem-vindo, " + usuario.username)
}
```

---

### `blaze_auth.alterar_senha(username, senha_antiga, senha_nova)`

Altera a senha de um usuário.

**Parâmetros:**
- `username` (string): Nome de usuário
- `senha_antiga` (string): Senha atual
- `senha_nova` (string): Nova senha (mínimo 6 caracteres)

**Retorna:**
```snask
{
    "sucesso": true/false,
    "erro": "mensagem de erro" // se sucesso = false
}
```

**Exemplo:**
```snask
let resultado = blaze_auth.alterar_senha("joao", "senha123", "novaSenha456")

if blaze.boolean(resultado.sucesso) {
    print("Senha alterada com sucesso!")
}
```

---

### `blaze_auth.listar_usuarios()`

Lista todos os usuários cadastrados (função administrativa).

**Retorna:**
```snask
[
    {
        "id": "uuid",
        "username": "usuario1",
        "email": "email1@example.com",
        "ativo": true
    },
    // ...
]
```

---

### `blaze_auth.desativar_usuario(username)`

Desativa uma conta de usuário.

**Parâmetros:**
- `username` (string): Nome de usuário

**Retorna:**
```snask
{
    "sucesso": true/false,
    "erro": "mensagem de erro" // se sucesso = false
}
```

---

## Exemplo Completo

```snask
// Criar aplicação
let app = blaze.create()
blaze.set_templates_dir("./templates")

// Rota de registro
fun register_handler(req, res) {
    let username = req.body.username
    let email = req.body.email
    let password = req.body.password
    
    let resultado = blaze_auth.registrar(username, email, password)
    
    if blaze.boolean(resultado.sucesso) {
        res = blaze.redirect(res, "/login")
    } else {
        res = blaze.render_template(res, "register.html", 
            blaze.create_dict("erro", resultado.erro))
    }
    
    return res
}

app = blaze.route(app, "POST", "/register", register_handler, false)

// Rota de login
fun login_handler(req, res) {
    let username = req.body.username
    let password = req.body.password
    
    let resultado = blaze_auth.login(username, password)
    
    if blaze.boolean(resultado.sucesso) {
        res = blaze.response.set(res, "session_id", resultado.session_id)
        res = blaze.redirect(res, "/dashboard")
    } else {
        res = blaze.render_template(res, "login.html",
            blaze.create_dict("erro", resultado.erro))
    }
    
    return res
}

app = blaze.route(app, "POST", "/login", login_handler, false)

// Rota protegida
fun dashboard_handler(req, res) {
    let usuario = blaze_auth.usuario_atual(req.session_id)
    
    if usuario {
        res = blaze.render_template(res, "dashboard.html",
            blaze.create_dict("username", usuario.username, "email", usuario.email))
    } else {
        res = blaze.redirect(res, "/login")
    }
    
    return res
}

app = blaze.route(app, "GET", "/dashboard", dashboard_handler, true)

blaze.listen(app, 3000)
```

## Templates Incluídos

O sistema vem com templates HTML prontos:

### `templates/login.html`
- Design moderno com gradiente
- Validação de formulário
- Mensagens de erro
- Link para cadastro

### `templates/register.html`
- Formulário completo de cadastro
- Validação de senha (confirmação)
- Requisitos de senha visíveis
- Mensagens de sucesso/erro

## Segurança

### Hash de Senhas
- Utiliza bcrypt com custo padrão (12 rounds)
- Senhas nunca são armazenadas em texto plano

### Sessões
- IDs únicos gerados com UUID v4
- Expiração automática após 7 dias
- Limpeza automática de sessões expiradas

### Validações
- Username: mínimo 3 caracteres
- Email: formato válido
- Senha: mínimo 6 caracteres
- Verificação de duplicatas

## Boas Práticas

1. **Sempre use HTTPS em produção**
2. **Configure cookies seguros:**
   ```snask
   res = blaze.response.set(res, "session_id", session_id)
   // Adicionar flags: HttpOnly, Secure, SameSite
   ```

3. **Implemente rate limiting para login**
4. **Adicione CSRF protection para formulários**
5. **Valide entrada do usuário**
6. **Implemente recuperação de senha**
7. **Adicione autenticação de dois fatores (2FA)**

## Roadmap

- [ ] Recuperação de senha via email
- [ ] Autenticação de dois fatores (2FA)
- [ ] OAuth/Social login
- [ ] Permissões e roles
- [ ] Auditoria de login
- [ ] Bloqueio de conta após tentativas falhas

---

**Desenvolvido para Blaze Framework v1.1**
