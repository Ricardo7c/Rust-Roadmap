# Exercícios para praticar o uso de traits em Rust

## 1. Implementando uma *trait* simples

Crie uma *trait* chamada `Descrevivel` com um método `descricao` que retorna uma string. Em seguida, implemente essa *trait* para uma struct chamada `Carro`, onde a descrição deve incluir a marca e o modelo do carro.

---

## 2. *Trait* com comportamento padrão

Crie uma *trait* chamada `Som` com um método `emitir_som` que, por padrão, imprime "Som genérico". Em seguida, crie uma struct chamada `Cachorro` e implemente essa *trait* nela, mas sobrescreva o comportamento do método para imprimir "Au au".

---

## 3. *Trait* para múltiplos tipos

Crie uma *trait* chamada `Movimento` com um método `mover`. Depois, implemente essa *trait* para as structs `Carro` e `Bicicleta`. Faça com que cada implementação do método imprima uma frase descrevendo o movimento de cada um.

---

## 4. Usando *traits* com funções

Crie uma *trait* chamada `Area` com um método `calcular_area`. Em seguida, crie duas structs: `Retangulo` e `Circulo`. Implemente a *trait* `Area` para ambas as structs e depois crie uma função que recebe um objeto que implementa a *trait* `Area` e imprime a área calculada.

---

## 5. Herança de *traits*

Crie uma *trait* chamada `Animal` com um método `comer`. Em seguida, crie outra *trait* chamada `Mamifero` que herda de `Animal` e adiciona um método `andar`. Implemente a *trait* `Mamifero` para a struct `Humano` e defina o comportamento de ambos os métodos.

---

## 6. Filtrando números com *trait bounds*

Crie uma *trait* chamada `Positivo` com um método `eh_positivo` que retorna um booleano. Implemente essa *trait* para o tipo `i32`. Em seguida, crie uma função `filtrar_positivos` que aceita uma lista de números e retorna apenas os positivos, usando *trait bounds* para garantir que a função só receba tipos que implementam a *trait* `Positivo`.

---

## 7. Somando valores com *trait bounds*

Crie uma *trait* chamada `Somavel` com um método `somar` que recebe outro valor do mesmo tipo e retorna a soma. Implemente essa *trait* para `i32` e `f64`. Depois, crie uma função genérica chamada `somar_valores` que aceita dois valores e retorna a soma deles, usando *trait bounds* para garantir que os parâmetros implementem `Somavel`.

---

## 8. Comparando objetos com *trait bounds*

Crie uma *trait* chamada `Comparavel` com um método `maior_que` que compara dois objetos do mesmo tipo. Implemente essa *trait* para a struct `Pessoa`, onde uma pessoa é maior que outra se sua idade for maior. Crie uma função genérica chamada `comparar_pessoas` que recebe duas pessoas e usa *trait bounds* para chamar o método `maior_que`.

---

## 9. Ordenando elementos com *trait bounds*

Crie uma *trait* chamada `Ordenavel` com um método `ordenar`. Implemente essa *trait* para `Vec<i32>`, onde o método `ordenar` organiza os elementos em ordem crescente. Depois, crie uma função genérica `ordenar_elementos` que recebe um vetor de elementos ordenáveis e usa *trait bounds* para garantir que o método `ordenar` seja chamado.

---

## 10. Conversão entre tipos com *trait bounds*

Crie uma *trait* chamada `Convertivel` com um método `converter` que converte um tipo em outro. Implemente essa *trait* para converter entre as structs `Celsius` e `Fahrenheit`. Depois, crie uma função genérica `converter_temperatura` que aceita um valor que implementa `Convertivel` e retorna a temperatura convertida.

---

## 11. Descrição e Identificação de Produtos

Crie uma *trait* `Descrevivel` com um método `descricao` que retorna uma `String`. Em seguida, crie outra *trait* `Identificavel` que herda de `Descrevivel` e define um método `identificar`, que utiliza o método `descricao` para exibir a identificação do objeto. Implemente essas *traits* para uma struct `Produto` com os campos `nome` e `preco`. Teste o código exibindo a descrição e a identificação do produto.

---

## 12. Exibição e Comparação de Carros

Crie uma *trait* `Exibivel` com um método `exibir`. Em seguida, crie uma *trait* `Comparavel` que herda de `Exibivel` e define um método `comparar` que recebe uma referência para outro objeto e retorna `true` se os objetos forem iguais e `false` caso contrário. Implemente essas *traits* para uma struct `Carro` com os campos `marca` e `velocidade`. Crie uma função que utilize `Comparavel` para exibir e comparar dois carros.

---

## 13. Cálculo e Exibição de Área de Formas

Crie uma *trait* `CalculoArea` com um método `area` que retorna um `f64` e uma *trait* `FormaExibivel` que herda de `CalculoArea` e define um método `exibir_area` para exibir a área da forma. Implemente essas *traits* para as structs `Quadrado` e `Circulo`. Para `Circulo`, considere o valor de PI como 3.14. Crie uma função que recebe uma forma e exibe sua área usando `FormaExibivel`.

---

## 14. Comparação de Nomes de Pessoas

Crie uma *trait* `Nomeavel` com um método `nome` que retorna uma `String` e uma *trait* `ComparavelPorNome` que herda de `Nomeavel`. Na *trait* `ComparavelPorNome`, defina um método `comparar_nomes` que recebe outro objeto e imprime se os nomes são iguais ou diferentes. Implemente essas *traits* para uma struct `Pessoa` com o campo `nome`. Crie uma função que recebe duas `Pessoa` e compara seus nomes.

---

## 15. Movimento Descritivo de Veículos

Crie uma *trait* `Movimento` com um método `mover` e uma *trait* `MovimentoDescritivo` que herda de `Movimento`. Em `MovimentoDescritivo`, defina um método `mover_com_descricao` que chama `mover` e exibe uma mensagem descritiva. Implemente essas *traits* para as structs `Carro` e `Aviao`. Para cada struct, o método `mover` deve imprimir uma mensagem sobre o movimento específico do veículo, e `mover_com_descricao` deve incluir uma descrição adicional. Crie uma função que recebe um veiculo e chama o metodo `mover_com_descricao`.

---

## 16. Coleção de Formas Geométricas

Crie uma *trait* chamada `Forma`, com o método `area`, que retorna a área de uma forma geométrica como `f64`. Depois:

1. Implemente a *trait* `Forma` para duas structs: `Circulo` (com raio `r`) e `Retangulo` (com largura e altura).
2. No `main`, crie um vetor de *trait objects* `Vec<Box<dyn Forma>>` para armazenar diferentes instâncias de `Circulo` e `Retangulo`.
3. Crie uma função `mostrar_areas`, que recebe esse vetor e imprime a área de cada forma.

Esse exercício ajuda a entender como *trait objects* permitem lidar com diferentes tipos que implementam uma mesma *trait*.

---

## 17. Sistema de Pagamento

Crie uma *trait* chamada `Pagamento`, com o método `processar`, que não retorna nada. Depois:

1. Implemente a *trait* para duas structs: `CartaoCredito` e `Boleto`.
   - Em `CartaoCredito`, `processar` deve imprimir algo como: `"Processando pagamento por cartão de crédito."`
   - Em `Boleto`, deve imprimir algo como: `"Processando pagamento por boleto bancário."`
2. No `main`, crie uma lista `Vec<Box<dyn Pagamento>>` contendo uma instância de cada método de pagamento.
3. Passe a lista para uma função `executar_pagamentos`, que chama `processar` para cada item da lista.

Com esse exercício, você verá como *trait objects* possibilitam armazenar e manipular diferentes implementações da mesma interface.

---

## 18. Monitoramento de Sensores

Defina uma *trait* chamada `Sensor`, com um método `ler_dado` que retorna uma `String` com o dado do sensor. Em seguida:

1. Crie duas structs, `SensorTemperatura` e `SensorUmidade`, cada uma implementando a *trait* `Sensor`.
   - Em `SensorTemperatura`, `ler_dado` retorna algo como `"Temperatura: 24°C"`.
   - Em `SensorUmidade`, `ler_dado` retorna algo como `"Umidade: 55%"`.
2. No `main`, crie um vetor `Vec<Box<dyn Sensor>>` que armazene instâncias de cada tipo de sensor.
3. Escreva uma função `coletar_dados` que recebe esse vetor e imprime o valor lido de cada sensor.

Esse exercício mostra como *trait objects* permitem abstrair diferentes tipos de sensores sob uma única interface.

---

## 19. Registro de Animais no Zoológico

Crie uma *trait* `Animal` com dois métodos: `nome` (retorna uma `String` com o nome do animal) e `emitir_som` (imprime um som característico do animal). Depois:

1. Implemente `Animal` para duas structs: `Leao` e `Macaco`.
   - `Leao` deve imprimir `"Rugido!"` em `emitir_som`.
   - `Macaco` deve imprimir `"Grito!"`.
2. No `main`, crie uma lista `Vec<Box<dyn Animal>>` para armazenar instâncias de `Leao` e `Macaco`.
3. Crie uma função `mostrar_animais` que recebe a lista e, para cada animal, imprime seu nome e chama `emitir_som`.

Este exercício explora como *trait objects* ajudam a criar coleções heterogêneas de objetos.

---

## 20. Sistema de Notificações

Implemente uma *trait* `Notificacao`, que tem um método `enviar`, o qual imprime uma mensagem de notificação. Em seguida:

1. Crie duas structs: `Email` e `SMS`.
   - Em `Email`, `enviar` deve imprimir `"Enviando e-mail de notificação."`.
   - Em `SMS`, deve imprimir `"Enviando SMS de notificação."`
2. No `main`, crie um vetor `Vec<Box<dyn Notificacao>>` contendo uma instância de `Email` e uma de `SMS`.
3. Crie uma função `enviar_notificacoes` que recebe essa lista e chama `enviar` para cada notificação.
