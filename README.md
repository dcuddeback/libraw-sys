# LibRaw Rust Bindings

The `libraw-sys` crate provides declarations and linkage for the `libraw` C library. Following the
`*-sys` package conventions, the `libraw-sys` crate does not define higher-level abstractions over
the native `libraw` library functions.

## Dependencies
In order to use the `libraw-sys` crate, you must have the `libraw_r` library installed where it can
be found by `pkg-config`. `libraw_r` is the reentrant version of LibRaw.

On Debian-based Linux distributions, install the `libraw-dev` package:

```
sudo apt-get install libraw-dev
```

On OS X, install `libraw` with Homebrew:

```
brew install libraw
```

On FreeBSD, install the `libraw` package:

```
sudo pkg install libraw
```

## Usage
Add `libraw-sys` as a dependency in `Cargo.toml`:

```toml
[dependencies]
libraw-sys = "0.1"
```

Import the `libraw_sys` crate and use the functions as they're defined in the native `libraw`
library.

```rust
extern crate libraw_sys as libraw;
```

### Finding Help
Since `libraw-sys` does nothing more than export symbols from the native `libraw` library, the best
source for help is the information already available for the native `libraw`:

* [Homepage](http://www.libraw.org/)
* [Source Code](https://github.com/LibRaw/LibRaw)

## License
Copyright © 2015 David Cuddeback

Distributed under the [MIT License](LICENSE).

*Note:* By using this crate, your executable will link to the `libraw` C library, which is available
under the [LGPL version 2.1, CDDL version 1.0, or LibRaw Software
License](https://github.com/LibRaw/LibRaw/blob/master/COPYRIGHT).
