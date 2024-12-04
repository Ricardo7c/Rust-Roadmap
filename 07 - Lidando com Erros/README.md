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

## 06. Usando `thiserror` para Erros Customizados  

**Objetivo**: Aprender a simplificar erros customizados com a crate `thiserror`.  
**Instruções**:  

- Adicione `thiserror` ao seu projeto (`cargo add thiserror`).  
- Reescreva o `MathError` usando a macro `#[derive(thiserror::Error)]`.  
- Personalize as mensagens de erro usando `#[error(...)]`.  

---
