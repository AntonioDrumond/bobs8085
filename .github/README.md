# BOBS
Intel 8085 processor simulator, built in rust.

This repository's goal is to build an accurate Intel 8085 simulator, with both text based and graphical based interfaces.

---

## Utilization
+ In the text based interface, type `help` to see available commands.

## Compilation
+ To compile this project, run this command from the repository's root:
```sh
    cargo build --bin bobs8085 --release # For the text based version
    cargo build --bin bobs8085-gui --release # For the graphical interface
```
+ The binaries can be found in `target/release/bobs8085` and `target/release/bobs8085-gui`
+ Alternatively, the latest stable builds are available as releases in this repository.
