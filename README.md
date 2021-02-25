# logik

## Instalação do rust

Se você já tem o rust instalado pode pular essa parte. Caso contrário abra a aba rustup a seguir.

<details>
<summary>rustup</summary>

### rustup

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Mais detalhes em [rustup.rs](https://rustup.rs)

### Setup do PATH

Após a instalação do rustup, todos os binários estarão na pasta `~/.cargo/bin`, `~/.local/share/cargo` ou em `$XDG_DATA_HOME/cargo/bin`. Essa pasta é mencionada durante a instalação do rustup.

Após determinar qual a pasta correta é necessário adicioná-la a variável de ambiente `PATH`.

Para isso é necessário a seguinte linha:

```shell
source PATH_DA_PASTA/env
```

onde `PATH_DA_PASTA` é uma das 3 pastas mencionadas anteriormente.

Para rodar *apenas* na atual seção do terminal basta apenas rodar o comando no terminal. Para um setup consistente é necessário adicionar a linha ao final do arquivo `~/.bashrc` e rodar `source ~/.bashrc`.

### Verificação da instalação

Apenas verifique que é possível rodar `rustc --version`. Seu output deve ser similar a isso:

```
$ rustc --version
rustc 1.50.0 (cb75ad5db 2021-02-10)
```

</details>

## Como compilar e rodar

### Para rodar e compilar

```shell
cargo run -- FLAGS
```

Para `FLAGS` pode ser passado `--help` para ajuda

O programa pode ser utilizado da seguinte maneira:

```shell
cargo run --quiet -- "1 +  1    - 5"
```

### Para compilar e rodar separadamente

Compilando

```shell
cargo build
```

Rodando

```shell
./target/debug/logik " 1 + 2 -3 + 4"
```
