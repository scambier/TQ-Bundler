# TQ-Bundler

> A _fast_ bundler/watcher/launcher for **TIC-80** games.

**Work In Progress**

- [x] Build a multiple-files project
- [x] Watch changes and auto rebuild
- [x] Launch TIC-80 in watch mode
- [x] Fennel support
- [ ] `init` command
- [ ] Lua support
- [ ] Moonscript support
- [ ] Wren support
- [ ] Squirrel support
- ~~[ ] JavaScript support~~ [Take a look at TSC-80, a TypeScript compiler for TIC-80](https://github.com/scambier/tic80-typescript)

## Installation

### Binary

Since TQ-Bundler is a single-file executable, you can simply download it and place it wherever you'd like.
For easy access from the terminal, it is recommended to place it somewhere in your `PATH`, or inside the folder containing your game sources.

### Cargo

If you have the rustup toolchain, you can also `cargo install --git https://github.com/scambier/TQ-Bundler`

## How To

### Create a new project

```sh
# In your terminal
$ mkdir mygame
```

```sh
# Inside TIC-80:
new fennel
save mygame/game.fnl
```

- Alongside `game.fnl`, create a new file `main.fnl` that will hold your code.
- Cut the code (comments included) from `game.fnl` and paste it inside `main.fnl`

### Include files

```lisp
;; Fennel syntax
(include macros) ;; will look for macros.fnl
(include tools.utils) ;; will look for tools/utils.fnl
```

All included files paths are resolved relative to the file including them. All includes are recursively resolved, with respect to their declaration order.

If a file has already been included, subsequent includes will be discarded.

### Bundle and launch your game

Tl;dr:
```
$ tq-bundler.exe run game.tic main.fnl --watch
````

Full version:
```
$ tq-bundler.exe help run

Bundle and launch your game

USAGE:
    tq-bundler.exe run [FLAGS] [OPTIONS] <game.tic>

FLAGS:
    -w, --watch      Watch for changes and rebuild automatically
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --code <main.fnl>       The "main" code file that will be injected inside the game [default: main.fnl]
    -o, --output <build.fnl>    The entry point of your TIC-80 game [default: build.fnl]
        --tic <path>            Path to the TIC-80 executable. If specified, will launch TIC-80 in watch mode, with your
                                game loaded.

ARGS:
    <game.tic>    The TIC game file in which the bundled code will be injected
```

**/!\\** The default bundled file is named `build.fnl`. TQ-Builder won't check if a file with this name already exists, and will happily overwrite it with each new compilation **/!\\**