# Hello, World

Every Rust journey starts with a small program that prints a greeting to the
terminal. This page walks through the classic *Hello, World* example.

## The program

The source lives in [`hello_world.rs`](https://github.com/Huauauaa/hi-rust/blob/main/hello_world.rs)
at the root of the repository:

<<< ../../hello_world.rs{rust}

- `fn main()` defines the entry point. Every executable Rust program starts here.
- `println!` is a macro (note the `!`) that prints a line to standard output.

## Run it

You need the [Rust toolchain](https://www.rust-lang.org/tools/install) installed.
From the project root:

```sh
rustc hello_world.rs
./hello_world
```

On Windows, run `hello_world.exe` instead of `./hello_world`.

You should see:

```text
Hello, world!
```

## Next steps

Once this works, try changing the string inside `println!` and recompiling.
When you are ready for larger projects, see [包管理](/syntax/package_management) to scaffold a crate with
dependencies and tests built in.
