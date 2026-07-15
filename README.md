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

### Analog — retro (`--analog-retro` / `-A`)

The same graphical clock with a vintage look: a warm ivory dial inside a brass
double-bezel, Roman numerals at the cardinals with bold baton markers elsewhere,
tapered black hands, and a red second hand with a counterweight. Also drawn with
the kitty graphics protocol.

### Retro + pendulum (`--retro-with-pendulum` / `-P`)

A cased grandfather-style clock: a hexagonal wooden hood frames the retro brass dial, and
a narrower trunk below houses a **swinging brass pendulum** in a dark cavity,
over a plinth base with feet. The pendulum swings smoothly at a high frame rate
(~60 fps cap) while the second hand ticks once per second like a real clock.
Each frame is uploaded off-screen and swapped in via double buffering, so it
stays flicker-free. A live FPS readout is shown under the clock. Needs a
kitty-graphics terminal.

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
