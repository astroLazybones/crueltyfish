# crueltyfish
Automatic fishing script for cruelty squad. <br>
<br>
## How it works:
Currently it has only been tested on linux but it will work the same on most computers. <br>
<br>
crueltyfish (or for short, autofish) works by automated mouse input using the enigo rust crate and timed input <br>
autofish casts the line after holding for 1 second, and pulls in after 1.5 seconds on loop. <br>
<br>
On all operating systems, **DO NOT** run the binary from anything other than a terminal, or you will have a hard time closing it <br>
<br>
### To compile on windows:
Clone the repo <br>
Open cmd inside the new folder <br>
<br>
###### To build exe:
`cargo build --release` (to run exe from terminal, go to the main folder, then the folder called target, then the folder called release, and run the exe from cmd inside there, **DO NOT DOUBLE CLICK IT**)
###### To run without compile:
`cargo r` or `cargo run`

### To compile on linux:
Clone repo <br>
Open terminal inside directory <br>
<br>
###### To build binary:
`cargo build --release` (to run binary from terminal, go to the repo directory, then the subdirectory called target, then the subdirectory inside that called release, and run the binary from terminal inside there, **DO NOT DOUBLE CLICK THE BINARY**)
###### To run without compile:
`cargo r` or `cargo run` <br>
#### If compiler complains about -lxdo missing
Get the xdotools package for your distro

### To compile on mac
Probably the same as linux and windows
