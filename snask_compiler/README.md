# 🐍 Snask: Uma Linguagem de Programação Moderna e de Alto Desempenho

<!-- BADGES (Simulação de Badges Profissionais) -->
<!--
[![Versão](https://img.shields.io/badge/Versão-v0.2.0-blue.svg)](https://github.com/Davivilasdev/Snask/releases/tag/v0.2.0)
[![Licença](https://img.shields.io/badge/Licença-MIT-green.svg)](LICENSE)
[![Construído com](https://img.shields.io/badge/Construído%20com-Rust-orange.svg)](https://www.rust-lang.org/)
-->

**Snask** é uma linguagem de programação **dinâmica** e **interpretada**, projetada para oferecer uma experiência de desenvolvimento **simples** e **poderosa**, combinada com o **alto desempenho** de uma *runtime* construída em **Rust**.

Seu objetivo é ser a ponte perfeita entre a facilidade de uso de linguagens de *scripting* e a robustez de sistemas de baixo nível.

## ✨ Principais Recursos (v0.2.0)

A versão `v0.2.0` traz um conjunto robusto de funcionalidades que a posicionam como uma ferramenta séria para desenvolvimento:

| Recurso | Descrição |
| :--- | :--- |
| **🚀 Performance Rust** | O interpretador é compilado em Rust, garantindo velocidade e segurança de memória. |
| **📚 Biblioteca Padrão Rica** | Mais de **70 funções** nativas para I/O, HTTP, JSON, matemática e manipulação de coleções. |
| **🔧 Gerenciador de Pacotes Automático** | Instale módulos Rust da comunidade com um único comando, estendendo a linguagem de forma nativa e automática. |
| **💻 REPL Interativo** | Um *shell* profissional com histórico de comandos e comandos especiais para prototipagem rápida. |
| **🎨 Diagnósticos de Erro** | Mensagens de erro coloridas e explicativas, inspiradas nas melhores práticas de linguagens como Rust e Elm. |
| **💡 Sintaxe Moderna** | Suporte a tipagem opcional, funções de primeira classe, estruturas de dados ricas (`list`, `dict`) e controle de fluxo intuitivo. |

## 📦 Instalação e Configuração

Snask é construído em Rust, o que torna a compilação e o uso diretos.

### Pré-requisitos

Certifique-se de ter o **Rust** instalado em seu sistema.

### Compilação

Para compilar o interpretador a partir do código-fonte:

```bash
# Navegue até o diretório do compilador
cd snask_compiler

# Compile o projeto em modo de release para otimização
cargo build --release
```

O executável final será gerado em `target/release/snask` (ou `target/release/snask.exe` no Windows).

### Execução

#### 1\. Modo Interativo (REPL)

Use o REPL para testar comandos e prototipar rapidamente:

```bash
./target/release/snask repl
```

#### 2\. Interpretar Arquivos

Para executar um arquivo de código-fonte Snask (extensão `.snask`):

```bash
./target/release/snask interpret meu_programa.snask
```

## 📖 Visão Geral da Linguagem

A sintaxe do Snask é projetada para ser familiar a desenvolvedores de Python e C, mas com a clareza de linguagens modernas.

### Variáveis e Tipos

Snask suporta três tipos de declaração de variáveis e tipagem dinâmica.

| Declaração | Exemplo | Descrição |
| :--- | :--- | :--- |
| `let` | `let nome = "Snask";` | Variável **imutável** (preferencial). |
| `mut` | `mut contador = 0;` | Variável **mutável**. |
| `const` | `const PI = 3.14159;` | Constante de valor fixo. |

**Tipos de Dados Principais:**

*   `float`: Números (inteiros e decimais).
*   `str`: Textos.
*   `bool`: Booleanos (`true`, `false`).
*   `list`: Listas dinâmicas.
*   `dict`: Dicionários (mapas de chave-valor).

### Funções

As funções suportam tipagem opcional para argumentos e retorno, promovendo código mais legível e robusto.

```snask
// Função com tipagem opcional
fun somar(a: float, b: float): float {
    return a + b;
}

// Chamada da função
print(somar(10, 20)); // Saída: 30
```

### Controle de Fluxo

O controle de fluxo utiliza a sintaxe `if/elif/else` e blocos `{}`.

```snask
if x > 10 {
    print("Maior que 10");
} elif x == 10 {
    print("Igual a 10");
} else {
    print("Menor que 10");
}

// Loop de iteração
for item in [1, 2, 3] {
    print(item);
}
```

## 🛠️ Biblioteca Padrão (Standard Library)

A biblioteca padrão é organizada em módulos e acessível globalmente.

| Módulo | Foco | Funções Chave (Exemplos) |
| :--- | :--- | :--- |
| **Math** | Operações matemáticas | `abs()`, `sqrt()`, `sin()`, `min()`, `PI` |
| **String** | Manipulação de texto | `len()`, `upper()`, `split()`, `join()`, `format()` |
| **Collections** | Listas e Dicionários | `range()`, `sort()`, `reverse()`, `unique()` |
| **IO** | Entrada/Saída e Arquivos | `read_file()`, `write_file()`, `exists()`, `delete()` |
| **HTTP & JSON** | Conectividade Web | `http_get()`, `json_parse()`, `json_stringify()` |
| **System** | Utilitários do Sistema | `time()`, `sleep()`, `exit()`, `args()`, `platform()` |

## 🚀 Gerenciador de Pacotes (Extensão Nativa)

Um dos recursos mais poderosos do Snask é a capacidade de estender a linguagem com módulos nativos escritos em Rust, de forma totalmente automatizada.

### Como Funciona

O comando `snask install <nome_do_modulo>` cuida de todo o processo:

1.  **Download** do código-fonte Rust do módulo.
2.  **Integração** automática no projeto Snask.
3.  **Recompilação** do interpretador.

**Resultado:** O novo módulo e suas funções são imediatamente adicionados à biblioteca padrão, prontos para uso.

```bash
# Exemplo de instalação
$ snask install advanced_math

# ... Processo automático de download, integração e compilação ...

✅ INSTALAÇÃO COMPLETA!
# As funções do módulo 'advanced_math' estão agora disponíveis globalmente.
```

## 🗺️ Roadmap Futuro

O desenvolvimento do Snask continua focado em aprimorar a experiência do desenvolvedor:

*   **String Interpolation**: `print("Olá {nome}")`
*   **Pattern Matching**: Estruturas de controle avançadas.
*   **Null Safety**: Tipos opcionais (`int?`) para maior segurança.
*   **Formatter**: Ferramenta `snask fmt` para padronização de código.

## 📄 Licença

Este projeto está licenciado sob a **Licença MIT**. Sinta-se à vontade para inspecionar, modificar e distribuir.

---
*Desenvolvido por Davivilasdev*
