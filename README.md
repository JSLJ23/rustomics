# Rustomics

## Currently available:
- BED file chromosome renaming tool.
    - Mapping chromosome names from GTF/GFF3 annotations to more readable naming (i.e. Chromosome 1, 2, 3, X, Y, etc...).

## Build instructions:
- Once Rust and Cargo have been installed and this repo has been cloned, run:
    ```rust
    cargo build --release
    ```
- The binary will be found in `target/release.rustomics`.
- Run `rustomics --help` for guide on usage.
