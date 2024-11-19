# Porque Rust?

Quando comecei a programar, iniciei com Python, uma linguagem intuitiva e acessível. No entanto, ouvi diversas vezes que, para realmente entender os fundamentos da programação, era necessário explorar uma linguagem de baixo nível. Decidi pausar meus estudos em Python e mergulhar em C, uma linguagem clássica que moldou grande parte da computação moderna.  

Durante meus estudos, percebi que as linguagens de baixo nível, embora exigentes, não eram tão intimidantes quanto parecem à primeira vista. Contudo, também notei que C, por ser mais antiga, permite práticas que podem comprometer a segurança e a estabilidade de aplicações, especialmente nas mãos de programadores iniciantes.  

Foi nesse contexto que, ao aprofundar minhas pesquisas, descobri o **Rust**. A linguagem chamou minha atenção por sua combinação de alto desempenho com segurança de memória garantida pelo compilador, eliminando muitos dos problemas comuns em linguagens como C. Após experimentá-la, fiquei impressionado com sua abordagem moderna e suas inovações.  

Minha admiração por Rust cresceu ainda mais ao perceber sua crescente adoção em indústrias críticas e sua recomendação até mesmo por instituições como a Casa Branca como um substituto seguro para o C em sistemas de alto impacto. Essa descoberta consolidou minha decisão de adotar Rust como minha principal linguagem de estudo e desenvolvimento.  

E se você chegou até aqui, acredito que pensa como eu. Sendo assim, este roadmap é para você. Vamos juntos nessa jornada!  

--- 

# Instalando Rust

Um guia detalhado para instalar o Rust em sistemas **Windows**, **Linux** e **Mac**:

## **1. Instalar Rust no Windows**
Rust é instalado via **rustup**, o instalador oficial.

### **Passo a passo:**
1. **Baixe o instalador do Rust:**
   - Acesse o site oficial: [https://rustup.rs](https://rustup.rs).
   - Clique no botão para baixar o instalador para Windows.

2. **Execute o instalador:**
   - Após o download, execute o arquivo `.exe`.
   - Siga as instruções na tela.

3. **Configure o ambiente:**
   - O instalador configurará automaticamente o Rust e o `cargo` no PATH do sistema. Você poderá usar o Rust diretamente no Prompt de Comando ou no PowerShell.

4. **Verifique a instalação:**
   - Abra o terminal (Prompt de Comando ou PowerShell).
   - Execute:
     ```bash
     rustc --version
     ```
   - Deve exibir a versão instalada do Rust.

---

## **2. Instalar Rust no Linux**
Rust também é instalado via **rustup**, disponível para a maioria das distribuições.

### **Passo a passo:**
1. **Abra o terminal.**
2. **Baixe e execute o script de instalação:**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
3. **Siga as instruções:**
   - O script pedirá confirmação e configurará o Rust.

4. **Configure o PATH (se necessário):**
   - Após a instalação, reinicie o terminal ou adicione o Rust ao PATH manualmente:
     ```bash
     source $HOME/.cargo/env
     ```

5. **Verifique a instalação:**
   - Execute:
     ```bash
     rustc --version
     ```
   - Deve exibir a versão instalada.

---

## **3. Instalar Rust no macOS**
O processo no macOS é semelhante ao Linux.

### **Passo a passo:**
1. **Abra o terminal.**
2. **Baixe e execute o script de instalação:**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
3. **Siga as instruções:**
   - O script instalará o Rust e configurará o ambiente.

4. **Configure o PATH (se necessário):**
   - Se o Rust não for reconhecido após a instalação, adicione ao PATH:
     ```bash
     source $HOME/.cargo/env
     ```

5. **Verifique a instalação:**
   - Execute:
     ```bash
     rustc --version
     ```
   - Deve exibir a versão instalada.

---

### **Dicas adicionais:**
- **Atualizar o Rust:**  
  Após instalar, você pode atualizar o Rust com:
  ```bash
  rustup update
  ```

- **Desinstalar o Rust:**  
  Para remover o Rust completamente, use:
  ```bash
  rustup self uninstall
  ```
