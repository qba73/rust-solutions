# Hello

Start a new project

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
