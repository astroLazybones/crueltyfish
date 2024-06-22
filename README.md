# crueltyfish
Automatic fishing script for cruelty squad.
<br><br>

## How it works:
Currently it has only been tested on linux but it will work the same on most computers.
<br><br>

crueltyfish (or for short, autofish) works by automated mouse input using the [enigo](https://crates.io/crates/enigo) and timed input <br>
autofish casts the line after holding for 1 second, and pulls in after 1.5 seconds on loop.
<br><br>

On all operating systems, **DO NOT** run the binary from anything other than a terminal, or you will have a hard time closing it
<br><br>

### To compile on windows:
Clone the repo <br>
Open cmd inside the new folder
<br><br>

*building exe:*
<br>
`cargo build --release` (to run exe from cmd, go to the main folder, then the folder called target, then the folder called release, and run the exe from cmd inside there, **DO NOT DOUBLE CLICK IT**)
<br>
<br>
*running without exe:*
<br>
`cargo r` or `cargo run`
<br><br>

### To compile on linux:
Clone repo <br>
Open terminal inside directory
<br><br>

*building binary:*
<br>
`cargo build --release` (to run binary from terminal, go to the repo directory, then the subdirectory called target, then the subdirectory inside that called release, and run the binary from terminal inside there, **DO NOT DOUBLE CLICK THE BINARY**
<br>
<br>
*running without binary:*
<br>
`cargo r` or `cargo run` <br>
#### If compiler complains about -lxdo missing
Get the xdotools package for your distro
<br><br>

### To compile on mac
Probably the same as linux and windows
<br><br><br>
# NOTABLE THINGS
## Timing
If the line gets pulled a little late, you can pause for a split second and wait for another fish to see the result.
<br>
This is because you have a chance at catching a fish every second in game (reccomended to turn on timer)
## Positioning
Make sure to stand close to the fishing spot (like, as close as you safely can)
<br>
due to the fact it is timed inputs, not smart detection, when you cast the rod at long distances, it can try to pull the rod while its still in the air
## Features
Here is a list of things that are here and arent here:
<br>
- [ ] Detection if the game is the main window
- [ ] Catching based on if a fish in on screen
- [ ] Uranus (this planet is something that isnt in the code)
- [x] Very few lines of code
- [x] Ability to cast and reel in fishing rod automatically
- [x] Ability to catch the DOSfish if set up properly (i caught 4 DOSfish total)
