# pwgrs

A secure command-line password generator built in Rust with sensible defaults and a built-in copy-to-clipboard feature.

## Features

- **Secure**: Uses ChaCha20 CSPRNG with OS entropy seeding per password
- **EntropyCalculator**: Automatically calculates password length for desired entropy (default: 256 bits)
- **Multiple modes**: Default, secret (256+ bits entropy), and WiFi-friendly (formatted)
- **Flexible charsets**: Include/exclude letters, numbers, symbols
- **Clipboard copy**: Single passwords are automatically copied to clipboard
- **Cross-platform**: Works on Linux, macOS, and Windows

## Installation

### From source

```bash
cargo install --git https://github.com/JustAB0x/pwgrs
```

### Arch Linux (AUR)

```bash
yay -S pwgrs
```

## Usage

```bash
pwgrs [OPTIONS] [COMMAND]
```

### Commands

- `secret` - Creates secrets with at least 256 bits of entropy
- `wifi` - Creates WiFi-friendly passwords (formatted: XXXX-XXXX-XXXX-XXXX)
- `help` - Print this message or the help of the given subcommand(s)

### Options

| Option | Description |
|--------|-------------|
| `-l, --length <LENGTH>` | Length of passwords |
| `-c, --count <COUNT>` | Amount of passwords to generate |
| `-a, --alphabet` | Use lowercase letters (a-z) |
| `-A, --alphabet-uppercase` | Use uppercase letters (A-Z) |
| `-n, --numbers` | Use numbers (0-9) |
| `-s, --special` | Use special characters (*, %, -, etc.) |
| `-h, --help` | Print help information |
| `-V, --version` | Print version information |

## Defaults

### Length calculation

Without `-l/--length`, pwgrs calculates the minimum length needed for **at least 256 bits of entropy**:

| Charsets | Distinct chars | Default length |
|----------|----------------|----------------|
| All (-aAns) | 91 | 40 |
| Lowercase only (-a) | 26 | 55 |
| Uppercase only (-A) | 26 | 55 |
| Numbers only (-n) | 10 | 78 |
| Numbers + lowercase (-an) | 36 | 46 |
| Special only (-s) | 29 | 54 |

Fewer charsets = longer required password

### Charset defaults

Without any charset flags (`-a`, `-A`, `-n`, `-s`), **all charsets are enabled**.

## Examples

### Generate default password

```bash
$ pwgrs

S7RSC8[/KLO%l_d}5i)3;IcwJn#.,o*#T1-25:A]
```

### Generate lowercase password

```bash
$ pwgrs -a

bhfxntyqiwxxyzedhvzofihfsheugguhjlvuylwxkzxubolnarwuvin
```

### Generate uppercase password

```bash
$ pwgrs -A

NDPIJFSUUUQGPLXRYDILAPBVJSICPOGVQKAWCRKIJJAKGPSDRMKFJZC
```

### Generate mixed case password

```bash
$ pwgrs -A -a

gkEHXbpySepuZwaaUDBbSZANHAeLuLWCBFyKnjMYARwMA
```

### Generate numbers-only password

```bash
$ pwgrs -n

975187521195848424218005701516308518870283325553323169274729257723409149714427
```

### Generate special characters password

```bash
$ pwgrs -s

\_[-\<=)/)~<,>@]{{|+{\?^\-=+[.|-_:+%.,==_))?.*<@=]/{^)
```

### Generate 20-character password

```bash
$ pwgrs -l 20

fH}8IbI2r.RPWA6F8uA9
```

### Generate 5 passwords, 20 characters each

```bash
$ pwgrs -l 20 -c 5

fR@DrEP)4~g(3-+&D|ea
A6bvj?mvT71:mQ74>+22
73!u89xYR#;jZwqL[qD(
so&50&7;?31Z9<=I-6=9
4=rG3E3;tPs\,l~ws_Gg
```

## Entropy Calculator

The password length is calculated using:

```
length = ceil(bits * log(2) / log(charset_size))
```

For 256 bits of entropy:
- 26 chars (letters): `ceil(256 * 0.693 / 3.258) = 55`
- 10 chars (numbers): `ceil(256 * 0.693 / 2.303) = 78`
- 62 chars (letters+numbers): `ceil(256 * 0.693 / 4.127) = 43`
- 91 chars (all): `ceil(256 * 0.693 / 4.511) = 40`

## License

MIT or Unlicense (at your option)

## Security

- Uses `ChaCha20` cryptographically secure PRNG from `rand_chacha`
- Seeded from OS entropy (`/dev/urandom`, `RandomSecure`, etc.)
- One RNG instance per password generation
- No password data stored or logged
