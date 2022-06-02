# Imagine ![Imagine Logo](https://raw.githubusercontent.com/BD103/Imagine/main/examples/logo.png)

**A console-based pixel art creation tool.**

(Or basically just me messing around with [LodePNG](https://lib.rs/crates/lodepng) and [Termion](https://lib.rs/crates/termion).)

## Running

Download the code and run a release version.

```shell
$ git clone https://github.com/BD103/Imagine.git
$ cd Imagine
$ cargo run --release
```

You can also open this project in Replit. This is where the project is developed, so there is native support. :)

[![Run on Repl.it](https://replit.com/badge/github/BD103/Imagine)](https://repl.it/github/BD103/Imagine)

## Usage

When running Imagine, you will we shown with an 8 by 8 square. On this, you will find a cursor `<>` in the top left corner.

This cursor is how you paint on the canvas.

|Keybind|Action|
|-|-|
|Enter|Paint selected pixel.|
|Arrow keys|Move cursor around the canvas.|
|1-5|Select color on palette.|
|Q, Ctrl+D, or Esc|Exit and save image.|
|Ctrl+C|Exit and **do not** save image.|

You have a set palette of 5 colors. When you are done with an image, it is saved to `out.png`.
