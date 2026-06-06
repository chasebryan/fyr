# Fyr

Fyr is a new systems programming language aiming for native performance, strong safety, and a small, readable surface.

The long-term goal is direct:

- fast like C
- secure like Rust
- simple like Python

This repository begins with a working bootstrap: a Rust implementation of the `fyr` command, a tiny parser/evaluator, `fyr run`, `fyr check`, and a terminal REPL.

## Try It

```sh
cargo run -p fyr -- run examples/hello.fyr
cargo run -p fyr -- run examples/fib.fyr
cargo run -p fyr -- check examples/hello.fyr
cargo run -p fyr
```

Inside the REPL:

```fyr
let answer = 40 + 2
answer
print("Fyr is alive")
```

Functions already use Python-style indented bodies:

```fyr
fn fib(n):
    if n < 2:
        n
    else:
        fib(n - 1) + fib(n - 2)

print(fib(10))
```

## Current Language Slice

The bootstrap supports:

- integer, boolean, and string literals
- `let` bindings
- arithmetic and comparison operators
- boolean `&&`, `||`, and `!`
- string concatenation with `+`
- Python-style indented function bodies
- recursive function calls
- `if` / `else` expressions
- built-in `print(value)` and `type(value)`
- one-statement-per-line scripts

## Direction

Fyr will grow in stages:

1. bootstrap interpreter and REPL
2. static type checker
3. ownership and safety checker
4. native backend
5. standard library
6. package manager and build system
7. the Fyr book

The repo should always keep a runnable language at the center. Design documents and book chapters should describe behavior that either exists or is actively being implemented.
