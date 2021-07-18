# TQ-Bundler

> A fast bundler/watcher/launcher for your [**TIC-80**](https://tic80.com/) projects.

!! DUE TO A PENDING TIC-80 ISSUE, THIS BUNDLER IS NOT YET COMPATIBLE WITH TIC-80 0.90 !!

[Downloads for Windows and Linux](https://github.com/scambier/TQ-Bundler/releases)

TQ-Builder streamlines the use of external editors for TIC-80. Split your project into several files, then bundle them and start your game in a single command.

🎈&nbsp;_It's a lightweight single-file executable!_&nbsp;🎈

Tl;dr:
```bash
$ mkdir my-game
$ cd my-game
$ tq-bundler.exe init lua
$ tq-bundler.exe run game.lua main.lua --watch --tic tic80.exe
```

## Features

- [x] Initializes your multi-files project
- [x] Builds all your files into a single bundle
- [x] Watches changes to rebuild automatically
- [x] Launches your game inside TIC-80 in watch mode
- [x] Supports Lua, Moonscript, Fennel, Wren, Squirrel and JavaScript

## Installation

Since TQ-Bundler is a single-file executable, you can simply [download it](https://github.com/scambier/TQ-Bundler/releases) and place it wherever you'd like.
For easy access, I recommend to place it somewhere in your `PATH`, next to TIC-80, or at the root of your games projects folder.

## Usage

TQ-Bundler has 2 sub-commands:
- `init` to quickly initialize a multi-file project in the language of your choice
- `run` to bundle the files and start TIC-80 with your game

### Create a project

```bash
$ mkdir my-game
$ cd my-game
$ tq-bundler.exe init lua # or moon, wren, fennel, squirrel, js
```

This will create the files `game.lua` (containing the sprites and sounds) and `main.lua` (the code entry point)

### Include your files

```lua
-- Lua, Moonscript syntax
include "macros" -- will look for ./macros.lua
include "tools.utils" -- ./tools/utils.lua
```

```lisp
;; Fennel syntax
(include "macros") ;; ./macros.fnl
(include "tools.utils") ;; ./tools/utils.fnl
```

```c
// Wren syntax
include "macros" // ./macros.wren
include "tools.utils" // ./tools/utils.wren
```

```js
// Squirrel, JavaScript syntax
include("macros") // ./macros.nut
include("tools.utils") // ./tools/utils.nut
```

All included files paths are resolved relative to the file including them. All includes are recursively resolved, with respect to their declaration order. `include`s must be on their own line (1 include per line).

### Bundle and launch your game

Simply bundle:
```sh
$ tq-bundler.exe run game.lua main.lua
```

Bundle, watch, launch TIC-80:
```sh
$ tq-bundler.exe run game.lua main.lua --watch --tic path/to/tic80.exe
```

View all options:
```sh
$ tq-bundler.exe help run
```

**/!\\** The default bundle file is named `build.lua` (or `.wren` etc.). TQ-Builder won't check if a file with this name already exists, and will happily overwrite it with each new compilation **/!\\**

The bundle file is annotated with comments delimiting the start and end of all included files.

## Addendum

**Why not use `require` or `import` statements that already exist in several of these languages?**

TQ-Bundler literally replaces `include` statements with the raw contents of said included files. Since statements like `require` or `import` work differently, I wanted to avoid any confusion.

**The bundle file only contains the code, how can I bundle *this* with the assets file?**

Simply `ctrl+s` inside TIC-80, and your whole game (code + assets) will be saved to `game.lua`

For convenience, TQ-Bundler leaves the game file (the one containing your sprites & sounds) alone. This allows you to edit those assets inside TIC-80 and your code inside your external editor, without risking to overwrite one or the other.

### TypeScript support

[Take a look at TSC-80, a TypeScript compiler for TIC-80](https://github.com/scambier/tic80-typescript)
