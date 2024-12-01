Aqui estão exercícios práticos para ajudá-lo a aprender e fixar os conceitos dessa parte do roadmap, com foco em **funções**, **lifetimes**, **generics**, e outros tópicos:

---

### **Exercício 1: Trabalhando com Funções**
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

### **Exercício 2: Usando Lifetimes**
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

### **Exercício 3: Trabalhando com Generics**
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

### **Exercício 4: Uso de Bibliotecas**
Use a biblioteca `rand` para gerar um número aleatório entre 1 e 100. Peça ao usuário para adivinhar o número.

**Objetivo:**  
- Aprender a adicionar e usar bibliotecas no projeto.  

**Requisitos:**  
1. Configure o `Cargo.toml` para incluir a biblioteca `rand`.  
2. Gere um número aleatório entre 1 e 100.  
3. Compare a entrada do usuário com o número gerado e exiba mensagens como "Muito alto" ou "Muito baixo".  

---

### **Exercício 5: Trabalhando com `Option`**
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

### **Exercício 6: Trabalhando com `Result`**
Implemente um programa que leia um arquivo texto e conte o número de linhas. Use `Result` para tratar erros, como o arquivo não existir.

**Objetivo:**  
- Aprender a trabalhar com `Result` para manipular possíveis erros.  

**Requisitos:**  
1. Use a função `std::fs::read_to_string` para ler o conteúdo do arquivo.  
2. Retorne um `Result` indicando sucesso ou erro.  
3. Exiba o número de linhas ou uma mensagem de erro.  

**Exemplo de Saída:**  
Caso o arquivo exista:  
`"Número de linhas: 42"`

Caso o arquivo não exista:  
`"Erro: Não foi possível abrir o arquivo"`

---

### **Exercício 7: Misturando Conceitos**
Crie uma função genérica que filtra os elementos de um vetor com base em um predicado passado como argumento.

**Objetivo:**  
- Integrar conceitos como funções, generics e closures.  

**Requisitos:**  
1. A função deve receber um vetor de qualquer tipo e uma closure como argumento.  
2. Retorne um novo vetor contendo apenas os elementos que satisfazem o predicado.  

**Exemplo de Entrada/Saída:**  
Entrada: `[1, 2, 3, 4, 5]`, predicado: `|x| x % 2 == 0`  
Saída: `[2, 4]`

---
