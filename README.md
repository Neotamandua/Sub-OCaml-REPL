# Sub-OCaml-REPL
[![made-with-rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg?style=flat-square)](https://www.rust-lang.org/)
[![License: MPL 2.0](https://img.shields.io/badge/License-MPL_2.0-brightgreen.svg?style=flat-square)](https://github.com/Neotamandua/Sub-OCaml-REPL/blob/master/LICENSE)
![License: MPL 2.0](https://img.shields.io/github/languages/code-size/Neotamandua/Sub-OCaml-REPL?style=flat-square)
![Github CI](https://img.shields.io/github/actions/workflow/status/Neotamandua/Sub-OCaml-REPL/build.yml?style=flat-square)
> Simple read–eval–print loop (REPL) for [Sub-OCaml](https://github.com/Neotamandua/Sub-Ocaml) with global persistent environment
### Example Screenshot:
[![Screenshot](assets/screenshot1.png)](#)

### Dependencies:
- [Sub-OCaml](https://github.com/Neotamandua/Sub-Ocaml)
```toml
[dependencies]
rustyline = "11.0.0"
anyhow = "1.0.69"

[dependencies.Sub-OCaml]
path = "Sub-OCaml" # change to your path
```