# pwgrs

CLI password generator with sensible defaults and a built-in copy-to-clipboard feature.

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

If you run `pwgrs` without the `-l` / `--length` option it will determine the length based on the selected charsets to get **at least 256 bits of entropy**. Fewer charsets selected, will result in a longer password.

Without any charset flags (`-a`, `-A`, `-n`, or `-s`) set, all possible charsets will be used.

## Examples

#### `pwgrs`

Length defaults to 40, because 91 distinct chars are available.

```
(I]#<qJf+5J97{<54YW^6h:9~OFtLCf:EsG>Opj>
```

#### `pwgrs -a`

Length defaults to 55, because 26 distinct chars are available.

```
cypnfhbprxrolfvqeijmtseglnjygpzwajifxkjqaexnwroicmuomqx
```

#### `pwgrs -A`

Length defaults to 55, because 26 distinct chars are available

```
UPZYYKFKWQCBYQXOBMQOFUHZDHGIYNDCBYHJIFATPFZBOMHFJDBCRWN
```

#### `pwgrs -Aa`

Length defaults to 45, because 52 distinct chars are available.

```
LqAOZvforNZerWfNZsHztQuRawusOSeRcqjiZxccrXbqW
```

#### `pwgrs -n`

Length defaults to 78, because 10 distinct chars are available.

```
404799749675656288633240846625868579212785738695528681525958300727279818763409
```

#### `pwgrs -s`

Length defaults to 54, because 29 distinct chars are available.

```
}>!*><;<&{>@*|#-.}_==_+^^)&!\]#!~*!>.|%;+,}-*\?&;[.\*)
```

#### `pwgrs -l 20`

Length is hard-set to 20.

```
IwMo0u,M;3l-06+E59uc
```

#### `pwgrs -l 20 -c 5`

```
c;]D9D*:294I66;25x#{
=z]?7jEvB6nEU(rADA4[
F!2&Xz|2f#D}X71Lo*^u
c78S/,P7w*c_XgK4(m33
a#v<F)<nOOqXZ27yp0Cv
```

### Sub commands

#### `pwgrs secret`

```
7Vh0o6f852375GFr1ZOlssZX2bNPIS5yTU0p2258DiJ
```

#### `pwgrs wifi`

```
j527-graj-1t79-m4lx
```
