# Learn-Rust-through-Spinning-Donut

This is a project for me to learn Rust by creating a spinning donut in the terminal. The goal is to practive Rust's programming language syntax and create a spinning donut project in the terminal. I'll try to just Googling the "math" behind the spinning donut instead of finding the programming solution.

## Build requirements

- Rust compiler.

## Learning log

### Hello World

<details>
<summary>Day 1</summary>

1. Install Rust compiler.
2. Create a Hello World Program.

To build the program, run ```rustc``` with the file name we wish to build. Then just run the generated executable.

    $ rustc hello.rs
    $ ./hello

</details>

### Updating the terminal

<details>
<summary>Day2</summary>

I'm testing to render the terminal and update it by creating a progress bar. I'm trying to do it the standard way, without the help of other crate. And as far as I know, there are special ANSI commands that can do something pretty interesting.

- ```\x1b[2J``` is for clearing the screen and set cursor to home.
- ```\x1b[H``` is for returning the cursor to the home position.
- ```\r``` is for returning the cursor to the start of the current line.

I'll try to create a better progress bar or event multiple progress bar to practice my rendering skill on the terminal.

</details>