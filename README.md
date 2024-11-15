# Aecium

An experimental alternative implementation of the Rust compiler frontend: that is, what happens when you run [`cargo check`][cargo check] or use [rust-analyzer][].

This is an early-stage research project to investigate how much faster the Rust compiler frontend could potentially be made, allowing for tweaks to the language in service of that goal. The best-case scenario is that we demonstrate the ability to achieve significant speedup with minimal breaking changes to the language, and the result is incorporated into the Rust 2027 [Edition][]. A more likely scenario is that significant speedup beyond [all][nnethercote 2021-11-12] [the][nnethercote 2022-02-25] [work][nnethercote 2022-04-12] [that][nnethercote 2022-07-20] [has][nnethercote 2022-10-27] [already][nnethercote 2023-03-24] [been][nnethercote 2023-08-25] [done][nnethercote 2024-03-06] proves to be impossible.

## Goals

- Support a "reasonable" subset of Rust.
- Be significantly faster than `rustc` and rust-analyzer.
- Propagate enough semantic information to feed to codegen.
- Propagate enough source information to be used in a language server.

## Non-goals

- Support all of Rust.
- Save time via incremental compilation.
- Save time via parallelism.

## Related

See [Chandler Carruth's CppNow 2023 talk][modernizing compiler design for carbon's toolchain] about the [Carbon][] compiler.

## License

Aecium is licensed under the [MIT License](LICENSE).

[carbon]: https://github.com/carbon-language/carbon-lang
[cargo check]: https://doc.rust-lang.org/cargo/commands/cargo-check.html
[edition]: https://doc.rust-lang.org/stable/edition-guide/
[modernizing compiler design for carbon's toolchain]: https://youtu.be/ZI198eFghJk
[nnethercote 2021-11-12]: https://nnethercote.github.io/2021/11/12/the-rust-compiler-has-gotten-faster-again.html
[nnethercote 2022-02-25]: https://nnethercote.github.io/2022/02/25/how-to-speed-up-the-rust-compiler-in-2022.html
[nnethercote 2022-04-12]: https://nnethercote.github.io/2022/04/12/how-to-speed-up-the-rust-compiler-in-april-2022.html
[nnethercote 2022-07-20]: https://nnethercote.github.io/2022/07/20/how-to-speed-up-the-rust-compiler-in-july-2022.html
[nnethercote 2022-10-27]: https://nnethercote.github.io/2022/10/27/how-to-speed-up-the-rust-compiler-in-october-2022.html
[nnethercote 2023-03-24]: https://nnethercote.github.io/2023/03/24/how-to-speed-up-the-rust-compiler-in-march-2023.html
[nnethercote 2023-08-25]: https://nnethercote.github.io/2023/08/25/how-to-speed-up-the-rust-compiler-in-august-2023.html
[nnethercote 2024-03-06]: https://nnethercote.github.io/2024/03/06/how-to-speed-up-the-rust-compiler-in-march-2024.html
[rust-analyzer]: https://rust-analyzer.github.io/
