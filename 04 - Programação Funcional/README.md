# Programação Funcional

## 01. Trabalhando com Funções

Escreva uma função que recebe dois números inteiros e retorna o maior deles.

**Objetivo:**  

- Aprender a definir e usar funções.  
- Praticar o retorno de valores.

**Requisitos:**  

1. Crie uma função chamada `maior` que receba dois inteiros (`i32`).  
2. Use uma expressão `if` para determinar o maior valor.  
3. Teste a função chamando-a com diferentes pares de números.  

**Exemplo de Entrada/Saída:**  
Entrada: `5, 10`  
Saída: `"O maior número é 10"`

---

## 02. Usando Lifetimes

Implemente uma função que retorna a menor palavra em uma frase usando lifetimes explícitos.

**Objetivo:**  

- Entender como trabalhar com lifetimes em funções que retornam referências.  

**Requisitos:**  

1. Declare uma função `menor_palavra` que recebe uma fatia de string (`&str`) e retorna a menor palavra.  
2. Use lifetimes explícitos para garantir que a referência retornada seja válida.  

**Exemplo de Entrada/Saída:**  
Entrada: `"Rust é fantástico"`  
Saída: `"é"`

---

## 03. Trabalhando com Generics

Implemente uma função que recebe um vetor de qualquer tipo e retorna o primeiro elemento.

**Objetivo:**  

- Aprender a criar funções genéricas.  

**Requisitos:**  

1. A função deve usar `generics` para ser compatível com diferentes tipos.  
2. Retorne uma referência para o primeiro elemento ou `None` se o vetor estiver vazio.  

**Exemplo de Entrada/Saída:**  
Entrada: `[10, 20, 30]`  
Saída: `"Primeiro elemento: 10"`

---

## 04. Uso de Bibliotecas

Usar a biblioteca `text-input` para pedir ao usuário seu nome e exibir uma saudação personalizada.

**Objetivo:**  

- Aprender a adicionar e usar bibliotecas no projeto.

**Instruções:**

1. Adicione a biblioteca `text-input` ao seu `Cargo.toml`.
2. Use a função `text()` para pedir ao usuário seu nome.
3. Exiba uma mensagem de saudação no formato: `"Olá, [nome]! Seja bem-vindo!"`.

---

## 05. Trabalhando com `Option`

Crie uma função que recebe um vetor de números inteiros e retorna o maior número como um `Option<i32>`.

**Objetivo:**  

- Praticar o uso de `Option`.  

**Requisitos:**  

1. A função deve retornar `Some` com o maior valor ou `None` se o vetor estiver vazio.  
2. Teste a função com diferentes entradas.  

**Exemplo de Entrada/Saída:**  
Entrada: `[5, 3, 9, 1]`  
Saída: `"Maior número: 9"`

---
