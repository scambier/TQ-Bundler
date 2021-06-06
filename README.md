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

All included files are relative to the file including them. All includes are recursively resolved, with respect to their declaration order.

If a file has already been included, subsequent includes will be discarded.

```lisp
;; Fennel syntax
(include macros) ;; will look for macros.fnl
(include tools.utils) ;; will look for tools/utils.fnl
```

### Bundle and launch your game

To make a simple build:
```sh
$ tq-bundler.exe game.fnl --code main.fnl
```

Watch changes to automatically rebuild, and launch TIC-80
```sh
$ tq-bundler.exe game.fnl --code main.fnl --watch --tic path/to/tic80.exe
```

You can execute `tq-bundler.exe` without any option, it will look for the files `game.tic` and `main.fnl`.

**/!\\** The default bundled file is named `build.fnl`. TQ-Builder won't check if a file with this name already exists, and will happily overwrite it with each new compilation **/!\\**