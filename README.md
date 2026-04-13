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

### Generate 5 default passwords

```bash
$ pwgrs -c 5

Ln29|6<<f:h8{~z2[~}W?aB<m0Huo3E4rnvDx86{
18y<wG.n@G5O71WjoD8\,OW/IpIY2/}0,@F4Hy_t
26fLwf9hiBQ[.9dxsYD.2;\7*}(Trj4hQFJpi3r=
r)|y1o495N93z!KmiWC5eoMFSra}WQ2+Uc49r[N&
cOOuwZ4+xK[tf,enwkI0_7j982N(2#|i7W6BRjc5
```

### Generate passwords with 20 characters

```bash
$ pwgrs -l 20 -c 5

&yK38&tA2.rU%,P*e=D>
R&u>#f+BlimQdzv08BU/
r>RjgD)gJ}s5f8_Q<NTw
!w1ye63_O?=5DZ2&1{V1
N{XBOx\G0yGpwA(29k2@
```

### Generate secrets with 256+ bits entropy

```bash
$ pwgrs secret -c 3

CxWrw55JXR7ZGsO8L1MICYtlIR8BW7hA5gLIMW6RO50
399mG9lXwZOzfRTn8ywUScYU1rJ70R359j26BZcggtV
C21uh9FBwAlNHk2QOEonjHYxmX0KdQC76zH9r709ZBD
```

### Generate WiFi-friendly passwords

```bash
$ pwgrs wifi -c 3

e8oc-285s-103c-322f
j9wa-86k1-p25j-d5f6
52nc-09mi-8256-bx03
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
