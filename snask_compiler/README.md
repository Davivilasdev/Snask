# 🐍 Snask Programming Language

**Versão Atual: v0.2.0**

Bem-vindo ao **Snask**, uma linguagem de programação moderna, dinâmica e interpretada, projetada para ser simples, poderosa e com uma experiência de desenvolvimento incrível.

---

## 🚀 Destaques da Versão v0.2.0

- **Biblioteca Padrão Completa**: Mais de **70 funções** nativas para matemática, strings, coleções, I/O, HTTP e JSON.
- **REPL Interativo**: Um shell interativo profissional com histórico e comandos especiais.
- **Diagnósticos Bonitos**: Mensagens de erro coloridas e explicativas, inspiradas em Rust e Elm.
- **Sintaxe Moderna**: Tipagem opcional, funções de primeira classe e estruturas de dados ricas.

---

## 📦 Instalação e Uso

### Pré-requisitos
- Rust (para compilar o interpretador)

### Compilando

```bash
cd snask_compiler
cargo build --release
```

O executável será gerado em `target/release/snask.exe`.

### Executando

#### 1. REPL (Modo Interativo)
Para experimentar o Snask rapidamente:

```bash
snask repl
```

#### 2. Interpretador (Executar Arquivos)
Para rodar um arquivo `.snask`:

```bash
snask interpret meu_programa.snask
```

---

## 📚 Sintaxe Básica

### Variáveis

```snask
let nome = "Snask";      // Imutável
mut contador = 0;        // Mutável
const PI = 3.14159;      // Constante
```

### Tipos de Dados

- **float**: Números (inteiros e decimais). Ex: `42`, `3.14`
- **str**: Textos. Ex: `"Olá"`
- **bool**: Booleanos. Ex: `true`, `false`
- **list**: Listas. Ex: `[1, 2, 3]`
- **dict**: Dicionários. Ex: `{"a": 1, "b": 2}`

### Controle de Fluxo

```snask
if x > 10 {
    print("Maior que 10");
} elif x == 10 {
    print("Igual a 10");
} else {
    print("Menor que 10");
}

while i < 5 {
    print(i);
    i = i + 1;
}

for item in [1, 2, 3] {
    print(item);
}
```

### Funções

```snask
fun somar(a: float, b: float): float {
    return a + b;
}

print(somar(10, 20)); // 30
```

---

## 🛠️ Biblioteca Padrão (Standard Library)

O Snask v0.2.0 vem com uma biblioteca padrão rica e organizada em módulos.

### 📐 Math (Matemática)
Funções matemáticas essenciais.

- `abs(n)`, `floor(n)`, `ceil(n)`, `round(n)`
- `pow(base, exp)`, `sqrt(n)`
- `sin(rad)`, `cos(rad)`, `tan(rad)`
- `min(a, b...)`, `max(a, b...)`
- Constantes: `PI`, `E`, `TAU`

### 🔤 String (Texto)
Manipulação de strings.

- `len(s)`: Tamanho da string
- `upper(s)`, `lower(s)`: Maiúsculas/Minúsculas
- `trim(s)`: Remove espaços
- `split(s, sep)`: Divide string em lista
- `join(list, sep)`: Junta lista em string
- `replace(s, old, new)`: Substitui texto
- `contains(s, sub)`, `starts_with(s, sub)`
- `format(template, args...)`: Interpolação básica

### 📦 Collections (Coleções)
Manipulação de listas e dicionários.

- `range(n)`: Cria lista de 0 a n-1
- `sort(list)`: Ordena lista
- `reverse(list)`: Inverte lista
- `unique(list)`: Remove duplicatas
- `flatten(list)`: Achata listas aninhadas
- `map`, `filter`, `reduce` (em desenvolvimento)

### 📁 IO (Entrada/Saída)
Operações de arquivo e sistema de arquivos.

- `read_file(path)`: Lê arquivo para string
- `write_file(path, content)`: Escreve string em arquivo
- `append_file(path, content)`: Adiciona ao final
- `exists(path)`: Verifica se existe
- `delete(path)`: Remove arquivo
- `read_dir(path)`: Lista diretório

### 🌐 HTTP & JSON
Conectividade e dados.

- `http_get(url)`: Faz requisição GET
- `http_post(url, body)`: Faz requisição POST
- `json_parse(str)`: Converte JSON para objeto
- `json_stringify(obj)`: Converte objeto para JSON

### ⚙️ System
Utilitários do sistema.

- `time()`: Timestamp atual
- `sleep(ms)`: Pausa execução
- `exit(code)`: Encerra programa
- `args()`: Argumentos de linha de comando
- `platform()`: Sistema operacional (windows, linux...)

---

## 💻 Exemplo Completo

```snask
// Calculadora de Fatorial
fun fatorial(n: float): float {
    if n <= 1 {
        return 1;
    }
    return n * fatorial(n - 1);
}

print("Calculando fatoriais...");
let numeros = range(1, 6); // [1, 2, 3, 4, 5]

for n in numeros {
    let fat = fatorial(n);
    print("Fatorial de", n, "é", fat);
}

// Manipulação de Arquivos
let resultado = "Resultado final: " + format("{}", fatorial(5));
write_file("resultado.txt", resultado);
print("Resultado salvo em resultado.txt");
print(math::sqrt(16));  // 4
```

---

## 📦 Gerenciador de Pacotes

Snask possui um gerenciador de pacotes **totalmente automático** que permite instalar módulos Rust da comunidade e estendê-los à biblioteca padrão.

### Instalando Módulos

```bash
# Instalar um módulo do registro oficial
snask install <nome_do_modulo>

# Exemplo: instalar módulo adicional
snask install advanced_math
```

### Processo Automático

Quando você executa `snask install <modulo>`, o sistema automaticamente:

1. ✅ **Download**: Baixa o módulo `.rs` do repositório [SnaskPackages](https://github.com/Davivilasdev/SnaskPackages)
2. ✅ **Integração**: Salva em `src/stdlib/`
3. ✅ **Declaração**: Adiciona `pub mod <modulo>;` em `src/stdlib.rs`
4. ✅ **Registro**: Adiciona `<modulo>::create_module()` em `register_stdlib()`
5. ✅ **Compilação**: Executa `cargo build --release` automaticamente

**Tudo isso acontece com um único comando!** 🚀

### Exemplo de Uso

```bash
# Instalar módulo
$ snask install advanced_math

📦 Baixando módulo Rust 'advanced_math' de https://...
✓ Módulo 'advanced_math' baixado para src/stdlib/advanced_math.rs

🔧 Integrando módulo automaticamente...
✓ Módulo integrado em src/stdlib.rs

🔨 Recompilando Snask...
✓ Compilação concluída com sucesso!

✅ INSTALAÇÃO COMPLETA!
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
O módulo 'advanced_math' está pronto para uso!
As funções do módulo estão disponíveis globalmente em seus programas Snask.
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

# Agora você pode usar imediatamente!
$ snask repl
> funcao_do_modulo(10);
```

### Repositório Oficial

Módulos oficiais estão disponíveis em: [SnaskPackages](https://github.com/Davivilasdev/SnaskPackages)

Cada módulo `.rs` deve exportar uma função `create_module()` que retorna um objeto com as funções do módulo.

---

## 🗺️ Roadmap

O futuro do Snask é brilhante! Próximas funcionalidades planejadas:

- [ ] **String Interpolation**: `print("Olá {nome}")`
- [ ] **Pattern Matching**: `match x { ... }`
- [ ] **Null Safety**: Tipos opcionais `int?`
- [ ] **Formatter**: `snask fmt`
- [ ] **Gerenciador de Pacotes**: `snask install`

---

## 📄 Licença

Snask é open-source e distribuído sob a licença MIT. Divirta-se codando!
