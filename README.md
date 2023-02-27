# clspv-sys

## Build instructions

Fetch the `clspv` submodule:

```bash
git submodule update --init --recursive
```

Build the library using cargo:

```bash
cargo build
```

## Building on Linux

Install CMake using your package manager:

```bash
apt install cmake

# or:

dnf in cmake
```

## Building on macOS

Install CMake:

```
brew install cmake
```

## Building on Windows

Install the following prerequisites:

* [Git](https://git-scm.com/downloads)
* [CMake](https://cmake.org/download/).
* [Python 3](https://www.python.org/downloads/).
* [Build Tools for Visual Studio](https://aka.ms/vs/17/release/vs_BuildTools.exe).
* [Ninja](https://github.com/ninja-build/ninja/releases)

You might have to [enable long paths](https://learn.microsoft.com/en-us/windows/win32/fileio/maximum-file-path-limitation?tabs=powershell) for CMake to work properly.
