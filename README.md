# TQ-Bundler

> A fast bundler/watcher/launcher for your [**TIC-80**](https://tic80.com/) projects.
>
> This README documents the `TQ-Bundler-python` fork. Options such as `--post-build` and `--post-output` are fork-specific and are not part of the original upstream TQ-Bundler.

----

TQ-Bundler streamlines the use of external editors for TIC-80. Split your project into several files, then bundle them and start your game in a single command.

<span>🎈&nbsp;_It's a lightweight single-file executable!_&nbsp;🎈</span>

**[Downloads for Windows and Linux.](https://github.com/scambier/TQ-Bundler/releases)**

Tl;dr:
```bash
$ mkdir my-game
$ cd my-game
$ tq-bundler.exe init lua
$ tq-bundler.exe run game.lua main.lua --tic tic80.exe
```

## Features

- [x] Initializes your multi-files project
- [x] Builds all your files into a single bundle
- [x] Runs an optional post-build command after each bundle (for minifiers/transforms)
- [x] Watches changes to rebuild automatically
- [x] Launches your game inside TIC-80
- [x] Supports Lua, Moonscript, Fennel, Janet, Wren, Squirrel, JavaScript, Ruby, and Python

## Installation

Since TQ-Bundler is a single-file executable, you can simply [download it](https://github.com/scambier/TQ-Bundler/releases) and place it wherever you'd like.
For easy access, I recommend to place it somewhere in your `PATH`, next to TIC-80, or at the root of your games projects folder.

## Local checks before push

Run these fast local checks before publishing a release:

```powershell
# Windows
./scripts/check.ps1
```

```bash
# WSL / Linux
./scripts/check.sh
```

## Releasing binaries (manual workflow)

This repository includes a manual GitHub Actions workflow at `.github/workflows/release-binaries.yml`.

It builds and publishes:
- `tq-bundler-windows-x86_64.zip`
- `tq-bundler-linux-x86_64-musl.tar.gz`
- `SHA256SUMS.txt`

Release flow:
1. Run local checks (`scripts/check.ps1` or `scripts/check.sh`).
2. Open GitHub Actions and run `Release Binaries`.
3. Set `version` to `vX.Y.Z` or `vX.Y.Z-postfix` (example: `v2.3.0-etm.1`).
4. Optionally set `set_as_latest`.

Notes:
- `Cargo.toml` version stays unchanged (useful when maintaining a fork of upstream).
- Release tags can carry fork-specific postfixes (for example `-etm.1`).

## Download from another repository

You can download assets from the latest release with the GitHub API.

```bash
REPO="OWNER/REPO"
ASSET="tq-bundler-linux-x86_64-musl.tar.gz"

ASSET_URL=$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" \
  | jq -r --arg name "$ASSET" '.assets[] | select(.name == $name) | .browser_download_url')

curl -fL "$ASSET_URL" -o "$ASSET"
```

## Usage

TQ-Bundler has 2 sub-commands:
- `init` to quickly initialize a multi-file project in the language of your choice
- `run` to bundle the files and start TIC-80 with your game

### Create a project

```bash
$ mkdir my-game
$ cd my-game
$ tq-bundler.exe init lua # or moon, wren, fennel, janet, squirrel, js, ruby, python
```

This will create the files `game.lua` (containing the sprites and sounds) and `main.lua` (the code entry point)

### Include your files

In all languages, paths are in the format of `"folder.subfolder.file"`. Paths are resolved relatively from the root of your project (see example [here](https://github.com/scambier/TQ-Bundler/blob/master/tests/lua/sub/nested.lua)).

All `include`s are recursively resolved, with respect to their declaration order. `include`s **must** be on their own line (1 per line).

```lua
-- Lua syntax
include "macros" -- will look for ./macros.lua
include "tools.utils" -- ./tools/utils.lua
```

```lua
-- Moonscript syntax
include "macros" -- ./macros.moon
include "tools.utils" -- ./tools/utils.moon
```

```lisp
;; Fennel syntax
(include "macros") ;; ./macros.fnl
(include "tools.utils") ;; ./tools/utils.fnl
```

```janet
# Janet syntax
(include "macros") # ./macros.janet
(include "tools.utils") # ./tools/utils.janet
```

```c
// Wren syntax
include "macros" // ./macros.wren
include "tools.utils" // ./tools/utils.wren
```

```js
// Squirrel syntax
include("macros") // ./macros.nut
include("tools.utils") // ./tools/utils.nut
```

```js
// JavaScript syntax
include("macros") // ./macros.js
include("tools.utils") // ./tools/utils.js
```

```ruby
# Ruby syntax
include "macros" # ./macros.rb
include "tools.utils" # ./tools/utils.rb
```

```python
# Python syntax
include("macros") # ./macros.py
include("tools.utils") # ./tools/utils.py
```

### Bundle and launch your game

> ⚠️ Be careful to respect the arguments order, or your game won't launch. It's always `tq-bundler.exe run GAME MAIN`.

```sh
# Bundle the game into `build.lua`:
$ tq-bundler.exe run game.lua main.lua
```

```sh
# Bundle and launch through TIC-80, then rebuild when files change:
$ tq-bundler.exe run game.lua main.lua --tic path/to/tic80.exe
```
This way, you can edit code inside your IDE and edit assets inside TIC-80 _at the same time_. Changes are applied after a `ctrl+r`.

```sh
# Bundle, then run a post-build minifier (fork-only):
$ tq-bundler.exe run game.py main.py --post-output build.min.py --post-build "python scripts/minify_tic80_build.py {input} {output}"
```

```sh
# Bundle + watch + TIC-80 launch, always applying post-build on rebuilds (fork-only):
$ tq-bundler.exe run game.py main.py --tic path/to/tic80.exe --post-output build.min.py --post-build "python scripts/minify_tic80_build.py {input} {output}"
```

`--post-build` is a command template executed after bundling. The placeholders `{input}` and `{output}` are replaced with file paths after argument tokenization, so placeholders can be used as standalone arguments without extra quotes. If `--post-output` is set, TIC-80 loads that file as runtime code; otherwise it loads the regular bundle output.

```sh
# View all options:
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

**TIC-80 doesn't correctly reload my code**

If you're building TIC-80 yourself, make sure to use the correct settings

```sh
$ cd <path-to-tic>/build
$ cmake -G "Visual Studio 16 2019" -DBUILD_PRO=On -DCMAKE_BUILD_TYPE=MinSizeRel ..
$ cmake --build . --config MinSizeRel --parallel
```

### TypeScript support

[Take a look at TSC-80, a TypeScript compiler for TIC-80](https://github.com/scambier/tic80-typescript)
