# **O Sistema de Ownership**

---

## **Exercício 1: Ownership e Borrowing**

Crie um programa que mova a propriedade de uma `String` para outra variável. Em seguida, tente acessar a variável original e observe o erro do compilador. Depois, corrija o programa utilizando uma **referência** para evitar mover a propriedade.

**Objetivo:**  

- Entender a movimentação de ownership e o uso de referências.  

**Tarefa:**  

1. Declare uma `String` e mova sua propriedade para outra variável.  
2. Tente usar a variável original e observe o erro.  
3. Corrija o código usando `&` para criar uma referência.  

---

## **Exercício 2: Borrowing Mutável**

Declare uma variável mutável com uma string e use uma **referência mutável** para alterar seu valor.

**Objetivo:**  

- Praticar borrowing mutável.  

**Tarefa:**  

1. Crie uma variável `mut texto` com o valor `"Rust"`.  
2. Use uma referência mutável para alterar o valor para `"Rust é incrível"`.  
3. Certifique-se de que a referência mutável seja usada corretamente (apenas uma de cada vez).  

---

## **Exercício 3: Fatiamento**

Declare uma string e use **slices** para imprimir partes dela.

**Objetivo:**  

- Trabalhar com slices em strings.  

**Tarefa:**  

1. Declare uma string com o valor `"Rust é fantástico!"`.  
2. Use slices para imprimir:  
   - As 4 primeiras letras (`"Rust"`)  
   - As palavras `"é fantástico!"`  

---

Esses exercícios estão simplificados para que você foque nos conceitos essenciais de ownership, borrowing e slices.
