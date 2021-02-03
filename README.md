# Rust Lang

Repo criado com o objetivo de aprender Rust

## Configurações necessárias para execução local: 

Para executar os fontes localmente é necessário instalar Rust na máquina com o seguinte comando

```$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh```

Clonar o projeto

```$ git clone https://github.com/FaveroFerreira/rustlang.git```

Alterar o mod no arquivo main.rs

```
mod arquivo_a_executar;

fn main() {
    arquivo_a_executar::metodo();
}
```

Executar com o comando

```$ cargo run```