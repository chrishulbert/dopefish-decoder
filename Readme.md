# Dopefish Decoder

![Dopefish](https://github.com/chrishulbert/dopefish-decoder/blob/main/Dopefish.png?raw=true)

This decodes/reads the graphics (and levels, eventually) for Commander Keen 4-5-6.

To run, clone this repo, install Rust, and run: `make 4`. It should generate a bunch of png files in the current folder.

## Decompressing EXE files

* You need to decompress the EXE files first before anything else.
* Download UNLZEXE (16 bit version) from here: https://keenwiki.shikadi.net/wiki/UNLZEXE
* Put it in the same folder as keen4-6.exe
* In DOSBox, run: UNLZEXE KEEN?.EXE

## Keen 5 and 6

* Shareware Keen 4 is included, but Keen 5 and 6 are BYO.
* Keen 5-6 files should be placed in data/keen5 and data/keen6.
* Exe files must be decompressed as described above.
* Then you can run `make 5` or `make 6`.

## References

https://moddingwiki.shikadi.net/wiki/Commander_Keen_4-6
https://moddingwiki.shikadi.net/wiki/EGAGraph_Format
