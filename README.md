# Hasanlang
![Build Status](https://github.com/greenbush5/hasanlang/actions/workflows/build.yml/badge.svg)
![Language](https://img.shields.io/badge/Language-Rust-orange)
![Contributions](https://img.shields.io/badge/Contributions-Welcome-brightgreen)

Hasanlang is a work-in-progress programming language, made for fun, that aims to be a self-hosted. The goal of this project is to extend my knowledge about compilers, programming languages, and how they are made. Currently, this is my primary project.

## Project Status
This project is in the early stages. The language specifications, parser, and basic compiler are under active development. Plans to become self-hosted are underway. I'm not certain whether I will have the will to finish this, but as it seeems, the likelihood of giving up is relatively low.

## Roadmap
- [x] Pest Parser
- [ ] Corrective Parser
- [ ] Compiler
- [ ] Standard Library
- [ ] Language Server
- [ ] Self-hosting

## Getting Started
To set up Hasanlang, you will need to have [Rust](https://www.rust-lang.org/tools/install) and [LLVM 15.0](https://releases.llvm.org/download.html) on your system. Then you can clone this repository and build the project using `cargo`, the Rust package manager.

```bash
git clone https://github.com/greenbush5/hasanlang.git
cd hasanlang
cargo build
```

Additionally, you can install [Hasanlang syntax highlighting extension](https://github.com/greenbush5/hasanlang-syntax-extension) for VS Code. This is my first time making a syntax highlighting extension, so feel free to contribute to this extension as well!

## Contributing
I would love to have your help in making Hasanlang better, contributions are very welcome.

Here are ways you can contribute:

- by reporting bugs
- by suggesting new features
- by writing or editing documentation
- by writing specifications
- by writing code and submitting pull requests

## Bugs
As I said before, this is my first time making a project as complex as this, so my code is particularly prone to errors. Due to the complexity of this project, your help in finding and/or fixing bugs would significantly contribute to its advancement. With your help, this entire project may become possible to complete!

## License
Hasanlang is [MIT licensed](https://en.wikipedia.org/wiki/MIT_License).

## My current TODO list
- [ ] Store a span in every AST node for error reporting
- [ ] Clarify `parse_..._expression` function calls *(call more specific functions instead)*
- [ ] Add more unary operators
- [ ] Implement anonymous functions
- [ ] Add support for floats
- [ ] Allow for recursive parsing of types
- [ ] Implement `for` statements
- [x] Implement `if` and `while` statements
- [x] Implement booleans
- [x] Implement enums
- [x] Make tests check the generated AST
- [x] Add raw types *(direct LLVM types, should not have any methods on them, intended for compiler usage only)* that can be denoted with an exclamation mark: `AType!`, `ArrayType[]!`, `GenericType<...>!`, `GenericArrType<...>[]!`
- [x] Add type cast operator to *recursive* expressions
- [x] Fix *recursive* expressions
- [x] Remove `inputX.hsl` code samples from root directory
- [x] Move this into `README.md`