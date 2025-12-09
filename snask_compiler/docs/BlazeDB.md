# Blaze_DB - ORM para SQLite

## Visão Geral

O módulo `blaze_db` fornece um ORM (Object-Relational Mapping) simples para trabalhar com bancos de dados SQLite no Snask. Inspirado no Django ORM, oferece uma interface intuitiva para operações CRUD.

## Recursos

- ✅ Conexão com SQLite
- ✅ Criação de tabelas
- ✅ Operações CRUD (Create, Read, Update, Delete)
- ✅ Consultas com condições
- ✅ Contagem de registros
- ✅ Execução de SQL direto
- ✅ Thread-safe

## Instalação

O módulo já está incluído na biblioteca padrão do Snask:

```snask
// blaze_db está disponível globalmente
let resultado = blaze_db.conectar("meu_banco.db")
```

## API de Referência

### `blaze_db.conectar(caminho_db)`

Conecta ao banco de dados SQLite. Cria o arquivo se não existir.

**Parâmetros:**
- `caminho_db` (string): Caminho para o arquivo do banco de dados

**Retorna:**
```snask
{
    "sucesso": true/false,
    "mensagem": "Conectado a meu_banco.db"
}
```

**Exemplo:**
```snask
let resultado = blaze_db.conectar("app.db")
if resultado.sucesso {
    println("Conectado!")
}
```

---

### `blaze_db.criar_tabela(nome, colunas)`

Cria uma nova tabela no banco de dados.

**Parâmetros:**
- `nome` (string): Nome da tabela
- `colunas` (dict): Dicionário com nome_coluna: tipo_sql

**Retorna:**
```snask
{
    "sucesso": true/false,
    "mensagem": "Tabela 'usuarios' criada"
}
```

**Exemplo:**
```snask
let colunas = blaze.create_dict(
    "id", "INTEGER PRIMARY KEY AUTOINCREMENT",
    "nome", "TEXT NOT NULL",
    "email", "TEXT UNIQUE",
    "idade", "INTEGER",
    "ativo", "INTEGER DEFAULT 1"
)

let resultado = blaze_db.criar_tabela("usuarios", colunas)
```

---

### `blaze_db.inserir(tabela, dados)`

Insere um novo registro na tabela.

**Parâmetros:**
- `tabela` (string): Nome da tabela
- `dados` (dict): Dicionário com coluna: valor

**Retorna:**
```snask
{
    "sucesso": true,
    "id": 1,  // ID do registro inserido
    "linhas_afetadas": 1
}
```

**Exemplo:**
```snask
let usuario = blaze.create_dict(
    "nome", "João Silva",
    "email", "joao@email.com",
    "idade", "25"
)

let resultado = blaze_db.inserir("usuarios", usuario)
println("Usuário criado com ID: " + format("{}", resultado.id))
```

---

### `blaze_db.consultar(tabela, [condicao])`

Consulta registros da tabela.

**Parâmetros:**
- `tabela` (string): Nome da tabela
- `condicao` (string, opcional): Condição WHERE do SQL

**Retorna:**
```snask
[
    {"id": "1", "nome": "João", "email": "joao@email.com"},
    {"id": "2", "nome": "Maria", "email": "maria@email.com"}
]
```

**Exemplos:**
```snask
// Todos os registros
let todos = blaze_db.consultar("usuarios")

// Com condição
let ativos = blaze_db.consultar("usuarios", "ativo = 1")
let adultos = blaze_db.consultar("usuarios", "idade >= 18")
let especifico = blaze_db.consultar("usuarios", "email = 'joao@email.com'")
```

---

### `blaze_db.atualizar(tabela, dados, condicao)`

Atualiza registros existentes.

**Parâmetros:**
- `tabela` (string): Nome da tabela
- `dados` (dict): Dicionário com coluna: novo_valor
- `condicao` (string): Condição WHERE

**Retorna:**
```snask
{
    "sucesso": true,
    "linhas_afetadas": 1
}
```

**Exemplo:**
```snask
let novos_dados = blaze.create_dict(
    "nome", "João Pedro Silva",
    "idade", "26"
)

let resultado = blaze_db.atualizar("usuarios", novos_dados, "id = 1")
println("Linhas atualizadas: " + format("{}", resultado.linhas_afetadas))
```

---

### `blaze_db.deletar(tabela, condicao)`

Deleta registros da tabela.

**Parâmetros:**
- `tabela` (string): Nome da tabela
- `condicao` (string): Condição WHERE

**Retorna:**
```snask
{
    "sucesso": true,
    "linhas_afetadas": 1
}
```

**Exemplo:**
```snask
// Deletar por ID
let resultado = blaze_db.deletar("usuarios", "id = 5")

// Deletar múltiplos
let resultado = blaze_db.deletar("usuarios", "ativo = 0")
```

---

### `blaze_db.contar(tabela, [condicao])`

Conta registros na tabela.

**Parâmetros:**
- `tabela` (string): Nome da tabela
- `condicao` (string, opcional): Condição WHERE

**Retorna:**
```snask
10  // Número de registros
```

**Exemplos:**
```snask
// Total de registros
let total = blaze_db.contar("usuarios")
println("Total de usuários: " + format("{}", total))

// Com condição
let ativos = blaze_db.contar("usuarios", "ativo = 1")
let adultos = blaze_db.contar("usuarios", "idade >= 18")
```

---

### `blaze_db.executar(sql)`

Executa SQL direto (para operações avançadas).

**Parâmetros:**
- `sql` (string): Comando SQL

**Retorna:**
```snask
{
    "sucesso": true,
    "linhas_afetadas": 1
}
```

**Exemplo:**
```snask
// Criar índice
let resultado = blaze_db.executar("CREATE INDEX idx_email ON usuarios(email)")

// Alterar tabela
let resultado = blaze_db.executar("ALTER TABLE usuarios ADD COLUMN telefone TEXT")
```

---

## Exemplo Completo: Sistema de Blog

```snask
// Conectar ao banco
blaze_db.conectar("blog.db")

// Criar tabelas
let colunas_posts = blaze.create_dict(
    "id", "INTEGER PRIMARY KEY AUTOINCREMENT",
    "titulo", "TEXT NOT NULL",
    "conteudo", "TEXT",
    "autor_id", "INTEGER",
    "criado_em", "TEXT DEFAULT CURRENT_TIMESTAMP"
)
blaze_db.criar_tabela("posts", colunas_posts)

let colunas_comentarios = blaze.create_dict(
    "id", "INTEGER PRIMARY KEY AUTOINCREMENT",
    "post_id", "INTEGER",
    "autor", "TEXT",
    "texto", "TEXT",
    "criado_em", "TEXT DEFAULT CURRENT_TIMESTAMP"
)
blaze_db.criar_tabela("comentarios", colunas_comentarios)

// Inserir post
let post = blaze.create_dict(
    "titulo", "Meu Primeiro Post",
    "conteudo", "Este é o conteúdo do post...",
    "autor_id", "1"
)
let r = blaze_db.inserir("posts", post)
let post_id = r.id

// Inserir comentários
let comentario1 = blaze.create_dict(
    "post_id", format("{}", post_id),
    "autor", "João",
    "texto", "Ótimo post!"
)
blaze_db.inserir("comentarios", comentario1)

// Consultar posts com comentários
let posts = blaze_db.consultar("posts")
let i = 0
while i < collections.len(posts) {
    let post = collections.get(posts, i)
    println("Post: " + post.titulo)
    
    let condicao = "post_id = " + post.id
    let comentarios = blaze_db.consultar("comentarios", condicao)
    println("  Comentários: " + format("{}", collections.len(comentarios)))
    
    i = i + 1
}
```

## Tipos de Dados SQLite

| Tipo SQL | Descrição | Exemplo |
|----------|-----------|---------|
| `INTEGER` | Número inteiro | `id INTEGER` |
| `TEXT` | String | `nome TEXT` |
| `REAL` | Número decimal | `preco REAL` |
| `BLOB` | Dados binários | `imagem BLOB` |

### Constraints Comuns

```snask
"id INTEGER PRIMARY KEY AUTOINCREMENT"  // Chave primária auto-incremento
"email TEXT UNIQUE"                      // Valor único
"nome TEXT NOT NULL"                     // Não pode ser nulo
"ativo INTEGER DEFAULT 1"                // Valor padrão
"criado_em TEXT DEFAULT CURRENT_TIMESTAMP"  // Timestamp automático
```

## Boas Práticas

### 1. Sempre Verificar Sucesso
```snask
let resultado = blaze_db.inserir("usuarios", dados)
if resultado.sucesso {
    println("Inserido com ID: " + format("{}", resultado.id))
} else {
    println("Erro: " + resultado.erro)
}
```

### 2. Usar Transações (SQL direto)
```snask
blaze_db.executar("BEGIN TRANSACTION")
// ... operações ...
blaze_db.executar("COMMIT")
```

### 3. Criar Índices para Performance
```snask
blaze_db.executar("CREATE INDEX idx_email ON usuarios(email)")
blaze_db.executar("CREATE INDEX idx_post_id ON comentarios(post_id)")
```

### 4. Validar Dados Antes de Inserir
```snask
if email.contains("@") {
    blaze_db.inserir("usuarios", dados)
} else {
    println("Email inválido")
}
```

## Limitações

- ✅ Suporta apenas SQLite (por enquanto)
- ✅ Sem suporte a JOINs diretos (use SQL direto)
- ✅ Sem migrations automáticas
- ✅ Sem relacionamentos ORM (use foreign keys no SQL)

## Roadmap

- [ ] Suporte a PostgreSQL
- [ ] Query builder avançado
- [ ] Migrations automáticas
- [ ] Relacionamentos ORM (hasMany, belongsTo)
- [ ] Validações integradas
- [ ] Soft deletes
- [ ] Timestamps automáticos

## Comparação com Django ORM

| Recurso | Django ORM | Blaze_DB |
|---------|------------|----------|
| Criar tabela | `class Model` | `criar_tabela()` |
| Inserir | `Model.objects.create()` | `inserir()` |
| Consultar | `Model.objects.all()` | `consultar()` |
| Filtrar | `Model.objects.filter()` | `consultar(tabela, condicao)` |
| Atualizar | `obj.save()` | `atualizar()` |
| Deletar | `obj.delete()` | `deletar()` |
| Contar | `Model.objects.count()` | `contar()` |

---

**Desenvolvido para Blaze Framework v1.1**
