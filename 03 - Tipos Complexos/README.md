# Tipos Complexos

---

## **01. Trabalhando com Strings**  

Crie um programa que concatene duas strings e imprima o resultado.  

**Objetivo:**  

- Praticar operações básicas com strings.  

**Tarefa:**  

1. Declare duas strings, como `"Rust"` e `"é incrível"`.  
2. Concatene as strings usando o método `push_str` ou o operador `+`.  
3. Imprima o resultado.  

**Exemplo de Saída:**  

```Powershell
Rust é incrível
```

---

## **02. Acessando Elementos de uma Tupla**  

Declare uma tupla contendo informações sobre uma pessoa: nome (string), idade (inteiro) e altura (float). Acesse e imprima cada elemento individualmente.  

**Objetivo:**  

- Trabalhar com tuplas e acesso posicional.  

**Tarefa:**  

1. Crie uma tupla com os valores: `"Alice"`, `30`, `1.65`.  
2. Imprima cada valor individualmente usando a notação de índice (`tupla.0`, `tupla.1`, etc.).  

```Poweshell
Nome: Alice  
Idade: 30  
Altura: 1.65
```

---

## 03. Somando Elementos de um Array**  

Crie um array com cinco números inteiros e calcule a soma de todos os elementos.  

**Objetivo:**  

- Praticar operações básicas em arrays.  

**Tarefa:**  

1. Declare um array `[1, 2, 3, 4, 5]`.  
2. Use um loop para somar os valores.  
3. Imprima o resultado.  

**Exemplo de Saída:**  

```Powershell
A soma dos elementos é: 15
```

---

## **04. Trabalhando com Vetores**  

Declare um vetor mutável de números inteiros e adicione novos elementos ao final do vetor. Em seguida, remova o último elemento e imprima o vetor resultante.  

**Objetivo:**  

- Praticar criação, adição e remoção de elementos em vetores.  

**Tarefa:**  

1. Crie um vetor vazio mutável.  
2. Adicione os números `10`, `20` e `30` usando o método `push`.  
3. Remova o último número com `pop`.  
4. Imprima o vetor.  

**Exemplo de Saída:**  

```Powershell
Vetor após remoção: [10, 20]
```

---

## **05. Misturando Strings, Tuplas e Vetores**

Crie um vetor de tuplas onde cada tupla representa um produto, contendo o nome do produto (string) e o preço (float). Em seguida, calcule o preço total dos produtos.  

**Objetivo:**

- Trabalhar com tipos compostos e vetores.  

**Tarefa:**  

1. Crie um vetor contendo tuplas como `("Maçã", 2.5)`, `("Banana", 1.5)` e `("Pêra", 3.0)`.  
2. Use um loop para calcular o total.  
3. Imprima o valor total.  

**Exemplo de Saída:**  

```powershell
Preço total: 7.0
```

---

## **06. Ordenando um Array**  

Declare um array de números inteiros e ordene-o em ordem crescente.  

**Objetivo:**  

- Praticar manipulação básica de arrays.  

**Tarefa:**  

1. Declare um array `[3, 1, 4, 1, 5, 9]`.  
2. Converta-o para um vetor com `to_vec()` e use o método `sort`.  
3. Imprima o array ordenado.  

**Exemplo de Saída:**  

```powershell
Array ordenado: [1, 1, 3, 4, 5, 9]
```

---

## **Exercício 7: Filtrando um Vetor**  

Dado um vetor de números inteiros, crie um novo vetor contendo apenas os números pares.  

**Objetivo:**  

- Praticar filtragem em vetores.  

**Tarefa:**  

1. Crie um vetor `[1, 2, 3, 4, 5, 6]`.  
2. Use um loop para filtrar apenas os números pares e adicione-os a um novo vetor.  
3. Imprima o vetor filtrado.  

**Exemplo de Saída:**  

```powershell
Números pares: [2, 4, 6]
```

---

Esses exercícios cobrem operações práticas e úteis com **strings**, **tuplas**, **arrays** e **vetores**.
