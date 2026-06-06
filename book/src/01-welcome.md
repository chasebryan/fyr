# Welcome to Fyr

Fyr is a programming language for people who want native speed, strong safety, and code that stays readable under pressure.

The first thing to learn is the terminal command:

```sh
fyr
```

With no arguments, `fyr` starts the REPL. That REPL is the front door to the language.

```fyr
let answer = 40 + 2
answer
```

Fyr functions use indented bodies:

```fyr
fn fib(n):
    if n < 2:
        n
    else:
        fib(n - 1) + fib(n - 2)

print(fib(10))
```

The bootstrap version of Fyr is intentionally small. Each chapter of this book should track real language behavior as the compiler grows.
