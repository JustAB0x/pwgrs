# @pwgrs

CLI password generator with smart defaults and a built-in copy-to-clipboard feature.

## Usage

```
Secure password generator

Usage: pwgrs [OPTIONS] [COMMAND]

Commands:
  secret  Creates secret with at least 256 bits of entropy
  wifi    Creates a wifi friendly password
  help    Print this message or the help of the given subcommand(s)

Options:
  -l, --length <LENGTH>     Length of passwords
  -c, --count <COUNT>       Amount of passwords
  -a, --alphabet            Use ALPHABET (a-z) charset
  -A, --alphabet-uppercase  Use ALPHABET_UPPERCASE (A-Z) charset
  -n, --numbers             Use NUMBERS (0-9) charset
  -s, --special             Use SPECIAL (*, %, -, ...) charset
  -h, --help                Print help
  -V, --version             Print version
```

## Defaults

If you run `pwgrs` without a `--length` option it will determine the length based on the available charsets to get to **at least 256 bits of entropy**. The less different characters that are available through those charsets, the longer the resulting password will get.

Without charset flags (`-a`, `-A`, `-n`, or `-s`) present, it uses all possible charsets.

## Examples

#### `pwgrs`

Length defaults to 40, because 91 distinct chars are available.

```
95VlP}C~uHw737KyPS-0Fb36TT8CGeNz4,hx;s[O
```

#### `pwgrs -a`

Length defaults to 55, because 26 distinct chars are available.

```
jvvntwckazacgdizlebskxmwqesapelcdbdcxaqcxkheznvfsgzdsbc
```

#### `pwgrs -A`

Length defaults to 55, because 26 distinct chars are available

```
OWLMMTRJOQFPABBHVBQFCQKZEXNWJIFOSDYZWBJSDXSUUQBDOJXXGDP
```

#### `pwgrs -Aa`

Length defaults to 45, because 52 distinct chars are available.

```
huTjuxxLmiBsVxWQTlVVbezboZCBfAsjxHPSToHajPqIi
```

#### `pwgrs -n`

Length defaults to 78, because 10 distinct chars are available.

```
122099737963124140099587161440104945150736189544627271670613369052503984616256
```

#### `pwgrs -s`

Length defaults to 53, because 29 distinct chars are available.

```
;/&~]_[[[[]{^<%^;-*.*,;`-[|?|>/^~<:&/_<<=*\!>;[>?%~<(
```

#### `pwgrs -l 20`

Length is hard-set to 20.

```
72T4M+w3[I2xCMvvCb]Y
```

#### `pwgrs -l 20 -c 5`

```
i*w:2BtH0KFegl.z91N8
)9jRQF9RD5eq89\4LuD\
_o|3E]pC^44R6M]43M[W
27s+zgH0U3Et5o/l4zq7
8g1`}/,0C0R\EB1uG{2+
```

### Sub commands

#### `pwgrs secret`

```
76mgeX1V4q64qdS0f0wZp72klQapsJ8b7At8vvl4XfR
```

#### `pwgrs wifi`

```
wmy9-7bgq-g469-9h2r
```
