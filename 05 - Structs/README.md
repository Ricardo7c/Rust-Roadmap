# Structs e Enums

## 01. Definição e Instanciação de Structs

Crie uma struct chamada `Livro` com os seguintes campos: `titulo`, `autor` e `paginas`. Em seguida, instancie um livro e imprima suas informações.

---

## 02. Struct com Métodos

Adicione um método `resumo` à struct `Livro`, que retorna uma string contendo o título e o autor do livro. Instancie um livro e chame esse método para exibir o resumo.

---

## 03. Tuple Structs

Crie uma tuple struct chamada `Coordenada` que contenha dois valores `f64`, representando a latitude e a longitude. Em seguida, crie uma função que recebe uma `Coordenada` e imprime os valores.

---

## 04. Atualização de Campos de Struct

Dada a struct `Carro` com os campos `modelo`, `ano` e `preco`, escreva uma função que recebe um carro e altera o valor do campo `preco`.

---

## 05. Enum Simples

Crie um enum chamado `EstadoLampada` com as variantes `Ligada` e `Desligada`. Escreva uma função que recebe um `EstadoLampada` e imprime o estado atual da lâmpada.

---

## 06. Enum com Dados

Defina um enum `Mensagem` com as variantes `Texto(String)`, `Imagem(String, u32, u32)`, e `Video(String, u32)`. Crie uma função que recebe uma `Mensagem` e imprime informações baseadas na variante recebida.

---

## 07. Structs Aninhadas

Crie duas structs: `Pessoa` (com os campos `nome` e `endereco`) e `Endereco` (com os campos `rua`, `numero` e `cidade`). Instancie uma `Pessoa` com um `Endereco` e imprima as informações de ambas as structs.

---

## 08. Enum e Structs Combinados

Crie um enum `Veiculo` com as variantes `Carro` e `Moto`, onde `Carro` contém uma struct `DetalhesCarro` e `Moto` contém uma struct `DetalhesMoto`. Escreva uma função que recebe um `Veiculo` e usa `match` para imprimir as informações detalhadas de cada variante.

Esses exercícios vão ajudar a reforçar seu aprendizado sobre structs, tuple structs, enums e matching. Boa prática!

---

## 09. Sistema de Notificações

Crie um `enum` chamado `Notification` que pode representar três tipos diferentes de notificações: `Email`, `SMS` e `PushNotification`. Cada variante deve conter diferentes informações:

- `Email`: endereço de email e assunto da mensagem.
- `SMS`: número de telefone e conteúdo do SMS.
- `PushNotification`: nome do aplicativo e conteúdo da notificação.

Implemente uma função chamada `send_notification` que recebe uma `Notification` e imprime a mensagem apropriada dependendo do tipo de notificação.

---

## 10. Estados de Pagamento

Crie um `enum` chamado `PaymentStatus` que represente os estados de um pagamento:

- `Pending`: pagamento pendente.
- `Completed`: pagamento completo com o valor pago.
- `Failed`: pagamento falhou com uma mensagem de erro.

Implemente uma função chamada `print_status` que recebe um `PaymentStatus` e imprime o estado atual do pagamento.

---

## 11. Tipos de Animal

Crie um `enum` chamado `Animal` que pode representar diferentes tipos de animais:

- `Dog`: com uma string representando o nome.
- `Cat`: com uma string representando o nome.
- `Fish`: sem dados adicionais.

Implemente uma função chamada `describe_animal` que recebe um `Animal` e imprime uma descrição do animal com base em seu tipo.

---

## 12. Sistema de Resultados

Implemente um `enum` chamado `Result` para representar os possíveis resultados de uma operação:

- `Success`: contendo o valor do resultado.
- `Error`: contendo uma mensagem de erro.

Implemente uma função chamada `process_result` que recebe um `Result` e realiza uma ação diferente dependendo de ser sucesso ou erro, imprimindo o valor ou a mensagem de erro.

---

## 13. Controle de Tráfego

Crie um `enum` chamado `TrafficLight` que represente os três sinais de um semáforo:

- `Red`
- `Yellow`
- `Green`

Implemente uma função chamada `next_light` que recebe um `TrafficLight` e retorna o próximo sinal (por exemplo, se for `Red`, deve retornar `Green`).
