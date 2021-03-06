# mycon

An interpreter for the esoteric programming language [Befunge-98].

[Befunge-98]: https://esolangs.org/wiki/Funge-98

## Installation

mycon can be installed with `cargo`:

```
$ cargo install mycon
```

This will install the executable `mycon` into `~/.cargo/bin`. Make sure that
this directory is in your `$PATH`.

## Usage

To interpret a file `foo.b98`, use

```
$ mycon foo.b98
```

The `--help` flag gives information about available options.

mycon will first try to read the file as UTF-8. If this fails, it will assume it
is in ISO-8859-1 (Latin-1) and treat each byte as one character.

## License

Copyright 2018 Johannes M. Griebler

mycon is released under the terms of the GNU General Public License version 3,
or (at your option), any later version. See [COPYING](COPYING) for details.
