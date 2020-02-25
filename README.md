## Embodiment-Of-Fading-Dream

A top down isometric turn-based strategy game making use of open source assets. This was written in Rust for the intro to Rust programming 410P/510P winter 2020 course at Portland State University.
# Instructions
 Execute cargo build && cargo run to install the dependencies for the program before running it.
# Dependencies
```
[dependencies]
```

```
raylib = "1.0.0"
```

```
cgmath = "0.17.0"
```

```
gl = "0.14.0"
```

## Requirements for linux
Install cmake via the command line with the following link.

```
$ wget https://github.com/Kitware/CMake/releases/download/v3.15.2/cmake-3.15.2.tar.gz
```
Then run the following commands.

```
$ tar -zxvf cmake-3.15.2.tar.gz
```

```
$ cd cmake-3.15.2
```

```
./bootstrap
```
## Requirements for windows

Ensure that cmake is installed before running with cargo build, this can be acquired at [here](https://cmake.org/download/). Make sure to select the option to have it set a PATH variable.