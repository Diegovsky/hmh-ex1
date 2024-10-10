# Exercício 1 
O objetivo é desenvolver um programa que seja capaz de ler o formato txt espeficiado e montar dois grafos, um implementado por lista de adjacência e outro por matriz de adjacência.

O código fonte completo e documentado se encontra em `src/main.rs`. O código está bem documentado, e inclui comentários a respeito de como algumas coisas em Rust funcionam para melhor compreensão.

## Como rodar
Para que não seja necessário que você tenha instalado um ambiente de desenvolvimento rust, incluí um binário pré compilado para linux-x86_64 na pasta `bin/`.

Dito isso, basta executar o programa passando como primeiro argumento o nome do arquivo:

```
ex1 <arquivo>
```

Incluí o arquivo de exemplo do exercício, salvo como `input.txt`.

## Como compilar (opcional)
Você vai precisar dos programas `rustc` e `cargo`. Normalmente podem ser instalados nas distribuições linux pelo pacote `rust`.

Basta executar `cargo build`, e após isso, o programa estará localizado em `target/debug/ex1`.
