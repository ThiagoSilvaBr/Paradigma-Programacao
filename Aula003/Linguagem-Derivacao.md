# Gramática do loop `for` em Rust

## 2. Loop `for` em Rust

Em Rust, o loop `for` funciona como um iterador de coleções ou intervalos, e sua gramática básica segue o formato:

```bnf
<for-statement> ::= "for" <identifier> "in" <iterable-expression> "{" <body> "}"

<iterable-expression> ::= <identifier> | <expression>

<body> ::= <statement> | <statement> <body>

<statement> ::= <assignment> | <expression>

<assignment> ::= <identifier> "=" <expression>

<expression> ::= <identifier> | <literal> | <expression> <operator> <expression>

<operator> ::= "+" | "-" | "*" | "/"
```
 | Regra | Significado |
|---|---|
| `<for-statement>` | Representa a estrutura completa do laço for |
| `<identifier>` | Representa um nome de variável |
| `<iterable-expression>` | Representa o objeto ou expressão que será percorrida pelo laço |
| `<body>` | Representa o conjunto de instruções executadas a cada iteração |
| `<statement>` | Representa uma instrução individual, como atribuição ou expressão |
| `<assignment>` | Representa uma atribuição de valor a uma variável |
| `<expression>` | Representa uma expressão no geral |
| `<operator>` | Representa operadores aritméticos |

