# Dopefish Decoder

![Dopefish](https://github.com/chrishulbert/dopefish-decoder/blob/main/Dopefish.png?raw=true)

This decodes/reads the graphics (and levels, eventually) for Commander Keen 4-5-6.

To run, clone this repo, install Rust, and run: `make 4`. It should generate a bunch of png files in the current folder.

## Keen 5 and 6

![Robo Red](https://github.com/chrishulbert/dopefish-decoder/blob/main/RoboRed.png?raw=true)

* Shareware Keen 4 is included, but Keen 5 and 6 are BYO.
* Keen 5-6 files should be placed in data/keen5 and data/keen6.
* Note you must decompress the EXEs first, as described below.
* Then you can run `make 5` or `make 6`.

## Decompressing EXE files

![Blooguard](https://github.com/chrishulbert/dopefish-decoder/blob/main/Blooguard.png?raw=true)

* Keen 5 and 6 EXE files must be decompressed.
* Download UNLZEXE (16 bit version) from here: https://keenwiki.shikadi.net/wiki/UNLZEXE
* UNLZEXE is also in this repo.
* Put it in the same folder as keen4-6.exe
* In DOSBox, run: UNLZEXE KEEN?.EXE

## References

https://moddingwiki.shikadi.net/wiki/Commander_Keen_4-6
https://moddingwiki.shikadi.net/wiki/EGAGraph_Format
