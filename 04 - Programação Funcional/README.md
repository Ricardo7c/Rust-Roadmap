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

## 06. Conversão de String para Inteiro

**Descrição**: Crie uma função que tenta converter uma string em um número inteiro. Se a conversão for bem-sucedida, a função deve retornar o número, caso contrário, ela deve retornar um `Result` com um erro explicando a falha.

- **Entrada**: Uma `String`.
- **Saída**: Um `Result<i32, String>`, onde o valor de sucesso é o número convertido, e o valor de erro é uma mensagem dizendo que a conversão falhou.

---

## 07. Busca em Lista de Nomes

**Descrição**: Crie uma função que recebe uma lista de nomes (um `Vec<String>`) e um nome para buscar. A função deve retornar a posição do nome na lista, ou `None` caso o nome não seja encontrado.

- **Entrada**: Um `Vec<String>` e um `String`.
- **Saída**: Um `Option<usize>`, que indica a posição do nome na lista ou `None` se o nome não estiver presente.

---

## 08. Divisão Segura

**Descrição**: Implemente uma função que realiza uma divisão segura entre dois números inteiros. A função deve retornar um `Result`, onde o valor de sucesso é o resultado da divisão e o valor de erro é uma mensagem indicando que não é possível dividir por zero.

- **Entrada**: Dois números inteiros.
- **Saída**: Um `Result<i32, String>`, onde o valor de sucesso é o resultado da divisão e o valor de erro é uma mensagem de erro.

---

## 09. Acesso a Elementos de uma Matriz

**Descrição**: Crie uma função que tenta acessar um elemento de uma matriz (um `Vec<Vec<i32>>`). A função deve retornar um `Option<i32>`: o valor do elemento se ele existir, ou `None` se os índices fornecidos estiverem fora do limite da matriz.

- **Entrada**: Uma matriz (um `Vec<Vec<i32>>`) e dois índices (linha e coluna).
- **Saída**: Um `Option<i32>`, que representa o valor do elemento ou `None` caso os índices sejam inválidos.

---

## 10. Validação de Idade

**Descrição**: Crie uma função que recebe a idade de uma pessoa como um número inteiro e retorna um `Result<String, String>`. Se a idade for maior ou igual a 18, a função deve retornar uma mensagem de sucesso dizendo que a pessoa é maior de idade. Se a idade for menor que 18, a função deve retornar uma mensagem de erro informando que a pessoa ainda é menor de idade.

## Entrada

- Um número inteiro representando a idade.

## Saída

- Um `Result<String, String>`, onde:
  - O valor de sucesso (`Ok`) é uma mensagem `"Maior de idade"`.
  - O valor de erro (`Err`) é uma mensagem `"Menor de idade"`.

---
