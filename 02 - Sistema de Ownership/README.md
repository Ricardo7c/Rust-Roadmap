# **O Sistema de Ownership**

---

## **01. Entendendo Ownership e Borrowing**

Crie um programa que receba uma string do usuário e determine se ela é um palíndromo. Use o sistema de **ownership** para passar a string entre funções e evite erros de compilação relacionados ao uso após mover a propriedade.

**Objetivo:**  

- Praticar a passagem de ownership e borrowing.  
- Garantir que a string original não seja perdida.  

**Requisitos:**  

1. Crie uma função `is_palindrome` que recebe uma **referência** para a string.  
2. Evite mover a propriedade ao longo do código.  

**Exemplo de Entrada/Saída:**  
Entrada: `"radar"`  
Saída: `"A string 'radar' é um palíndromo."`

---

## **02. Borrowing Mutável**

Escreva um programa que construa uma frase a partir de palavras individuais. Use **referências mutáveis** para modificar o conteúdo de um `String`.

**Objetivo:**  

- Praticar a regra de uma única referência mutável por vez.  

**Requisitos:**  

1. Inicialize uma `String` vazia.  
2. Adicione palavras ao final da string usando uma função `adicionar_palavra`.  
3. Use apenas **referências mutáveis** para alterar a string.  

**Exemplo de Saída:**  
Saída: `"Frase final: 'Olá mundo bonito'"`

---

## **03. Lifetimes**

Implemente uma função que retorna a maior palavra em uma fatia de string.

**Objetivo:**  

- Trabalhar com lifetimes explícitos.  

**Requisitos:**  

1. Escreva uma função `maior_palavra` que recebe uma **fatia de string** (`&str`).  
2. Retorne a referência da maior palavra encontrada.  

**Exemplo de Entrada/Saída:**  
Entrada: `"Rust é incrível"`  
Saída: `"incrível"`

---

## **04. Fatiamento**

Dado um vetor de números inteiros, divida o vetor em duas partes com base em um índice. Retorne ambas as partes usando slices.

**Objetivo:**  

- Praticar o uso de slices em vetores.  

**Requisitos:**  

1. Crie uma função `dividir_vetor` que recebe um vetor e um índice.  
2. Retorne duas slices: a parte antes do índice e a parte depois.  

**Exemplo de Entrada/Saída:**  
Entrada: `[1, 2, 3, 4, 5]`, índice = 3  
Saída: `Parte 1: [1, 2, 3], Parte 2: [4, 5]`

---

## **05. Unsafe**

Implemente uma função que copia manualmente os bytes de uma slice de inteiros para outra usando código **unsafe**.

**Objetivo:**  

- Entender o uso de código **unsafe** e manipulação de ponteiros.  

**Requisitos:**  

1. Use ponteiros e o operador `*` para copiar elementos.  
2. Garanta que o programa seja seguro (não cause `segmentation fault`).  

**Exemplo de Entrada/Saída:**  
Entrada: `[1, 2, 3]`  
Saída: `[1, 2, 3] copiado com sucesso!`

---

## **06. Regras de Ownership**

Escreva um programa que simula um estoque. Cada item possui um nome e uma quantidade. Use um `HashMap` para armazenar os itens e suas quantidades.

**Objetivo:**  

- Consolidar as regras de ownership no uso de coleções.  

**Requisitos:**  

1. Crie uma função `adicionar_item` que recebe o nome do item e o valor a ser adicionado.  
2. Use borrowing para evitar mover o `HashMap`.  

**Exemplo de Saída:**  
Entrada: `"Maçã, 10"`  
Saída: `"Estoque: {'Maçã': 10}"`

---

## **07. Misturando Conceitos**

Desenvolva um programa que calcula o comprimento médio das palavras em uma frase, excluindo os espaços.

**Objetivo:**  

- Trabalhar com referências, slices e lifetimes.  

**Requisitos:**  

1. Use uma função para dividir a frase em palavras.  
2. Retorne o comprimento médio como `f64`.  

**Exemplo de Entrada/Saída:**  
Entrada: `"Rust é divertido"`  
Saída: `Comprimento médio: 4.33`

---

## Dicas para Resolver

1. **Compilador é seu amigo**: Se algo não funcionar, leia atentamente as mensagens de erro do Rust. Elas normalmente explicam como resolver o problema.  
2. **Documentação**: Consulte [a documentação do Rust](https://doc.rust-lang.org/) para se aprofundar em conceitos específicos.  
3. **Use `cargo check`**: Teste frequentemente seu código para validar as regras de ownership.  

Esses exercícios abordam os conceitos principais que você precisa entender para dominar o sistema de ownership em Rust!
