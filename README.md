# Atalanta

Atalanta, named after the Greek godess of running, performs typical software
development tasks like installing dependencies and running scripts for a wide
range of software stacks. This is useful if you work on a large variety of
codebases, package managers, and developer tooling.

### installation

You need [Rust](https://www.rust-lang.org). The recommended (and super easy) way
to install it is via [RustUp](https://rustup.rs). To install Atalanta:

```
cargo install https://github.com/kevgo/atalanta
```

This installs a binary called `a` into the `~/.cargo/bin` folder. Please add it
to your `$PATH`.

### usage

Install dependencies for your codebase (`-s` stands for "setup"):

```
a -s
```

Running Atalanta without any parameters shows all available tasks:

```
a
```

Run the `test` script defined in `package.json`:

```
a test
```

You can also provide fewer characters for task names as long as they uniquely
match exactly one task:

```
a t
```
