Hex Dump
========

Format bytes as a classic hex dump.

Binary
------

### Synopsis

`hexdump [-n `_`length`_`] [-s `_`offset`_`] `_`file`_`...`

### Description

Display a hex dump of the specified files. Woefully incomplete, but it works.

* `-n `_`length`_

  Interpret only _`length`_ bytes of the inputs. Only accepts decimal numbers.

* `-s `_`offset`_

  Skip _`offset`_ bytes from the inputs. Only accepts decimal numbers.

Library
-------

Not available on [crates.io](https://crates.io) sorry...

```rust
extern crate hexdump;
use hexdump::hexdump;

const BYTES: &'static [u8] = b"\x00\x11\x22\x33\x44\x55\x66\x77\x88\x99\xAA\xBB\xCC\xDD\xEE\xFF";

assert_eq!(format!("{}", hexdump(BYTES, 32)),
	"00000020:  00 11 22 33 44 55 66 77  88 99 AA BB CC DD EE FF  |..\"3DUfw........|\n");
```

More in the documentation, which is only available after you run `cargo doc`.

License
-------

MIT, see license.txt
