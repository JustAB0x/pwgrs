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
|----------|---------------|----------------|
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

### Generate 5 default passwords

```bash
$ pwgrs -c 5

{aH3U@M[8p5YbB8|vT2r$WcX8&K]5MfjG}WZmV+Kj
7Lw}1QV|?Q|_vV2d@0B8rD}R4\4Sb8Z0cV]zU%j{
1|e5Y\KtC8v2c2|9}2r}5M7kVt5[5]1{0|tVb5+D
^bW2\JqX4Xk2}2s5\1k\0t]t0|v|7q9WwB6V4X2m
Y2XbA4s4r7J5k222r2rW6XV4k15V5W2Y7X2K5V2X
```

### Generate 10 passwords with 50 characters using only letters

```bash
$ pwgrs -l 50 -a -A -c 10
```

### Generate secrets with 256+ bits entropy

```bash
$ pwgrs secret -c 3

rC5f0m6wC92e1f4p2r4c4d7g1n2c5f0m6wC92e1f4p2r4c4d7g1n2c5f0m
qB3d2m5wC82f1e4p3r3c3d6g2n1c4f3m5wB82e1f3p3r3c3d6g2n1c4f3m
tE6j8n2wF92a3d7p6r6c6g9j4n7f6e2wZ92b3a6p6r6c6g9j4n7f6e2w
```

### Generate WiFi-friendly passwords

```bash
$ pwgrs wifi -c 3

a1b2-c3d4-e5f6-g7h8
i9j0-k1l2-m3n4-o5p6
q7r8-s9t0-u1v2-w3x4
```

### Generate a single password (auto-copied to clipboard)

```bash
$ pwgrs -l 30 -a -A -n

K3n9Qw2R8t7Y6u5I4o3P2a1S0d9F8g
# Copied to clipboard!
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
