# pngme

This small tool allows you to hide secret messages in PNG files by encoding them as ancillary chunks. It is based on the excellent [pngme book](https://picklenerd.github.io/pngme_book/introduction.html).

```bash
$ pngme encode Example.png ruSt "I like reindeer"
I like reindeer

$ pngme decode Example.png ruSt 
I like reindeer
```

## Usage

```
Usage: pngme <COMMAND>

Commands:
  encode  Encodes a secret message into the given PNG file
  decode  Tries to decode a secret message from the given PNG file
  remove  Tries to removes a secret message from the given PNG file
  print   Prints given PNG file
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

