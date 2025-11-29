# 🚀 Snask v0.2 - Guia de Início Rápido

## Instalação

```bash
cd c:\Users\Davi\Desktop\codes\Snask\snask_compiler
cargo build --release
```

O executável estará em: `target\release\snask.exe`

## Comandos Disponíveis

### 1. Interpretar Arquivo

```bash
snask interpret arquivo.snask
```

### 2. REPL Interativo

```bash
snask repl
```

## Exemplos de Código

### Hello World

```snask
print("Olá, Snask 2.0!");
```

### Usando a Stdlib

```snask
// Math
print("Raiz de 16:", sqrt(16));
print("2 elevado a 8:", pow(2, 8));
print("PI:", PI);

// String
let texto = "  Snask é incrível!  ";
print(upper(trim(texto)));

// Collections
let numeros = [5, 2, 8, 1, 9];
print("Ordenado:", sort(numeros));
print("Range:", range(10));

// I/O
write_file("teste.txt", "Olá do Snask!");
let conteudo = read_file("teste.txt");
print(conteudo);

// System
print("Plataforma:", platform());
print("Timestamp:", time());
```

### Funções

```snask
fun fibonacci(n: float): float {
    if n <= 1 {
        return n;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);
}

print("Fibonacci(10):", fibonacci(10));
```

## Biblioteca Padrão Completa

### Math (24 funções)
`abs`, `floor`, `ceil`, `round`, `pow`, `sqrt`, `log`, `log10`, `exp`, `min`, `max`, `sin`, `cos`, `tan`, `asin`, `acos`, `atan`, `atan2`, `PI`, `E`, `TAU`

### String (15 funções)
`len`, `upper`, `lower`, `trim`, `split`, `join`, `replace`, `contains`, `starts_with`, `ends_with`, `chars`, `substring`, `format`

### Collections (11 funções)
`map`, `filter`, `reduce`, `find`, `any`, `all`, `reverse`, `sort`, `unique`, `flatten`, `range`

### IO (9 funções)
`read_file`, `write_file`, `append_file`, `exists`, `delete`, `read_dir`, `is_file`, `is_dir`, `create_dir`

### HTTP (2 funções)
`http_get`, `http_post`

### JSON (3 funções)
`json_parse`, `json_stringify`, `json_stringify_pretty`

### System (9 funções)
`time`, `sleep`, `exit`, `args`, `env`, `set_env`, `cwd`, `platform`, `arch`

## REPL - Comandos Especiais

- `help` - Mostrar ajuda
- `history` - Ver histórico de comandos
- `clear` - Limpar histórico
- `exit` ou `quit` - Sair

## Próximos Passos

Explore a documentação completa em [`walkthrough.md`](file:///C:/Users/Davi/.gemini/antigravity/brain/fee98215-9716-48a9-ab0c-2494c12bd849/walkthrough.md)!
