# Hello

## Start a new project

```shell
➜  01_hello git:(main) cargo new hello
     Created binary (application) `hello` package
```

Check initial structure

```shell
➜  01_hello git:(main) ✗ ls
hello
➜  01_hello git:(main) ✗ tree
.
└── hello
    ├── Cargo.toml
    └── src
        └── main.rs

3 directories, 2 files
```

Run the program

```shell
➜  hello git:(main) ✗ cargo run
   Compiling hello v0.1.0 (/Users/jakub/edu/rust-projects/rust-solutions/01_hello/hello)
    Finished dev [unoptimized + debuginfo] target(s) in 0.21s
     Running `target/debug/hello`
Hello, world!
```

Run the program with the quiet option

```shell
➜  hello git:(main) ✗ cargo run -q
Hello, world!
➜  hello git:(main) ✗ ls -l
```

## Run tests

1. Create tests dir and add a test.

```shell
➜  hello git:(main) ✗ tree -L 2
.
├── Cargo.lock
├── Cargo.toml
├── src
│   └── main.rs
├── target
│   ├── CACHEDIR.TAG
│   ├── debug
│   └── tmp
└── tests
    └── cli.rs

6 directories, 5 files
```

1. Run tests:

```shell
➜  hello git:(main) ✗ cargo test
   Compiling hello v0.1.0 (/Users/jakub/edu/rust-projects/rust-solutions/01_hello/hello)
    Finished test [unoptimized + debuginfo] target(s) in 0.25s
     Running unittests src/main.rs (target/debug/deps/hello-2d14051fb1909b13)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```
