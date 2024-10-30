# Rust Roadmap + Exercícios
Esse roadmap permite uma boa cobertura dos conceitos essenciais de Rust.

---

## 🛠️ 1. **Fundamentos do Rust**
   - **Objetivo**: Entender a sintaxe e os conceitos básicos.
   - **Conteúdos**:
     - Instalação do Rust e do Cargo (gerenciador de pacotes)
     - Variáveis e Imutabilidade
     - Tipos de Dados e Conversões
     - Estruturas de Controle: `if`, `loop`, `while`, `for`
     - Funções e escopos
     - Tipagem estática e inferência de tipos

   - **Exercícios**:
     - Crie um programa "Hello World!" e experimente com variáveis imutáveis.
     - Escreva um programa que converta temperaturas entre Celsius e Fahrenheit.
     - Faça um contador com um loop que exibe números de 1 a 100.

---

## 📦 2. **Entendendo a Gestão de Memória e Borrowing**
   - **Objetivo**: Aprender os conceitos centrais de ownership, borrowing e lifetimes.
   - **Conteúdos**:
     - Conceito de Ownership e Borrowing
     - Referências e `&mut` 
     - Ciclo de vida (`lifetimes`) e escopo
     - Tipos compostos (tuplas e arrays)

   - **Exercícios**:
     - Implemente uma função que retorne o maior número em um array usando borrowing.
     - Crie um programa que simule uma pilha usando um `Vec`, adicionando e removendo elementos com funções.
     - Construa um programa que demonstre o uso de `&mut` e explique o que acontece se tentar alterar o valor diretamente.

---

## 🧩 3. **Tipos de Dados Complexos**
   - **Objetivo**: Manipular tipos como `structs`, `enums`, e `Option`/`Result`.
   - **Conteúdos**:
     - `structs`: definição e métodos associados
     - `enums` e `match`
     - Trabalhando com `Option` e `Result` para tratamento de erros
     - `Vec` e `String`

   - **Exercícios**:
     - Crie uma estrutura `Pessoa` com nome, idade e métodos para modificar esses valores.
     - Desenvolva um programa que receba uma lista de números e retorne o menor valor ou `None` se estiver vazia.
     - Implemente um sistema de login básico com enums para definir o estado do login (ativo, inativo, bloqueado).

---

## 🧵 4. **Funções, Closures e Iteradores**
   - **Objetivo**: Aprender a criar e usar funções e closures.
   - **Conteúdos**:
     - Closures e como elas interagem com o ownership
     - Uso de `map`, `filter`, e `collect` em coleções
     - Iteradores e a API de iteradores

   - **Exercícios**:
     - Escreva uma função que filtre todos os números pares de uma lista.
     - Crie um programa que simule uma calculadora usando closures.
     - Implemente uma função que percorra um `Vec` de strings e retorne apenas as que começam com uma letra específica.

---

## 🧱 5. **Programação Orientada a Objetos com Rust**
   - **Objetivo**: Estruturar o código usando os princípios de OOP.
   - **Conteúdos**:
     - Traits e implementação de traits
     - Traits dinâmicos e polimorfismo
     - Herança com Traits (composição de traits)

   - **Exercícios**:
     - Implemente uma trait `Descritivo` que forneça uma descrição para diferentes tipos de objetos.
     - Crie uma trait `Desconto` e implemente-a em `Produto`, que possui preço e calcula o preço com desconto.
     - Use traits dinâmicos para criar uma coleção de objetos que possam ser descritos de maneira polimórfica.

---

## ⚙️ 6. **Trabalhando com Concurrency**
   - **Objetivo**: Utilizar a biblioteca `std::thread` para manipular threads e explorar `async` em Rust.
   - **Conteúdos**:
     - Threads e `std::thread`
     - `async` e `await`
     - Mensagens e canal (`mpsc`)

   - **Exercícios**:
     - Crie um programa que execute duas tarefas simultaneamente em threads separadas.
     - Escreva uma função assíncrona que simule uma operação de entrada e saída.
     - Construa um sistema de mensagens entre threads usando canais.

---

## 🕸️ 7. **Rust Avançado e Manipulação de Memória**
   - **Objetivo**: Explorar conceitos mais avançados e otimizados para desempenho.
   - **Conteúdos**:
     - Macros e criação de macros
     - Unsafe Rust e manipulação direta de ponteiros
     - Manipulação de memória com `Box`, `Rc`, e `Arc`
     - Gerenciamento de coleções com `HashMap` e `BTreeMap`

   - **Exercícios**:
     - Crie um macro básico que receba um número indefinido de argumentos e imprima cada um.
     - Implemente uma estrutura de dados que usa `Box` para gerenciar memória.
     - Desenvolva uma aplicação multi-threaded que compartilha dados usando `Arc`.

---

## 📜 8. **Projetos Práticos**
   - **Objetivo**: Consolidar o conhecimento em um projeto real.
   - **Sugestões de Projetos**:
     - Desenvolva um CLI simples para gerenciamento de tarefas (`todo list`)
     - Crie uma API REST em Rust usando o framework Rocket ou Actix
     - Desenvolva um sistema de cache simples em memória
     - Construa um jogo simples, como "Pedra, Papel e Tesoura"

--- 