# Manual da Linguagem de Programação Snask

Bem-vindo ao manual oficial da Snask, uma linguagem de script simples, dinâmica e interpretada.

## Visão Geral

Snask é projetada para ser fácil de aprender e usar. Ela suporta variáveis, tipos de dados primitivos, estruturas de dados como listas e dicionários, fluxo de controle condicional, laços e funções.

## Como Executar um Programa Snask

Para executar um programa Snask, você precisa do interpretador `snask.exe`. Salve seu código em um arquivo com a extensão `.snask` (por exemplo, `meu_programa.snask`) e execute o seguinte comando no seu terminal, a partir do diretório `snask_compiler`:

```bash
\.\\target\\debug\\snask.exe interpret meu_programa.snask
```

---

## Referência da Linguagem

### 1. Comentários

Comentários são de linha única e começam com `//`. Tudo após `//` até o final da linha é ignorado pelo interpretador.

```snask
// Isto é um comentário.
let x = 10; // Isto também é um comentário.
```

### 2. Variáveis e Tipos de Dados

#### Declaração de Variáveis

Existem três maneiras de declarar variáveis:

-   `let`: Cria uma variável imutável (seu valor não pode ser reatribuído).
-   `mut`: Cria uma variável mutável (seu valor pode ser alterado).
-   `const`: Cria uma constante (seu valor não pode ser alterado e deve ser conhecido em tempo de compilação, embora no Snask atual funcione como `let`).

```snask
let message = "Olá";
mut counter = 0;
counter = 1; // Válido

// message = "Tchau"; // Inválido, vai gerar um erro.
```

#### Tipos de Dados

Snask suporta os seguintes tipos de dados, com anotações de tipo opcionais:

-   **float**: Números de ponto flutuante (e inteiros). Ex: `10`, `3.14`.
-   **str**: Sequências de caracteres. Ex: `"Olá, mundo!"`.
-   **bool**: Valores booleanos, `true` ou `false`.
-   **list**: Uma coleção ordenada de valores. Ex: `[1, "dois", true]`.
-   **dict**: Uma coleção de pares chave-valor. Ex: `{"nome": "Snask", "versao": 1.0}`.

### 3. Entrada e Saída

#### `print()`

A função `print()` exibe valores no console. Ela pode aceitar múltiplos argumentos, que serão separados por espaços.

```snask
print("A resposta é:", 42); // Saída: A resposta é: 42
```

#### `input`

A declaração `input` lê uma linha de texto do console e a armazena em uma variável. A anotação de tipo é **obrigatória** e determina como a entrada será tratada.

```snask
print("Qual é o seu nome?");
input name: str;

print("Qual é a sua idade?");
input age: float;

print("Olá,", name, "! Você tem", age, "anos.");
```

### 4. Operadores

-   **Aritméticos**: `+`, `-`, `*`, `/`
-   **Comparação**: `==` (igual), `!=` (diferente), `<` (menor que), `>` (maior que), `<=` (menor ou igual), `>=` (maior ou igual)

### 5. Controle de Fluxo

#### `if / elif / else`

Executa blocos de código condicionalmente.

```snask
let x = 10;
if x > 0 {
    print("x é positivo");
} elif x < 0 {
    print("x é negativo");
} else {
    print("x é zero");
}
```

#### `while`

Executa um bloco de código repetidamente enquanto uma condição for verdadeira.

```snask
mut i = 0;
while i < 3 {
    print(i);
    i = i + 1;
}
// Saída: 0 1 2
```

### 6. Funções

Funções são declaradas com a palavra-chave `fun`. Anotações de tipo para parâmetros e valor de retorno são suportadas e recomendadas.

```snask
fun somar(a: float, b: float): float {
    return a + b;
}

let resultado = somar(5, 3);
print("5 + 3 =", resultado); // Saída: 5 + 3 = 8
```

### 7. Estruturas de Dados

#### Listas (Lists)

-   **Criação**: `let minha_lista = [10, 20, 30];`
-   **Acesso**: `minha_lista[0]` (retorna `10`)
-   **Métodos**:
    -   `.push(valor)`: Adiciona um elemento ao final da lista.
        ```snask
        mut minha_lista = [1];
        minha_lista.push(2);
        print(minha_lista); // Saída: [1, 2]
        ```

#### Dicionários (Dicts)

-   **Criação**: `let meu_dict = {"chave1": "valor1", "outra_chave": 100};
-   **Acesso**: `meu_dict["chave1"]` (retorna `"valor1"`)
-   **Métodos**:
    -   `.set(chave, valor)`: Adiciona ou atualiza um par chave-valor no dicionário.
        ```snask
        mut meu_dict = {"nome": "Snask"};
        meu_dict.set("versao", 1.0);
        print(meu_dict); // Saída: {nome: Snask, versao: 1.0}
        ```

---

## Exemplo Completo de Programa Snask

```snask
// --- Demonstração da Calculadora Snask ---

print("--- Calculadora Snask ---");

// Definição de Funções
fun somar(a: float, b: float): float {
    return a + b;
}

fun dividir(a: float, b: float): float {
    if b == 0 {
        print("Erro: Divisão por zero!");
        return 0;
    }
    return a / b;
}

// Operações Básicas
let num1 = 20;
let num2 = 5;
print(num1, "/", num2, "=", dividir(num1, num2));

// Teste de Entrada do Usuário
print("Digite um número para somar com 10:");
input numero_usuario: float;
let resultado_soma = somar(10, numero_usuario);
print("10 +", numero_usuario, "=", resultado_soma);
```
