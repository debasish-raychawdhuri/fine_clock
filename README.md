# fine_clock

A terminal clock that displays the current time in large ASCII art, centered in your terminal. Built with Rust and ncurses.

## Features

- Large, readable clock display centered in the terminal
- Color-coded: hours (cyan), minutes (green), seconds (yellow), date (magenta)
- Three font styles to choose from
- Live-updating display
- Shows the current date below the clock

## Styles

### Block (default)

```
                              -- fine_clock --

                 ####   ####       ####   ####      ######  ####
                ##  ## ##  ##  ## ##  ## ##  ##  ## ##     ##  ##
                    ## ##  ##     ##  ##     ##     ##     ##  ##
                  ##   ##  ##     ##  ##   ##       #####   ####
                 ##    ##  ##     ##  ##  ##            ## ##  ##
                ##     ##  ##  ## ##  ## ##      ## ##  ## ##  ##
                ######  ####       ####  ######      ####   ####


                             Sunday, March 08, 2026
```

### Fancy (`--fancy` / `-f`)

Uses `/`, `\`, `|`, `-` line characters for a hand-drawn look.

```
                              -- fine_clock --

                /----\   /----\       /----\   /----\       /----\      /|
               |      | |      |  oo |      |       |   oo |      |    / |
                     /  |      |     |      |       |      |      |      |
                 ---/   |      |     |      |   ---/       |      |      |
                /       |      |     |      |       \      |      |      |
               |        |      |  oo |      |       |   oo |      |      |
                \------  \----/       \----/   \----/       \----/    ------


                             Sunday, March 08, 2026
```

### Double (`--double` / `-d`)

Uses Unicode double-line box-drawing characters (`╔═╗║╚╝╠╣`) for a bold, clean look. Requires a terminal with Unicode support (kitty, alacritty, gnome-terminal, etc.).

```
                                -- fine_clock --

         ╔══════╗  ╔══════╗          ╔══════╗  ╔══════╗          ╔══════╗      ╔╗
         ║      ║  ║      ║   ╔╗     ║      ║  ║      ║   ╔╗    ║      ║     ╔╝║
                ║  ║      ║   ╚╝     ║      ║         ║   ╚╝    ║      ║     ║ ║
                ║  ║      ║          ║      ║       ╔╝           ║      ║     ║ ║
         ╔══════╝  ║      ║           ══════╣      ║            ╠══════╗     ╚══╝
         ║         ║      ║                 ║     ╔╝            ║      ║
         ║         ║      ║   ╔╗            ║    ╔╝      ╔╗    ║      ║
         ║      ╗  ║      ║   ╚╝     ║      ║    ║       ╚╝    ║      ║
         ╚══════╝  ╚══════╝          ╚══════╝    ║              ╚══════╝


                                Sunday, March 08, 2026
```

### Analog (`--analog` / `-a`)

A graphical analog clock drawn with the **kitty graphics protocol** (an actual
rasterized image, not text): a round face with tick marks and cyan hour, green
minute, and red second hands, updated every second. Requires a terminal that
supports the kitty graphics protocol (kitty, Ghostty, WezTerm, Konsole, …).
Unlike the text styles, this mode bypasses ncurses and renders directly.

## Dependencies

- Rust (1.85+, edition 2024)
- ncursesw (wide character ncurses)

### Installing ncursesw

**Debian / Ubuntu:**
```bash
sudo apt install libncursesw5-dev
```

**Fedora / RHEL:**
```bash
sudo dnf install ncurses-devel
```

**Arch Linux:**
```bash
sudo pacman -S ncurses
```

**macOS:**
```bash
brew install ncurses
```

## Building

```bash
git clone <repo-url>
cd fine_clock
cargo build --release
```

The binary will be at `target/release/fine_clock`.

## Usage

```
fine_clock [OPTIONS]

Options:
    --fancy,  -f    Line-drawing style (/\|-)
    --double, -d    Double-line box-drawing style (╔═║╗╚╝)
    (default)       Block style (##)

Controls:
    q / Q           Quit
```

## License

MIT
