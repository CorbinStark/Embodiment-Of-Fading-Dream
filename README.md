# Embodiment-Of-Fading-Dream-RPG
https://github.com/CorbinStark/Embodiment-Of-Fading-Dream

Forest Pearson <fpearson@pdx.edu>

Corbin Stark <costark@pdx.edu>

Max Van Raden <vanraden@pdx.edu>

A top down isometric turn-based strategy game making use of open source assets. This was written in Rust for the intro to Rust programming 410P/510P winter 2020 course at Portland State University. Utilizes open source assets: https://0x72.itch.io/16x16-dungeon-tileset
## Instructions
 Execute cargo build && cargo run to install the dependencies for the program before running it, may take a minute to install raylibs.
## Dependencies
```
[dependencies]
```

```
raylib = "1.0.0"
```

```
rand = "0.7.3"
```

```
byteorder = "1.3.4"
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

Ensure that cmake is installed before running with cargo build, this can be acquired [here](https://cmake.org/download/). Make sure to select the option to have it set a PATH variable.

## Testing
Testing via unit test for our project was somewhat limited due to the nature of many of our functions returning something that is difficult to verify using the assert macro. Because of this, most of our unit tests focus on functions that return either boolean or i32 values, as those outputs are easily verified. 

Specifically, using unit tests, we tested the heuristic functions responsible for making floodfill function. This was valuable, as floodfill is a relatively complex function, and unit tests verifying the reliability of the underlying heuristics made the process of debugging easier. 

As part of our combat system, we implemented hit chance using RNG. Using unit tests, we verified that expected results would be returned consistently based on the given test unit's base hit chance.

Finally, we used unit testing to ensure that selecting units and menu options was working correctly by testing the ismoused function.

## Results
Overall the project went well and we successfully developed the basic framework for the game we set out to make. We wish we could have developed and fleshed out the game more but are satisfied that we managed to accomplish all the core functions. 

A few things that didn't work were our old methods of using queues in C++ as well as references and direct writing of ints to a file, although all of this was solved after sufficiently looking up alternative Rust based solutions. We did also have to end up using two allowances for clippy due to two separate functions having too many arguments, this was done due to it making the code cleaner because of their complex nature as well as allowing us to not unnecessarily duplicate the code outside of the helper functions.

As to what worked well the map editor and unit selection came out quite well with only a few borrow checker errors that had to be solved at the time, the main-menu and state transition system also worked well and was fairly simple to make use of for later development once it was set up.

For the future, as stated before fleshing out the game would be desirable with more levels,enemies,combat depth and content in general. On the more technical aspect setting up dynamic resolution so that all assets are properly adjusted would be a nice addition later on.