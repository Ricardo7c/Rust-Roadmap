# Lidando com Erros  

---

## 01. Propagação de Erros com `Result`  

**Objetivo**: Implementar funções que propaguem erros usando o tipo `Result`.  
**Instruções**:  

- Crie uma função `read_file` que tenta ler o conteúdo de um arquivo e retorna `Result<String, std::io::Error>`.  
- Use a função `std::fs::read_to_string` para ler o arquivo e propague possíveis erros usando `?`.  
- Chame a função em `main` e trate o erro, exibindo uma mensagem amigável ao usuário.  

---

## 02. Funções Encadeadas com Erros  

**Objetivo**: Propagar erros em chamadas de funções encadeadas.  
**Instruções**:  

- Crie uma função `parse_number` que recebe uma string e converte para `i32`.  
- Crie outra função `calculate_square` que calcula o quadrado do número convertido.  
- Propague erros em ambas as funções usando `Result`.
  
---

## 03. Erro Customizado Simples  

**Objetivo**: Criar um tipo de erro customizado.  
**Instruções**:  

- Crie um `enum` chamado `MathError` com dois casos: `DivisionByZero` e `NegativeNumber`.  
- Implemente a função `divide` que retorna `Result<f64, MathError>` e retorna `Err` em divisões por zero.  

---

## 04. Erro Customizado com Mensagens Personalizadas  

**Objetivo**: Aprender a usar `std::fmt` em erros customizados.  
**Instruções**:  

- Adicione o trait `std::fmt::Display` ao `enum MathError` do exercício anterior.  
- Personalize mensagens como *"Erro: Não é possível dividir por zero."*.  
- Teste a implementação no `main`.  

---

## 05. Testando Funções com Erros

**Objetivo**: Escrever testes unitários para funções que retornam `Result`.  
**Instruções**:  

- Escreva testes para as funções dos exercícios anteriores usando o módulo `#[cfg(test)]`.  
- Verifique se os erros são corretamente retornados para entradas inválidas.  

---

## 06. Lidando com Vários Tipos de Erros

**Objetivo**: Trabalhar com múltiplos tipos de erros.  
**Instruções**:  

- Implemente uma função `process_file` que lê um arquivo, converte cada linha em um número e calcula sua soma.  
- Lide com erros de I/O e de parsing usando `Result`.  
**Saída esperada**: Um erro detalhado, como *"Erro ao ler o arquivo: ..."* ou *"Erro ao converter uma linha para número."*.  

---

## 07. Usando `thiserror` para Erros Customizados  

**Objetivo**: Aprender a simplificar erros customizados com a crate `thiserror`.  
**Instruções**:  

- Adicione `thiserror` ao seu projeto (`cargo add thiserror`).  
- Reescreva o `MathError` usando a macro `#[derive(thiserror::Error)]`.  
- Personalize as mensagens de erro usando `#[error(...)]`.  

---

## 08. Testando Erros com o Macro `assert_eq!` e `assert!(matches!())`  

**Objetivo**: Escrever testes específicos para erros customizados.  
**Instruções**:  

- Escreva testes para garantir que funções retornam erros esperados.  
- Use `assert!(matches!(...))` para verificar casos de erro.  
**Exemplo de uso**: `assert!(matches!(divide(1.0, 0.0), Err(MathError::DivisionByZero)));`  

---

## 09. Tratando Erros com `anyhow`  

**Objetivo**: Trabalhar com a crate `anyhow` para simplificar propagação de erros.  
**Instruções**:  

- Adicione a crate `anyhow` ao projeto (`cargo add anyhow`).  
- Implemente a função `read_and_parse` que tenta ler e processar um arquivo.  
- Use `anyhow::Result` para simplificar os retornos de erro.  

---

## 10. Benchmark de Erros com `cargo test`

**Objetivo**: Avaliar o desempenho de funções com erros em cenários de teste.  
**Instruções**:  

- Escreva testes para medir o desempenho de funções que propagam erros.  
- Use entradas válidas e inválidas para verificar comportamento e performance.  
**Saída esperada**: Mensagens indicando o tempo de execução para cada cenário.  

---
