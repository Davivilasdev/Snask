# 📘 Guia Completo de Snask para Iniciantes

**Bem-vindo ao Snask!** Este guia foi criado para ajudar iniciantes a aprender Snask, comparando-o com Python e C para facilitar o entendimento.

---

## 📑 Índice

1. [O que é Snask?](#o-que-é-snask)
2. [Instalação](#instalação)
3. [Primeiros Passos](#primeiros-passos)
4. [Sintaxe Básica](#sintaxe-básica)
5. [Comparação com Python](#comparação-com-python)
6. [Comparação com C](#comparação-com-c)
7. [Tipos de Dados](#tipos-de-dados)
8. [Controle de Fluxo](#controle-de-fluxo)
9. [Funções](#funções)
10. [Estruturas de Dados](#estruturas-de-dados)
11. [Biblioteca Padrão](#biblioteca-padrão)
12. [Exemplos Práticos](#exemplos-práticos)
13. [Padrões Comuns](#padrões-comuns)
14. [Solução de Problemas](#solução-de-problemas)

---

## 🐍 O que é Snask?

**Snask** é uma linguagem de programação moderna, dinâmica e interpretada, projetada para ser:
- **Simples**: Sintaxe clara e fácil de aprender
- **Poderosa**: Biblioteca padrão rica com mais de 70 funções
- **Rápida**: Compilada em Rust para máximo desempenho
- **Moderna**: Recursos como tipagem opcional e funções de primeira classe

---

## 📦 Instalação

### Pré-requisitos
- Rust (para compilar o interpretador)
- No Windows, para que o módulo `blaze_db` (que usa SQLite) compile corretamente, o `rusqlite` pode exigir a feature `bundled`. Isso geralmente é resolvido automaticamente pelo `Cargo`, mas é bom estar ciente.

### Compilando o Snask

```bash
cd snask_compiler
cargo build --release
```

O executável será gerado em `target/release/snask.exe`.

### Adicionando ao PATH (Opcional)

Para usar `snask` de qualquer lugar, adicione o diretório ao PATH do sistema.

---

## 🚀 Primeiros Passos

### 1. Hello World

Crie um arquivo `hello.snask`:

```snask
print("Olá, Mundo!");
```

Execute:

```bash
snask interpret hello.snask
```

### 2. REPL Interativo

Para experimentar código rapidamente:

```bash
snask repl
```

No REPL, você pode digitar comandos e ver os resultados imediatamente:

```snask
> print("Testando Snask!");
Testando Snask!

> let x = 10;
> print(x * 2);
20
```

---

## 📝 Sintaxe Básica

### Comentários

```snask
// Comentário de uma linha
// Não há comentários de múltiplas linhas ainda
```

### Variáveis

```snask
let nome = "João";        // Imutável (não pode ser alterada)
mut idade = 25;           // Mutável (pode ser alterada)
const PI = 3.14159;       // Constante (nunca muda)

idade = 26;               // OK - variável mutável
// nome = "Maria";        // ERRO - variável imutável
```

### Impressão

```snask
print("Texto simples");
print("Valor:", 42);
print("Múltiplos", "valores", "separados");
```

---

## 🐍 Comparação com Python

### Variáveis

| Python | Snask |
|--------|-------|
| `x = 10` | `let x = 10;` ou `mut x = 10;` |
| `PI = 3.14` | `const PI = 3.14;` |

**Diferenças:**
- Snask usa `let` para imutáveis e `mut` para mutáveis
- Snask requer ponto-e-vírgula `;` no final das instruções
- Snask tem `const` para constantes verdadeiras

### Impressão

| Python | Snask |
|--------|-------|
| `print("Olá")` | `print("Olá");` |
| `print(f"x = {x}")` | `print("x =", x);` |

### Condicionais

**Python:**
```python
if x > 10:
    print("Maior")
elif x == 10:
    print("Igual")
else:
    print("Menor")
```

**Snask:**
```snask
if x > 10 {
    print("Maior");
} elif x == 10 {
    print("Igual");
} else {
    print("Menor");
}
```

**Diferenças:**
- Snask usa `{}` em vez de indentação
- Snask usa `elif` (igual ao Python)

### Loops

**Python:**
```python
# While
while i < 5:
    print(i)
    i += 1

# For
for item in [1, 2, 3]:
    print(item)
```

**Snask:**
```snask
// While
while i < 5 {
    print(i);
    i = i + 1;
}

// For
for item in [1, 2, 3] {
    print(item);
}
```

**Diferenças:**
- Snask não tem `+=`, use `i = i + 1`
- Snask usa `{}` em vez de indentação

### Funções

**Python:**
```python
def somar(a, b):
    return a + b

resultado = somar(5, 3)
```

**Snask:**
```snask
fun somar(a: float, b: float): float {
    return a + b;
}

let resultado = somar(5, 3);
```

**Diferenças:**
- Snask usa `fun` em vez de `def`
- Snask tem tipagem opcional (`: float`)
- Snask requer `{}` e `;`

### Listas

**Python:**
```python
numeros = [1, 2, 3, 4, 5]
numeros.append(6)
print(numeros[0])
```

**Snask:**
```snask
let numeros = [1, 2, 3, 4, 5];
push(numeros, 6);
print(numeros[0]);
```

### Dicionários

**Python:**
```python
pessoa = {"nome": "João", "idade": 25}
print(pessoa["nome"])
pessoa["cidade"] = "São Paulo"
```

**Snask:**
```snask
let pessoa = {nome: "João", idade: 25};
print(pessoa[nome]);
pessoa[cidade] = "São Paulo";
```

---

## 🔧 Comparação com C

### Variáveis

| C | Snask |
|---|-------|
| `int x = 10;` | `let x = 10;` |
| `const float PI = 3.14;` | `const PI = 3.14;` |
| `char nome[] = "João";` | `let nome = "João";` |

**Diferenças:**
- Snask não requer declaração de tipo (inferência automática)
- Snask é dinamicamente tipado
- Snask não tem ponteiros

### Impressão

| C | Snask |
|---|-------|
| `printf("Olá\n");` | `print("Olá");` |
| `printf("x = %d\n", x);` | `print("x =", x);` |

### Condicionais

**C:**
```c
if (x > 10) {
    printf("Maior\n");
} else if (x == 10) {
    printf("Igual\n");
} else {
    printf("Menor\n");
}
```

**Snask:**
```snask
if x > 10 {
    print("Maior");
} elif x == 10 {
    print("Igual");
} else {
    print("Menor");
}
```

**Diferenças:**
- Snask não precisa de `()` ao redor da condição
- Snask usa `elif` em vez de `else if`

### Loops

**C:**
```c
// While
while (i < 5) {
    printf("%d\n", i);
    i++;
}

// For
for (int i = 0; i < 5; i++) {
    printf("%d\n", i);
}
```

**Snask:**
```snask
// While
while i < 5 {
    print(i);
    i = i + 1;
}

// For (estilo Python)
for i in range(5) {
    print(i);
}
```

**Diferenças:**
- Snask não precisa de `()` ao redor da condição
- Snask não tem `for` tradicional de C, usa `for-in`
- Snask não tem `i++`, use `i = i + 1`

### Funções

**C:**
```c
int somar(int a, int b) {
    return a + b;
}

int resultado = somar(5, 3);
```

**Snask:**
```snask
fun somar(a: float, b: float): float {
    return a + b;
}

let resultado = somar(5, 3);
```

**Diferenças:**
- Snask usa `fun` em vez do tipo de retorno antes do nome
- Snask usa `:` para tipos
- Snask não requer declaração de tipo de variável (`let` vs `int`)

### Arrays

**C:**
```c
int numeros[] = {1, 2, 3, 4, 5};
printf("%d\n", numeros[0]);
```

**Snask:**
```snask
let numeros = [1, 2, 3, 4, 5];
print(numeros[0]);
```

**Diferenças:**
- Snask tem listas dinâmicas (como Python)
- Snask não requer tamanho fixo
- Snask pode misturar tipos na mesma lista

---

## 📊 Tipos de Dados

Snask tem 6 tipos principais:

### 1. Números (float)

```snask
let inteiro = 42;
let decimal = 3.14;
let negativo = -10;
let cientifico = 1.5e10;
```

**Nota:** Todos os números em Snask são `float` (ponto flutuante de 64 bits).

### 2. Strings (str)

```snask
let nome = "João";
let frase = "Olá, mundo!";
let vazio = "";
```

### 3. Booleanos (bool)

```snask
let verdadeiro = true;
let falso = false;
```

### 4. Listas (list)

```snask
let vazia = [];
let numeros = [1, 2, 3, 4, 5];
let mista = [1, "texto", true, [1, 2]];  // Pode misturar tipos
```

### 5. Dicionários (dict)

```snask
let vazio = {};
let pessoa = {
    nome: "João",
    idade: 25,
    ativo: true
};
```

### 6. Nil

```snask
let nulo = nil;  // Equivalente a None (Python) ou NULL (C)
```

---

## 🔀 Controle de Fluxo

### If / Elif / Else

```snask
let nota = 85;

if nota >= 90 {
    print("Excelente!");
} elif nota >= 70 {
    print("Bom!");
} elif nota >= 50 {
    print("Regular");
} else {
    print("Precisa melhorar");
}
```

### While

```snask
mut contador = 0;

while contador < 5 {
    print("Contador:", contador);
    contador = contador + 1;
}
```

### For-In

```snask
// Iterar sobre lista
for numero in [1, 2, 3, 4, 5] {
    print(numero);
}

// Iterar sobre range
for i in range(10) {
    print("Índice:", i);
}

// Iterar sobre string
for char in "Snask" {
    print(char);
}
```

---

## 🔧 Funções

### Declaração Básica

```snask
fun saudar(nome: str): str {
    return "Olá, " + nome + "!";
}

print(saudar("Maria"));  // "Olá, Maria!"
```

### Sem Retorno (Void)

```snask
fun imprimir_linha(texto: str): void {
    print(">>> " + texto);
}

imprimir_linha("Teste");
```

### Múltiplos Parâmetros

```snask
fun calcular_area(largura: float, altura: float): float {
    return largura * altura;
}

let area = calcular_area(10, 5);
print("Área:", area);  // 50
```

### Recursão

```snask
fun fatorial(n: float): float {
    if n <= 1 {
        return 1;
    }
    return n * fatorial(n - 1);
}

print("5! =", fatorial(5));  // 120
```

---

## 📦 Estruturas de Dados

### Listas

```snask
// Criar lista
let frutas = ["maçã", "banana", "laranja"];

// Acessar elemento
print(frutas[0]);  // "maçã"

// Adicionar elemento
push(frutas, "uva");

// Remover último elemento
let ultimo = pop(frutas);

// Tamanho
print("Tamanho:", len(frutas));

// Ordenar
let numeros = [5, 2, 8, 1, 9];
let ordenados = sort(numeros);
print(ordenados);  // [1, 2, 5, 8, 9]

// Reverter
let invertidos = reverse(numeros);
print(invertidos);  // [9, 1, 8, 2, 5]
```

### Dicionários

```snask
// Criar dicionário
let pessoa = {
    nome: "João",
    idade: 25,
    cidade: "São Paulo"
};

// Acessar valor
print(pessoa[nome]);  // "João"

// Adicionar/Modificar
pessoa[profissao] = "Programador";
pessoa[idade] = 26;

// Verificar existência (usando try-catch futuro)
print(pessoa[nome]);
```

---

## 📚 Biblioteca Padrão

Snask vem com uma biblioteca padrão rica. Aqui estão as funções mais úteis:

### 🔢 Matemática

```snask
// Básicas
print(abs(-5));           // 5
print(floor(3.7));        // 3
print(ceil(3.2));         // 4
print(round(3.5));        // 4
print(pow(2, 8));         // 256
print(sqrt(16));          // 4

// Novas funções
print(mod(10, 3));        // 1 (resto da divisão)
print(random());          // Número aleatório entre 0 e 1
print(random_range(1, 10)); // Número aleatório entre 1 e 10
print(clamp(15, 0, 10));  // 10 (limita valor entre min e max)
print(sign(-5));          // -1 (retorna -1, 0 ou 1)
print(deg_to_rad(180));   // 3.14159... (converte graus para radianos)
print(rad_to_deg(3.14));  // 179.9... (converte radianos para graus)

// Trigonometria
print(sin(PI / 2));       // 1
print(cos(0));            // 1
print(tan(PI / 4));       // 1

// Constantes
print(PI);                // 3.14159...
print(E);                 // 2.71828...
print(TAU);               // 6.28318... (2 * PI)

// Min/Max
print(min(5, 3, 8, 1));   // 1
print(max(5, 3, 8, 1));   // 8
```

### 🔤 Strings

```snask
let texto = "  Snask é Incrível!  ";

print(len(texto));                    // 21
print(upper(texto));                  // "  SNASK É INCRÍVEL!  "
print(lower(texto));                  // "  snask é incrível!  "
print(trim(texto));                   // "Snask é Incrível!"

let palavras = split("a,b,c", ",");   // ["a", "b", "c"]
print(join(palavras, " - "));         // "a - b - c"

print(replace("Olá Mundo", "Mundo", "Snask"));  // "Olá Snask"
print(contains("Snask", "ask"));      // true
print(starts_with("Snask", "Sn"));    // true
print(ends_with("Snask", "sk"));      // true

let chars = chars("ABC");             // ["A", "B", "C"]
print(substring("Snask", 0, 3));      // "Sna"
```

### 📦 Coleções

```snask
let numeros = [5, 2, 8, 1, 9, 3];

print(sort(numeros));                 // [1, 2, 3, 5, 8, 9]
print(reverse(numeros));              // [3, 9, 1, 8, 2, 5]
print(unique([1, 2, 2, 3, 3, 3]));    // [1, 2, 3]

let aninhada = [[1, 2], [3, 4]];
print(flatten(aninhada));             // [1, 2, 3, 4]

print(range(5));                      // [0, 1, 2, 3, 4]
print(range(2, 7));                   // [2, 3, 4, 5, 6]
print(range(0, 10, 2));               // [0, 2, 4, 6, 8]
```

### 📁 Entrada/Saída (I/O)

```snask
// Escrever arquivo
write_file("teste.txt", "Olá, Snask!");

// Ler arquivo
let conteudo = read_file("teste.txt");
print(conteudo);

// Adicionar ao arquivo
append_file("teste.txt", "\nNova linha");

// Verificar existência
if exists("teste.txt") {
    print("Arquivo existe!");
}

// Deletar arquivo
delete("teste.txt");

// Listar diretório
let arquivos = read_dir(".");
for arquivo in arquivos {
    print(arquivo);
}

// Verificar tipo
print(is_file("teste.txt"));
print(is_dir("."));

// Criar diretório
create_dir("nova_pasta");
```

### 🌐 HTTP e JSON

```snask
// Requisição HTTP GET
let resposta = http_get("https://api.github.com");
print(resposta);

// Requisição HTTP POST
http_post("https://httpbin.org/post", '{"nome": "João"}');

// JSON
let dados = {nome: "João", idade: 25};
let json_str = json_stringify(dados);
print(json_str);  // {"nome":"João","idade":25}

let parsed = json_parse(json_str);
print(parsed[nome]);  // "João"
```

### ⚙️ Sistema

```snask
// Tempo
print(time());                        // Timestamp atual

// Pausar execução
sleep(1000);                          // Pausa por 1 segundo (1000ms)

// Sair do programa
// exit(0);

// Argumentos de linha de comando
let args = args();
print(args);

// Variáveis de ambiente
let home = env("HOME");
print(home);

set_env("MINHA_VAR", "valor");

// Informações do sistema
print(platform());                    // "windows", "linux", etc.
print(arch());                        // "x86_64", etc.
print(cwd());                         // Diretório atual
```

---

## 💡 Exemplos Práticos

### Exemplo 1: Calculadora Simples

```snask
fun calculadora(a: float, b: float, op: str): float {
    if op == "+" {
        return a + b;
    } elif op == "-" {
        return a - b;
    } elif op == "*" {
        return a * b;
    } elif op == "/" {
        if b == 0 {
            print("Erro: Divisão por zero!");
            return 0;
        }
        return a / b;
    } else {
        print("Operação inválida!");
        return 0;
    }
}

print("10 + 5 =", calculadora(10, 5, "+"));
print("10 - 5 =", calculadora(10, 5, "-"));
print("10 * 5 =", calculadora(10, 5, "*"));
print("10 / 5 =", calculadora(10, 5, "/"));
```

### Exemplo 2: Fibonacci

```snask
fun fibonacci(n: float): float {
    if n <= 1 {
        return n;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);
}

print("Sequência de Fibonacci:");
for i in range(10) {
    print("F(" + format("{}", i) + ") =", fibonacci(i));
}
```

### Exemplo 3: Processamento de Lista

```snask
let numeros = [12, 45, 23, 67, 34, 89, 15];

// Encontrar maior e menor
let maior = max(numeros[0], numeros[1], numeros[2], numeros[3], numeros[4], numeros[5], numeros[6]);
let menor = min(numeros[0], numeros[1], numeros[2], numeros[3], numeros[4], numeros[5], numeros[6]);

print("Maior:", maior);
print("Menor:", menor);

// Ordenar
let ordenados = sort(numeros);
print("Ordenados:", ordenados);

// Calcular média
mut soma = 0;
for num in numeros {
    soma = soma + num;
}
let media = soma / len(numeros);
print("Média:", media);
```

### Exemplo 4: Manipulação de Arquivos

```snask
// Criar arquivo de log
fun adicionar_log(mensagem: str): void {
    let timestamp = time();
    let linha = format("[{}] {}\n", timestamp, mensagem);
    
    if exists("log.txt") {
        append_file("log.txt", linha);
    } else {
        write_file("log.txt", linha);
    }
}

adicionar_log("Aplicação iniciada");
adicionar_log("Processando dados");
adicionar_log("Aplicação finalizada");

// Ler e exibir log
let log = read_file("log.txt");
print("=== LOG ===");
print(log);
```

### Exemplo 5: Jogo de Adivinhação

```snask
fun jogo_adivinhacao(): void {
    let numero_secreto = floor(random_range(1, 101));
    mut tentativas = 0;
    let max_tentativas = 7;
    
    print("=== JOGO DE ADIVINHAÇÃO ===");
    print("Adivinhe o número entre 1 e 100!");
    print("Você tem", max_tentativas, "tentativas.");
    
    while tentativas < max_tentativas {
        tentativas = tentativas + 1;
        
        // Simulando input do usuário (em versão futura)
        let palpite = floor(random_range(1, 101));
        print("\nTentativa", tentativas, "- Seu palpite:", palpite);
        
        if palpite == numero_secreto {
            print("🎉 PARABÉNS! Você acertou em", tentativas, "tentativas!");
            return;
        } elif palpite < numero_secreto {
            print("📈 Muito baixo! Tente um número maior.");
        } else {
            print("📉 Muito alto! Tente um número menor.");
        }
    }
    
    print("\n😞 Suas tentativas acabaram!");
    print("O número era:", numero_secreto);
}

jogo_adivinhacao();
```

---

## 🎯 Padrões Comuns

### 1. Validação de Entrada

```snask
fun validar_idade(idade: float): bool {
    return idade >= 0 && idade <= 150;
}

let idade = 25;
if validar_idade(idade) {
    print("Idade válida!");
} else {
    print("Idade inválida!");
}
```

### 2. Processamento de Strings

```snask
fun processar_nome(nome: str): str {
    let limpo = trim(nome);
    let palavras = split(limpo, " ");
    mut resultado = "";
    
    for palavra in palavras {
        if len(resultado) > 0 {
            resultado = resultado + " ";
        }
        // Capitalizar primeira letra (simulado)
        resultado = resultado + upper(substring(palavra, 0, 1)) + lower(substring(palavra, 1, len(palavra)));
    }
    
    return resultado;
}

print(processar_nome("  jOãO   SiLvA  "));  // "João Silva"
```

### 3. Trabalhar com Listas

```snask
fun filtrar_pares(lista: list): list {
    mut resultado = [];
    
    for num in lista {
        if mod(num, 2) == 0 {
            push(resultado, num);
        }
    }
    
    return resultado;
}

let numeros = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
let pares = filtrar_pares(numeros);
print("Números pares:", pares);  // [2, 4, 6, 8, 10]
```

### 4. Configuração com Dicionários

```snask
let config = {
    debug: true,
    porta: 8080,
    host: "localhost",
    max_conexoes: 100
};

fun obter_config(chave: str, padrao: any): any {
    // Em versão futura com suporte a verificação de chave
    return config[chave];
}

print("Porta:", config[porta]);
print("Debug:", config[debug]);
```

---

## 🐛 Solução de Problemas

### Erro: "Variable not found"

**Problema:** Tentando usar uma variável que não foi declarada.

```snask
print(x);  // ERRO: x não foi declarada
```

**Solução:**

```snask
let x = 10;
print(x);  // OK
```

### Erro: "Cannot assign to immutable variable"

**Problema:** Tentando modificar uma variável imutável.

```snask
let x = 10;
x = 20;  // ERRO: x é imutável
```

**Solução:**

```snask
mut x = 10;
x = 20;  // OK
```

### Erro: "Type mismatch"

**Problema:** Tipos incompatíveis em operações.

```snask
let x = "10";
let y = 5;
print(x + y);  // ERRO: não pode somar string com número
```

**Solução:**

```snask
let x = 10;  // Usar número
let y = 5;
print(x + y);  // OK: 15
```

### Erro: "Function not found"

**Problema:** Chamando uma função que não existe.

```snask
print(minha_funcao());  // ERRO: função não existe
```

**Solução:**

```snask
fun minha_funcao(): float {
    return 42;
}

print(minha_funcao());  // OK
```

### Erro: "Index out of bounds"

**Problema:** Acessando índice que não existe na lista.

```snask
let lista = [1, 2, 3];
print(lista[10]);  // ERRO: índice 10 não existe
```

**Solução:**

```snask
let lista = [1, 2, 3];
if len(lista) > 10 {
    print(lista[10]);
} else {
    print("Índice inválido");
}
```

### Dica: Usar REPL para Testar

Quando tiver dúvidas sobre como algo funciona, use o REPL:

```bash
snask repl
```

```snask
> let x = [1, 2, 3];
> print(x);
[1, 2, 3]
> push(x, 4);
> print(x);
[1, 2, 3, 4]
```

---

## 📖 Recursos Adicionais

### Comandos do REPL

- `help` - Mostrar ajuda
- `history` - Ver histórico de comandos
- `clear` - Limpar histórico
- `exit` ou `quit` - Sair do REPL

### Executar Arquivos

```bash
# Executar arquivo .snask
snask interpret meu_programa.snask

# Ou simplesmente (se configurado)
snask meu_programa.snask
```

### Gerenciador de Pacotes

```bash
# Instalar módulo
snask install nome_do_modulo

# Listar módulos instalados
snask list

# Desinstalar módulo
snask uninstall nome_do_modulo
```

---

## 🎓 Conclusão

Parabéns! Você agora conhece os fundamentos de Snask. Continue praticando e explorando a linguagem.

**Próximos passos:**
1. Experimente os exemplos deste guia
2. Crie seus próprios programas
3. Explore a biblioteca padrão
4. Contribua para o projeto Snask!

**Recursos:**
- [Documentação Completa](README.md)
- [Exemplos](examples/)
- [Repositório GitHub](https://github.com/Davivilasdev/Snask)

---

**Feito com ❤️ pela comunidade Snask**
