# Advent-of-Code-2024

Another year, another attempt at this fun thing!

Language: [Rust](https://www.rust-lang.org/)

## Running Code For Each Day

- Run the code

  - ```sh
    cd day_x
    cargo run -q
    ```

## Tests For Each Day

Tests are based on the sample input provided for each day.

```sh
cd day_x
cargo test -q
```

## Debugging Code Issues

To actively monitor code issues use [`bacon`](https://github.com/Canop/bacon).
Install [`bacon`](https://github.com/Canop/bacon): `cargo install --locked bacon`

```sh
bacon --all-features --job clippy
```
