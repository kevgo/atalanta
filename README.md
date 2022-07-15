# Atalanta

Atalanta &ndash; named after the Greek godess of running &ndash; performs
typical software development tasks like installing dependencies and running
scripts for a wide range of software stacks. This is useful if you work on a
large variety of codebases, package managers, and developer tooling.

### installation

Install [Rust](https://www.rust-lang.org) via [RustUp](https://rustup.rs). Then
install Atalanta:

```
cargo install https://github.com/kevgo/atalanta
```

This installs a binary called `a` into the `~/.cargo/bin` folder. Please add it
to your `$PATH`.

To set up the auto-completions for [Fish shell](https://fishshell.com), add this
to `~/.config/fish/config.fish`:

```
if test -f ~/.cargo/bin/a
  ~/.cargo/bin/a --print-fish-completions | source
end
```

### usage

Atalanta determines the stack of your codebase and does the right thing. For
example, to install dependencies for your codebase:

```
a -s
```

`-s` stands for "setup" here. Running Atalanta without any parameters shows all
available tasks:

```
a
```

If your codebase contains a file `package.json` that defines a `test` script:

```
a test
```

If your codebase contains a `Makefile` that defines a `build` task:

```
a build
```

You can also provide fewer characters for task names as long as they uniquely
match a task. To run the test script again:

```
a t
```
