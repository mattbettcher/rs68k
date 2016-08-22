# rs68k
68k cpu emulator written in Rust.

TODO
-------------
* Everything it seems...  ;)
* Separate memory decoding from main cpu emulation. I want to 'pass' the memory decoding and storage to the cpu.
* Need to decide on opcode matching approach.
  * Currently, I plan on building a table of all 60k opcodes in either...
    * A HashMap
    * Directly in an array of function pointers
