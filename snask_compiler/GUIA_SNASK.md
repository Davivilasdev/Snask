# 📘 Guia de Início Rápido da Linguagem Snask

**Bem-vindo ao Snask!** Este guia foi estruturado para fornecer um caminho de aprendizado rápido e intuitivo, cobrindo desde a instalação até os conceitos fundamentais da linguagem.

## 📑 Índice

1.  [O que é Snask?](#1-o-que-é-snask)
2.  [Configuração e Primeiros Passos](#2-configuração-e-primeiros-passos)
    *   [Pré-requisitos e Compilação](#pré-requisitos-e-compilação)
    *   [Hello World e REPL](#hello-world-e-repl)
3.  [Fundamentos da Linguagem](#3-fundamentos-da-linguagem)
    *   [Variáveis: Imutabilidade e Mutabilidade](#variáveis-imutabilidade-e-mutabilidade)
    *   [Tipos de Dados](#tipos-de-dados)
    *   [Impressão e Comentários](#impressão-e-comentários)
4.  [Estruturas de Controle](#4-estruturas-de-controle)
    *   [Condicionais (`if`, `elif`, `else`)](#condicionais-if-elif-else)
    *   [Loops (`while` e `for-in`)](#loops-while-e-for-in)
5.  [Funções](#5-funções)
    *   [Definição e Tipagem Opcional](#definição-e-tipagem-opcional)
6.  [Estruturas de Dados](#6-estruturas-de-dados)
    *   [Listas (`list`)](#listas-list)
    *   [Dicionários (`dict`)](#dicionários-dict)
7.  [Biblioteca Padrão](#7-biblioteca-padrão)
8.  [Snask para Desenvolvedores (Comparativos)](#8-snask-para-desenvolvedores-comparativos)

---

## 1. O que é Snask?

**Snask** é uma linguagem de programação dinâmica, interpretada e de alto desempenho, construída em Rust. Ela combina a **simplicidade de sintaxe** de linguagens como Python com a **velocidade de execução** de uma *runtime* compilada.

Seu design foca em:
*   **Clareza:** Sintaxe limpa e diagnósticos de erro detalhados.
*   **Produtividade:** Biblioteca padrão rica e REPL interativo.
*   **Extensibilidade:** Gerenciador de pacotes que permite estender a linguagem com módulos nativos em Rust.

## 2. Configuração e Primeiros Passos

### Pré-requisitos e Compilação

Snask requer o **Rust** instalado para ser compilado.

1.  **Navegue** até o diretório do compilador:
    ```bash
    cd snask_compiler
    ```
2.  **Compile** o interpretador:
    ```bash
    cargo build --release
    ```
    O executável será gerado em `target/release/snask` (ou `snask.exe`).

### Hello World e REPL

#### Executando um Arquivo

Crie um arquivo chamado `hello.snask`:

```snask
print("Olá, Mundo!");
```

Execute-o usando o interpretador:

```bash
./target/release/snask interpret hello.snask
```

#### Modo Interativo (REPL)

Para testar comandos rapidamente, use o *Read-Eval-Print Loop* (REPL):

```bash
./target/release/snask repl
```

**Exemplo no REPL:**

```
> print("Testando Snask!");
Testando Snask!

> let x = 10;
> print(x * 2);
20
```

## 3. Fundamentos da Linguagem

### Variáveis: Imutabilidade e Mutabilidade

Snask incentiva a **imutabilidade** por padrão, o que ajuda a prevenir erros e torna o código mais seguro.

| Palavra-chave | Propósito | Exemplo |
| :--- | :--- | :--- |
| `let` | **Imutável** (valor não pode ser alterado após a atribuição). **Uso preferencial.** | `let nome = "João";` |
| `mut` | **Mutável** (valor pode ser alterado). | `mut idade = 25;` |
| `const` | **Constante** (valor fixo, usado para valores globais). | `const PI = 3.14159;` |

**Nota:** Todas as instruções em Snask devem ser finalizadas com ponto-e-vírgula (`;`).

### Tipos de Dados

Snask é dinamicamente tipado, mas possui tipos internos bem definidos:

| Tipo | Descrição | Exemplo |
| :--- | :--- | :--- |
| `float` | Números de ponto flutuante (inclui inteiros). | `42`, `3.14`, `-10` |
| `str` | Cadeias de caracteres (texto). | `"Olá Snask"` |
| `bool` | Valores booleanos. | `true`, `false` |
| `list` | Coleção ordenada e mutável de valores. | `[1, "a", true]` |
| `dict` | Coleção de pares chave-valor. | `{nome: "Ana", idade: 30}` |
| `nil` | Representa a ausência de valor (equivalente a `None` em Python). | `let vazio = nil;` |

### Impressão e Comentários

| Ação | Sintaxe Snask |
| :--- | :--- |
| **Imprimir** | `print("Texto simples");` ou `print("Valor:", 42);` |
| **Comentário** | `// Comentário de uma linha` |

## 4. Estruturas de Controle

### Condicionais (`if`, `elif`, `else`)

As estruturas condicionais utilizam blocos `{}` em vez de indentação.

```snask
let nota = 85;

if nota >= 90 {
    print("Excelente!");
} elif nota >= 70 {
    print("Bom!");
} else {
    print("Precisa melhorar");
}
```

### Loops (`while` e `for-in`)

Snask suporta o loop `while` tradicional e o loop `for-in` para iteração sobre coleções.

**Loop `while`:**

```snask
mut contador = 0;

while contador < 5 {
    print(contador);
    contador = contador + 1; // Snask não tem i++
}
```

**Loop `for-in` (Iteração):**

```snask
let lista = [10, 20, 30];

for item in lista {
    print("Valor:", item);
}

// Usando a função range() da Stdlib
for i in range(5) {
    print(i); // Imprime 0, 1, 2, 3, 4
}
```

## 5. Funções

### Definição e Tipagem Opcional

Funções são definidas com a palavra-chave `fun`. A tipagem de argumentos (`: float`) e do retorno (`: float`) é opcional, mas altamente recomendada para clareza.

```snask
// Função com tipagem completa
fun somar(a: float, b: float): float {
    return a + b;
}

// Função sem tipagem (inferência automática)
fun saudar(nome) {
    print("Olá,", nome);
}

let resultado = somar(5, 3); // 8
saudar("Snasker"); // Olá, Snasker
```

## 6. Estruturas de Dados

### Listas (`list`)

Listas são coleções dinâmicas e mutáveis.

```snask
let numeros = [1, 2, 3, 4, 5];

// Acesso por índice
print(numeros[0]); // 1

// Adicionar um elemento (usando função da Stdlib)
push(numeros, 6);
print(numeros); // [1, 2, 3, 4, 5, 6]
```

### Dicionários (`dict`)

Dicionários armazenam dados em pares chave-valor.

```snask
let pessoa = {
    nome: "João",
    idade: 25
};

// Acesso por chave
print(pessoa[nome]); // João

// Adicionar ou modificar um valor
pessoa[cidade] = "São Paulo";
print(pessoa); // {nome: "João", idade: 25, cidade: "São Paulo"}
```

## 7. Biblioteca Padrão

A Biblioteca Padrão (Stdlib) do Snask é rica e organizada em módulos. Você pode acessar todas as funções globalmente.

| Módulo | Foco | Exemplos de Uso |
| :--- | :--- | :--- |
| **Math** | Matemática | `math::sqrt(16)`, `math::PI` |
| **String** | Manipulação de Texto | `len("texto")`, `upper("texto")`, `split("a,b", ",")` |
| **IO** | Arquivos e Sistema | `read_file("data.txt")`, `write_file("out.txt", content)` |
| **HTTP & JSON** | Web | `http_get(url)`, `json_parse(str)` |
| **Collections** | Listas e Dicionários | `range(10)`, `sort(lista)` |

Para uma referência completa de todas as 70+ funções, consulte a **Documentação Oficial da Stdlib**.

## 8. Snask para Desenvolvedores (Comparativos)

Para desenvolvedores que vêm de outras linguagens, esta tabela resume as principais diferenças de sintaxe:

| Conceito | Python | C | Snask |
| :--- | :--- | :--- | :--- |
| **Variável Mutável** | `x = 10` | `int x = 10;` | `mut x = 10;` |
| **Variável Imutável** | (Não nativo) | `const int x = 10;` | `let x = 10;` |
| **Bloco de Código** | Indentação | `{ ... }` | `{ ... }` |
| **Fim de Instrução** | Nova linha | `;` | `;` |
| **Condicional** | `elif` | `else if` | `elif` |
| **Função** | `def nome(args):` | `tipo nome(args)` | `fun nome(args): tipo` |
| **Adicionar à Lista** | `lista.append(x)` | (Manual) | `push(lista, x)` |
| **Loop de Iteração** | `for item in lista:` | (Não nativo) | `for item in lista { ... }` |

---
*Este guia foi refatorado para a versão v0.2.0 do Snask.*
