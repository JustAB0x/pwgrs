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

If you run `pwgrs` without the `-l` / `--length` option it will determine the length based on the selected charsets to get **at least 256 bits of entropy**. Fewer charsets selected, will result in a longer password.

Without any charset flags (`-a`, `-A`, `-n`, or `-s`) set, all possible charsets will be used.

## Examples

#### `pwgrs`

Length defaults to 40, because 91 distinct chars are available.

```
3W56|R\8DQz]?qV9Gv@Wi%s!*~d12_*mZ&qTm?Q?
```

#### `pwgrs -a`

Length defaults to 55, because 26 distinct chars are available.

```
aachgovgbekfrgebxfscruwjgrmzzxsicakzwqtlcrwarpqntkpvydk
```

#### `pwgrs -A`

Length defaults to 55, because 26 distinct chars are available

```
YPFNCITWYLOGEEIPLWUWIOUJDTIVJFRTXQSDAORFOGXFUHZFLDYXKZY
```

#### `pwgrs -Aa`

Length defaults to 45, because 52 distinct chars are available.

```
ugSVnQabbnkKhxdCrhleBCyUnlLqUojUMVTUnJSfMHTTJ
```

#### `pwgrs -n`

Length defaults to 78, because 10 distinct chars are available.

```
404033964956362206872986540699097332184979913456121844514705164504017662414846
```

#### `pwgrs -s`

Length defaults to 54, because 29 distinct chars are available.

```
)<:}=_)=/{@!:/[/[.\])-^~</<,)>.\,}=^*(&}([&#_,=&^\&|(=
```

#### `pwgrs -l 20`

Length is hard-set to 20.

```
?vi_H1tH6_F}1a3.5lnO
```

#### `pwgrs -l 20 -c 5`

```
33gM6q_NY+)QmFu?hSon
1!sF~127(&Sb[2b)c/J}
9hi0yjw_bGOF92LIph=0
#1mMVsk5I7e&sfg>p9U0
M5WdLId/00!}h=v741mN
```

### Sub commands

#### `pwgrs secret`

```
o93Zwf73lNphrq997mt76j0c5M1P7IQnyG86rDCBf98
```

#### `pwgrs wifi`

```
2fad-03e3-930h-ko1s
```
