# RS Sequence Alignment
A simple rust CLI program which executes the Dynamic Programming Needleman–Wunsch algorithm for aligning 2 DNA sequences supplied through argv.

* Example usage
```sh
cargo r CACACG CACTGCAT
```
* Example output
```text
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/rs-sequence-alignment CACACG CACTGCAT`
CAC-AC-G
CACTGCAT
```
