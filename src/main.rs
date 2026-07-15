use std::env;
use std::ffi::CString;
use std::io::{Read, Write};
use std::f64::consts::PI;
use std::sync::atomic::{AtomicBool, Ordering};

use chrono::{Local, Timelike};

const DIGIT_HEIGHT: usize = 7;

// -- Block style (##), 6 wide --
const BLOCK_W: usize = 6;
const BLOCK_DIGITS: [[&str; DIGIT_HEIGHT]; 10] = [
    [" #### ", "##  ##", "##  ##", "##  ##", "##  ##", "##  ##", " #### "],
    ["  ##  ", " ###  ", "  ##  ", "  ##  ", "  ##  ", "  ##  ", " #### "],
    [" #### ", "##  ##", "    ##", "  ##  ", " ##   ", "##    ", "######"],
    [" #### ", "##  ##", "    ##", "  ### ", "    ##", "##  ##", " #### "],
    ["##  ##", "##  ##", "##  ##", "######", "    ##", "    ##", "    ##"],
    ["######", "##    ", "##    ", "##### ", "    ##", "##  ##", " #### "],
    [" #### ", "##  ##", "##    ", "##### ", "##  ##", "##  ##", " #### "],
    ["######", "    ##", "   ## ", "  ##  ", "  ##  ", "  ##  ", "  ##  "],
    [" #### ", "##  ##", "##  ##", " #### ", "##  ##", "##  ##", " #### "],
    [" #### ", "##  ##", "##  ##", " #####", "    ##", "##  ##", " #### "],
];
const BLOCK_COLON: [&str; DIGIT_HEIGHT] = ["  ", "##", "  ", "  ", "  ", "##", "  "];
const BLOCK_COLON_W: usize = 2;

// -- Fancy style (line drawing), 8 wide --
const FANCY_W: usize = 8;
const FANCY_DIGITS: [[&str; DIGIT_HEIGHT]; 10] = [
    [" /----\\ ", "|      |", "|      |", "|      |", "|      |", "|      |", " \\----/ "],
    ["    /|  ", "   / |  ", "     |  ", "     |  ", "     |  ", "     |  ", "  ------"],
    [" /----\\ ", "|      |", "      / ", "  ---/  ", " /      ", "|       ", " \\------"],
    [" /----\\ ", "      | ", "      | ", "  ---/  ", "      \\ ", "      | ", " \\----/ "],
    ["|      |", "|      |", "|      |", " \\-----|", "       |", "       |", "       |"],
    [" ------\\", "|       ", "|       ", " -----\\ ", "       |", "       |", " -----/ "],
    [" /----\\ ", "|      |", "|       ", "|-----\\ ", "|      |", "|      |", " \\----/ "],
    ["-------\\", "       |", "      / ", "     /  ", "    /   ", "   /    ", "  /     "],
    [" /----\\ ", "|      |", "|      |", " >----< ", "|      |", "|      |", " \\----/ "],
    [" /----\\ ", "|      |", "|      |", " \\-----|", "       |", "       |", " -----/ "],
];
const FANCY_COLON: [&str; DIGIT_HEIGHT] = ["  ", "oo", "  ", "  ", "  ", "oo", "  "];
const FANCY_COLON_W: usize = 2;

// -- Double-line style (box drawing), 10 wide, 9 tall --
const DOUBLE_HEIGHT: usize = 9;
const DOUBLE_W: usize = 10;
const DOUBLE_DIGITS: [[&str; DOUBLE_HEIGHT]; 10] = [
    // 0
    [
        " ╔══════╗ ",
        " ║      ║ ",
        " ║      ║ ",
        " ║      ║ ",
        " ║      ║ ",
        " ║      ║ ",
        " ║      ║ ",
        " ║      ║ ",
        " ╚══════╝ ",
    ],
    // 1
    [
        "     ╔╗   ",
        "    ╔╝║   ",
        "    ║ ║   ",
        "    ║ ║   ",
        "    ║ ║   ",
        "    ║ ║   ",
        "    ║ ║   ",
        "    ║ ║   ",
        "   ╚══╝   ",
    ],
    // 2
    [
        " ╔══════╗ ",
        " ║      ║ ",
        "        ║ ",
        "        ║ ",
        " ╔══════╝ ",
        " ║        ",
        " ║        ",
        " ║      ╗ ",
        " ╚══════╝ ",
    ],
    // 3
    [
        " ╔══════╗ ",
        " ║      ║ ",
        "        ║ ",
        "        ║ ",
        "  ══════╣ ",
        "        ║ ",
        "        ║ ",
        " ║      ║ ",
        " ╚══════╝ ",
    ],
    // 4
    [
        " ║      ║ ",
        " ║      ║ ",
        " ║      ║ ",
        " ║      ║ ",
        " ╚══════╣ ",
        "        ║ ",
        "        ║ ",
        "        ║ ",
        "        ║ ",
    ],
    // 5
    [
        " ╔══════╗ ",
        " ║      ║ ",
        " ║        ",
        " ║        ",
        " ╚══════╗ ",
        "        ║ ",
        "        ║ ",
        " ║      ║ ",
        " ╚══════╝ ",
    ],
    // 6
    [
        " ╔══════╗ ",
        " ║      ║ ",
        " ║        ",
        " ║        ",
        " ╠══════╗ ",
        " ║      ║ ",
        " ║      ║ ",
        " ║      ║ ",
        " ╚══════╝ ",
    ],
    // 7
    [
        " ╔══════╗ ",
        " ║      ║ ",
        "        ║ ",
        "       ╔╝ ",
        "       ║  ",
        "      ╔╝  ",
        "      ║   ",
        "      ║   ",
        "      ║   ",
    ],
    // 8
    [
        " ╔══════╗ ",
        " ║      ║ ",
        " ║      ║ ",
        " ║      ║ ",
        " ╠══════╣ ",
        " ║      ║ ",
        " ║      ║ ",
        " ║      ║ ",
        " ╚══════╝ ",
    ],
    // 9
    [
        " ╔══════╗ ",
        " ║      ║ ",
        " ║      ║ ",
        " ║      ║ ",
        " ╚══════╣ ",
        "        ║ ",
        "        ║ ",
        " ║      ║ ",
        " ╚══════╝ ",
    ],
];
const DOUBLE_COLON: [&str; DOUBLE_HEIGHT] = [
    "    ",
    " ╔╗ ",
    " ╚╝ ",
    "    ",
    "    ",
    "    ",
    " ╔╗ ",
    " ╚╝ ",
    "    ",
];
const DOUBLE_COLON_W: usize = 4;

const GAP: usize = 1;

#[derive(Clone, Copy)]
enum Style {
    Block,
    Fancy,
    Double,
}

fn draw_clock(time_str: &str, date_str: &str, style: Style) {
    ncurses::erase();

    let max_y = ncurses::getmaxy(ncurses::stdscr()) as usize;
    let max_x = ncurses::getmaxx(ncurses::stdscr()) as usize;

    let (digit_w, colon_w, height) = match style {
        Style::Block => (BLOCK_W, BLOCK_COLON_W, DIGIT_HEIGHT),
        Style::Fancy => (FANCY_W, FANCY_COLON_W, DIGIT_HEIGHT),
        Style::Double => (DOUBLE_W, DOUBLE_COLON_W, DOUBLE_HEIGHT),
    };
    let total_width = 6 * digit_w + 5 * GAP + 2 * (colon_w + 2 * GAP);

    if max_y < height + 4 || max_x < total_width {
        ncurses::mvaddstr(0, 0, "Terminal too small!");
        ncurses::refresh();
        return;
    }

    let start_y = (max_y - height) / 2 - 1;
    let start_x = (max_x - total_width) / 2;

    let chars: Vec<char> = time_str.chars().collect();
    let has_colors = ncurses::has_colors();

    for row in 0..height {
        let mut x = start_x;
        for (i, &ch) in chars.iter().enumerate() {
            if ch == ':' {
                x += GAP;
                let line = match style {
                    Style::Block => BLOCK_COLON[row],
                    Style::Fancy => FANCY_COLON[row],
                    Style::Double => DOUBLE_COLON[row],
                };
                if has_colors {
                    ncurses::attron(ncurses::COLOR_PAIR(2));
                }
                ncurses::mvaddstr(start_y as i32 + row as i32, x as i32, line);
                if has_colors {
                    ncurses::attroff(ncurses::COLOR_PAIR(2));
                }
                x += colon_w + GAP;
            } else if let Some(d) = ch.to_digit(10) {
                let line = match style {
                    Style::Block => BLOCK_DIGITS[d as usize][row],
                    Style::Fancy => FANCY_DIGITS[d as usize][row],
                    Style::Double => DOUBLE_DIGITS[d as usize][row],
                };
                let color = if i < 2 { 1 } else if i < 5 { 3 } else { 4 };
                if has_colors {
                    ncurses::attron(ncurses::COLOR_PAIR(color));
                }
                ncurses::mvaddstr(start_y as i32 + row as i32, x as i32, line);
                if has_colors {
                    ncurses::attroff(ncurses::COLOR_PAIR(color));
                }
                x += digit_w + GAP;
            }
        }
    }

    let date_x = (max_x.saturating_sub(date_str.len())) / 2;
    let date_y = start_y + height + 2;
    if has_colors {
        ncurses::attron(ncurses::COLOR_PAIR(5));
    }
    ncurses::mvaddstr(date_y as i32, date_x as i32, date_str);
    if has_colors {
        ncurses::attroff(ncurses::COLOR_PAIR(5));
    }

    let title = "-- fine_clock --";
    let title_x = (max_x - title.len()) / 2;
    if has_colors {
        ncurses::attron(ncurses::COLOR_PAIR(6));
    }
    ncurses::mvaddstr(start_y as i32 - 2, title_x as i32, title);
    if has_colors {
        ncurses::attroff(ncurses::COLOR_PAIR(6));
    }

    ncurses::refresh();
}

// ===================== Analog mode (graphical, kitty graphics) =====================

static RUNNING: AtomicBool = AtomicBool::new(true);

extern "C" fn on_signal(_sig: libc::c_int) {
    RUNNING.store(false, Ordering::SeqCst);
}

/// (cols, rows, x_pixels, y_pixels) of the terminal; pixels are 0 if unknown.
fn term_size() -> (u16, u16, u16, u16) {
    unsafe {
        let mut ws: libc::winsize = std::mem::zeroed();
        libc::ioctl(libc::STDOUT_FILENO, libc::TIOCGWINSZ, &mut ws);
        (ws.ws_col, ws.ws_row, ws.ws_xpixel, ws.ws_ypixel)
    }
}

fn set_raw() -> libc::termios {
    unsafe {
        let mut t: libc::termios = std::mem::zeroed();
        libc::tcgetattr(libc::STDIN_FILENO, &mut t);
        let orig = t;
        // Non-canonical, no echo, but keep ISIG so SIGINT reaches our handler.
        t.c_lflag &= !(libc::ICANON | libc::ECHO);
        t.c_cc[libc::VMIN] = 0;
        t.c_cc[libc::VTIME] = 0;
        libc::tcsetattr(libc::STDIN_FILENO, libc::TCSANOW, &t);
        orig
    }
}

fn restore_term(orig: &libc::termios) {
    unsafe {
        libc::tcsetattr(libc::STDIN_FILENO, libc::TCSANOW, orig);
    }
}

fn b64(data: &[u8]) -> String {
    const T: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity((data.len() + 2) / 3 * 4);
    for chunk in data.chunks(3) {
        let b0 = chunk[0];
        let b1 = *chunk.get(1).unwrap_or(&0);
        let b2 = *chunk.get(2).unwrap_or(&0);
        out.push(T[(b0 >> 2) as usize] as char);
        out.push(T[(((b0 & 0b11) << 4) | (b1 >> 4)) as usize] as char);
        out.push(if chunk.len() > 1 { T[(((b1 & 0b1111) << 2) | (b2 >> 6)) as usize] as char } else { '=' });
        out.push(if chunk.len() > 2 { T[(b2 & 0b111111) as usize] as char } else { '=' });
    }
    out
}

#[inline]
fn put(buf: &mut [u8], w: usize, h: usize, x: i32, y: i32, c: (u8, u8, u8, u8)) {
    if x < 0 || y < 0 || x as usize >= w || y as usize >= h {
        return;
    }
    let idx = (y as usize * w + x as usize) * 4;
    buf[idx] = c.0;
    buf[idx + 1] = c.1;
    buf[idx + 2] = c.2;
    buf[idx + 3] = c.3;
}

fn fill_disk(buf: &mut [u8], w: usize, h: usize, cx: f64, cy: f64, r: f64, c: (u8, u8, u8, u8)) {
    let r2 = r * r;
    let y0 = (cy - r).floor().max(0.0) as i32;
    let y1 = (cy + r).ceil().min(h as f64) as i32;
    let x0 = (cx - r).floor().max(0.0) as i32;
    let x1 = (cx + r).ceil().min(w as f64) as i32;
    for y in y0..y1 {
        for x in x0..x1 {
            let dx = x as f64 - cx;
            let dy = y as f64 - cy;
            if dx * dx + dy * dy <= r2 {
                put(buf, w, h, x, y, c);
            }
        }
    }
}

fn stroke_ring(buf: &mut [u8], w: usize, h: usize, cx: f64, cy: f64, r: f64, th: f64, c: (u8, u8, u8, u8)) {
    let outer = r + th / 2.0;
    let y0 = (cy - outer).floor().max(0.0) as i32;
    let y1 = (cy + outer).ceil().min(h as f64) as i32;
    let x0 = (cx - outer).floor().max(0.0) as i32;
    let x1 = (cx + outer).ceil().min(w as f64) as i32;
    for y in y0..y1 {
        for x in x0..x1 {
            let d = ((x as f64 - cx).powi(2) + (y as f64 - cy).powi(2)).sqrt();
            if (d - r).abs() <= th / 2.0 {
                put(buf, w, h, x, y, c);
            }
        }
    }
}

fn stroke_seg(buf: &mut [u8], w: usize, h: usize, x0: f64, y0: f64, x1: f64, y1: f64, th: f64, c: (u8, u8, u8, u8)) {
    let minx = (x0.min(x1) - th).floor().max(0.0) as i32;
    let maxx = (x0.max(x1) + th).ceil().min(w as f64) as i32;
    let miny = (y0.min(y1) - th).floor().max(0.0) as i32;
    let maxy = (y0.max(y1) + th).ceil().min(h as f64) as i32;
    let dx = x1 - x0;
    let dy = y1 - y0;
    let l2 = dx * dx + dy * dy;
    for y in miny..maxy {
        for x in minx..maxx {
            let px = x as f64;
            let py = y as f64;
            let t = if l2 == 0.0 { 0.0 } else { (((px - x0) * dx + (py - y0) * dy) / l2).clamp(0.0, 1.0) };
            let d = ((px - (x0 + t * dx)).powi(2) + (py - (y0 + t * dy)).powi(2)).sqrt();
            if d <= th / 2.0 {
                put(buf, w, h, x, y, c);
            }
        }
    }
}

/// Rasterize the clock face + hands to an RGBA buffer (transparent background).
fn render_analog(size: usize, hour12: f64, minute: f64, second: f64) -> Vec<u8> {
    let (w, h) = (size, size);
    let mut buf = vec![0u8; w * h * 4]; // transparent
    let cx = w as f64 / 2.0 - 0.5;
    let cy = h as f64 / 2.0 - 0.5;
    let r = size as f64 / 2.0 - 3.0;

    fill_disk(&mut buf, w, h, cx, cy, r, (24, 24, 34, 255)); // face
    stroke_ring(&mut buf, w, h, cx, cy, r, 3.0, (210, 210, 225, 255)); // bezel

    // Tick marks: long/bright every 5, short/dim otherwise.
    for i in 0..60 {
        let a = i as f64 / 60.0 * 2.0 * PI;
        let major = i % 5 == 0;
        let (inner, th, col) = if major {
            (r - 18.0, 3.0, (235, 235, 245, 255))
        } else {
            (r - 9.0, 1.5, (110, 110, 130, 255))
        };
        let outer = r - 5.0;
        stroke_seg(&mut buf, w, h, cx + inner * a.sin(), cy - inner * a.cos(), cx + outer * a.sin(), cy - outer * a.cos(), th, col);
    }

    // Hands.
    let ha = (hour12 + minute / 60.0) / 12.0 * 2.0 * PI;
    let ma = (minute + second / 60.0) / 60.0 * 2.0 * PI;
    let sa = second / 60.0 * 2.0 * PI;
    stroke_seg(&mut buf, w, h, cx, cy, cx + r * 0.50 * ha.sin(), cy - r * 0.50 * ha.cos(), 7.0, (0, 200, 255, 255)); // hour: cyan
    stroke_seg(&mut buf, w, h, cx, cy, cx + r * 0.72 * ma.sin(), cy - r * 0.72 * ma.cos(), 4.5, (0, 220, 90, 255)); // minute: green
    stroke_seg(&mut buf, w, h, cx, cy, cx + r * 0.82 * sa.sin(), cy - r * 0.82 * sa.cos(), 2.0, (255, 70, 70, 255)); // second: red
    fill_disk(&mut buf, w, h, cx, cy, 5.0, (255, 255, 255, 255)); // hub

    buf
}

/// True if (px,py) is inside the convex polygon `pts` (consistent winding).
fn point_in_convex(px: f64, py: f64, pts: &[(f64, f64)]) -> bool {
    let n = pts.len();
    let mut sign: i32 = 0;
    for i in 0..n {
        let (ax, ay) = pts[i];
        let (bx, by) = pts[(i + 1) % n];
        let cross = (bx - ax) * (py - ay) - (by - ay) * (px - ax);
        let s = if cross > 0.0 { 1 } else if cross < 0.0 { -1 } else { 0 };
        if s != 0 {
            if sign == 0 {
                sign = s;
            } else if s != sign {
                return false;
            }
        }
    }
    true
}

fn fill_poly(buf: &mut [u8], w: usize, h: usize, pts: &[(f64, f64)], c: (u8, u8, u8, u8)) {
    let mut minx = f64::MAX;
    let mut miny = f64::MAX;
    let mut maxx = f64::MIN;
    let mut maxy = f64::MIN;
    for &(x, y) in pts {
        minx = minx.min(x);
        miny = miny.min(y);
        maxx = maxx.max(x);
        maxy = maxy.max(y);
    }
    let x0 = minx.floor().max(0.0) as i32;
    let x1 = (maxx.ceil()).min(w as f64) as i32;
    let y0 = miny.floor().max(0.0) as i32;
    let y1 = (maxy.ceil()).min(h as f64) as i32;
    let mut y = y0;
    while y < y1 {
        let mut x = x0;
        while x < x1 {
            if point_in_convex(x as f64 + 0.5, y as f64 + 0.5, pts) {
                put(buf, w, h, x, y, c);
            }
            x += 1;
        }
        y += 1;
    }
}

/// A tapered "lozenge" clock hand: a diamond from a short tail, through a wide
/// mid-point, to a pointed tip.
fn hand_poly(cx: f64, cy: f64, angle: f64, len: f64, hw: f64, tail: f64) -> [(f64, f64); 4] {
    let (dx, dy) = (angle.sin(), -angle.cos());
    let (px, py) = (angle.cos(), angle.sin());
    let base = (cx - dx * tail, cy - dy * tail);
    let tip = (cx + dx * len, cy + dy * len);
    let mx = cx + dx * len * 0.32;
    let my = cy + dy * len * 0.32;
    [base, (mx + px * hw, my + py * hw), tip, (mx - px * hw, my - py * hw)]
}

/// Draw one Roman-numeral character (I, V, X) upright in the box whose top-left
/// is (tx,ty) with the given width/height, using straight strokes.
fn roman_char(buf: &mut [u8], w: usize, h: usize, ch: u8, tx: f64, ty: f64, cw: f64, hg: f64, th: f64, c: (u8, u8, u8, u8)) {
    match ch {
        b'I' => stroke_seg(buf, w, h, tx + cw / 2.0, ty, tx + cw / 2.0, ty + hg, th, c),
        b'V' => {
            stroke_seg(buf, w, h, tx, ty, tx + cw / 2.0, ty + hg, th, c);
            stroke_seg(buf, w, h, tx + cw, ty, tx + cw / 2.0, ty + hg, th, c);
        }
        b'X' => {
            stroke_seg(buf, w, h, tx, ty, tx + cw, ty + hg, th, c);
            stroke_seg(buf, w, h, tx + cw, ty, tx, ty + hg, th, c);
        }
        _ => {}
    }
}

/// Draw a Roman numeral centered at (ccx,ccy).
fn draw_roman(buf: &mut [u8], w: usize, h: usize, s: &[u8], ccx: f64, ccy: f64, hg: f64, c: (u8, u8, u8, u8)) {
    let cw = hg * 0.5;
    let gap = hg * 0.14;
    let total = s.len() as f64 * cw + (s.len() as f64 - 1.0) * gap;
    let th = (hg * 0.14).max(1.5);
    let mut tx = ccx - total / 2.0;
    let ty = ccy - hg / 2.0;
    for &ch in s {
        roman_char(buf, w, h, ch, tx, ty, cw, hg, th, c);
        tx += cw + gap;
    }
}

const BRONZE: (u8, u8, u8, u8) = (94, 66, 28, 255);
const BRASS: (u8, u8, u8, u8) = (198, 152, 74, 255);
const CREAM: (u8, u8, u8, u8) = (238, 229, 203, 255);
const INK: (u8, u8, u8, u8) = (34, 28, 22, 255);
const VINTAGE_RED: (u8, u8, u8, u8) = (168, 44, 32, 255);

/// Draw the retro clock face (bezel, dial, markers, Roman numerals, hands, hub)
/// centered at (cx,cy) with radius r, into an arbitrary-size buffer.
fn draw_retro_face(buf: &mut [u8], w: usize, h: usize, cx: f64, cy: f64, r: f64, hour12: f64, minute: f64, second: f64) {
    let d = r * 2.0; // scale base

    // Brass double bezel over a cream dial.
    fill_disk(buf, w, h, cx, cy, r, BRONZE);
    fill_disk(buf, w, h, cx, cy, r - 2.0, BRASS);
    fill_disk(buf, w, h, cx, cy, r - d * 0.035, CREAM);
    // Aged patina: a couple of faint rings.
    stroke_ring(buf, w, h, cx, cy, r - d * 0.05, 1.0, (150, 130, 95, 90));
    stroke_ring(buf, w, h, cx, cy, r * 0.30, 1.0, (150, 130, 95, 70));

    // Minute track.
    let rim = r - d * 0.05;
    for i in 0..60 {
        // Skip the eight non-cardinal hour positions (they carry bold batons),
        // but keep the four cardinals (12/3/6/9) so the rim isn't gapped under
        // the Roman numerals — drawn thicker, same length, as an accent.
        if i % 5 == 0 && i % 15 != 0 {
            continue;
        }
        let a = i as f64 / 60.0 * 2.0 * PI;
        let th = if i % 15 == 0 { (d * 0.014).max(1.5) } else { (d * 0.006).max(1.0) };
        stroke_seg(buf, w, h, cx + (rim - d * 0.02) * a.sin(), cy - (rim - d * 0.02) * a.cos(), cx + rim * a.sin(), cy - rim * a.cos(), th, INK);
    }

    // Hours: Roman numerals at the cardinals, bold batons elsewhere.
    let numerals: [&[u8]; 12] = [b"XII", b"I", b"II", b"III", b"IIII", b"V", b"VI", b"VII", b"VIII", b"IX", b"X", b"XI"];
    for i in 0..12 {
        let a = i as f64 / 12.0 * 2.0 * PI;
        if i % 3 == 0 {
            let rn = r * 0.66;
            draw_roman(buf, w, h, numerals[i], cx + rn * a.sin(), cy - rn * a.cos(), d * 0.085, INK);
        } else {
            let outer = rim - d * 0.02;
            let inner = outer - d * 0.06;
            stroke_seg(buf, w, h, cx + inner * a.sin(), cy - inner * a.cos(), cx + outer * a.sin(), cy - outer * a.cos(), d * 0.022, INK);
        }
    }

    // Hands.
    let ha = (hour12 + minute / 60.0) / 12.0 * 2.0 * PI;
    let ma = (minute + second / 60.0) / 60.0 * 2.0 * PI;
    let sa = second / 60.0 * 2.0 * PI;
    fill_poly(buf, w, h, &hand_poly(cx, cy, ha, r * 0.52, d * 0.028, d * 0.06), INK);
    fill_poly(buf, w, h, &hand_poly(cx, cy, ma, r * 0.76, d * 0.020, d * 0.06), INK);
    let (sdx, sdy) = (sa.sin(), -sa.cos());
    stroke_seg(buf, w, h, cx - sdx * r * 0.16, cy - sdy * r * 0.16, cx + sdx * r * 0.84, cy + sdy * r * 0.84, (d * 0.008).max(1.5), VINTAGE_RED);
    fill_disk(buf, w, h, cx - sdx * r * 0.16, cy - sdy * r * 0.16, d * 0.02, VINTAGE_RED);

    // Hub.
    fill_disk(buf, w, h, cx, cy, d * 0.028, INK);
    fill_disk(buf, w, h, cx, cy, d * 0.012, BRASS);
}

/// Retro variant: warm ivory dial, brass double-bezel, black baton markers,
/// Roman numerals at the cardinals, tapered hands, and a red second hand.
fn render_analog_retro(size: usize, hour12: f64, minute: f64, second: f64) -> Vec<u8> {
    let (w, h) = (size, size);
    let mut buf = vec![0u8; w * h * 4];
    draw_retro_face(&mut buf, w, h, w as f64 / 2.0 - 0.5, h as f64 / 2.0 - 0.5, size as f64 / 2.0 - 2.0, hour12, minute, second);
    buf
}

fn fill_rect(buf: &mut [u8], w: usize, h: usize, x: f64, y: f64, rw: f64, rh: f64, c: (u8, u8, u8, u8)) {
    let x0 = x.floor().max(0.0) as i32;
    let y0 = y.floor().max(0.0) as i32;
    let x1 = (x + rw).ceil().min(w as f64) as i32;
    let y1 = (y + rh).ceil().min(h as f64) as i32;
    let mut yy = y0;
    while yy < y1 {
        let mut xx = x0;
        while xx < x1 {
            put(buf, w, h, xx, yy, c);
            xx += 1;
        }
        yy += 1;
    }
}

const WOOD: (u8, u8, u8, u8) = (99, 62, 31, 255);
const WOOD_HI: (u8, u8, u8, u8) = (142, 94, 54, 255);
const WOOD_DK: (u8, u8, u8, u8) = (64, 39, 19, 255);
const CAVITY: (u8, u8, u8, u8) = (26, 18, 12, 255);

/// A cased grandfather-style clock: a wooden hood framing the retro dial, a
/// narrower trunk below housing the swinging pendulum, and a plinth base.
/// `phase` is the pendulum angle (radians from vertical); `second` should be a
/// whole second so the second hand ticks.
fn render_pendulum_clock(w: usize, h: usize, hour12: f64, minute: f64, second: f64, phase: f64) -> Vec<u8> {
    let mut buf = vec![0u8; w * h * 4];
    let wf = w as f64;
    let hf = h as f64;

    let hood_h = wf * 0.96;
    let base_h = wf * 0.14;
    let trunk_top = hood_h * 0.86; // starts behind the hood
    let trunk_bot = hf - base_h;
    let trunk_w = wf * 0.56;
    let tx0 = (wf - trunk_w) / 2.0;

    // Trunk: wood case → inner frame → dark cavity.
    fill_rect(&mut buf, w, h, tx0, trunk_top, trunk_w, trunk_bot - trunk_top, WOOD);
    let i1 = wf * 0.045;
    fill_rect(&mut buf, w, h, tx0 + i1, trunk_top + i1, trunk_w - 2.0 * i1, (trunk_bot - trunk_top) - 2.0 * i1, WOOD_DK);
    let i2 = i1 + wf * 0.018;
    fill_rect(&mut buf, w, h, tx0 + i2, trunk_top + i2, trunk_w - 2.0 * i2, (trunk_bot - trunk_top) - 2.0 * i2, CAVITY);

    // Pendulum, swinging inside the cavity (pivot hidden behind the hood).
    let pivot_x = wf / 2.0;
    let pivot_y = trunk_top + wf * 0.03;
    let rod_len = (trunk_bot - pivot_y) - wf * 0.20;
    let bx = pivot_x + rod_len * phase.sin();
    let by = pivot_y + rod_len * phase.cos();
    stroke_seg(&mut buf, w, h, pivot_x, pivot_y, bx, by, (wf * 0.011).max(2.0), BRASS);
    let bob = wf * 0.072;
    fill_disk(&mut buf, w, h, bx, by, bob, BRONZE);
    fill_disk(&mut buf, w, h, bx, by, bob - wf * 0.011, BRASS);
    fill_disk(&mut buf, w, h, bx - bob * 0.3, by - bob * 0.3, bob * 0.28, (232, 200, 140, 255));

    // Base plinth (wider), with a molding strip and small feet.
    let base_w = wf * 0.88;
    let bx0 = (wf - base_w) / 2.0;
    fill_rect(&mut buf, w, h, bx0, trunk_bot, base_w, base_h, WOOD);
    fill_rect(&mut buf, w, h, bx0, trunk_bot, base_w, wf * 0.02, WOOD_HI);
    fill_rect(&mut buf, w, h, bx0 + wf * 0.02, hf - wf * 0.03, wf * 0.12, wf * 0.03, WOOD_DK);
    fill_rect(&mut buf, w, h, bx0 + base_w - wf * 0.14, hf - wf * 0.03, wf * 0.12, wf * 0.03, WOOD_DK);

    // Hood: a hexagonal wooden housing framing the dial, covering the trunk top
    // and the pendulum pivot. Flat top and bottom edges, points at left/right.
    let dcx = wf / 2.0 - 0.5;
    let dcy = hood_h * 0.5;
    let hex = |r: f64| -> [(f64, f64); 6] {
        let mut v = [(0.0, 0.0); 6];
        let mut i = 0;
        while i < 6 {
            let a = (30.0 + 60.0 * i as f64) * PI / 180.0;
            v[i] = (dcx + r * a.sin(), dcy - r * a.cos());
            i += 1;
        }
        v
    };
    let r_hex = wf * 0.49; // center-to-vertex; overall width ~0.98w
    fill_poly(&mut buf, w, h, &hex(r_hex), WOOD_DK);
    fill_poly(&mut buf, w, h, &hex(r_hex - wf * 0.015), WOOD_HI);
    fill_poly(&mut buf, w, h, &hex(r_hex - wf * 0.04), WOOD);

    // Round dial nested inside the hexagonal frame.
    let r_face = r_hex * 0.866 - wf * 0.06;
    fill_disk(&mut buf, w, h, dcx, dcy, r_face + wf * 0.02, WOOD_DK);
    draw_retro_face(&mut buf, w, h, dcx, dcy, r_face, hour12, minute, second);
    buf
}

/// Transmit an RGBA image via the kitty graphics protocol and display it at the
/// cursor's current position (replacing any prior frame of image id 1).
/// Upload an RGBA image to the terminal WITHOUT displaying it (a=t). This is
/// the "prerender into the buffer" step — the heavy transmission happens with
/// nothing changing on screen. `q=2` suppresses OK/error replies (which would
/// otherwise land on our stdin). Display it later with `kitty_put`.
fn kitty_upload(out: &mut impl Write, rgba: &[u8], w: usize, h: usize, id: u32) {
    let payload = b64(rgba);
    let bytes = payload.as_bytes();
    let chunks: Vec<&[u8]> = bytes.chunks(4096).collect();
    for (i, ch) in chunks.iter().enumerate() {
        let m = if i == chunks.len() - 1 { 0 } else { 1 };
        if i == 0 {
            let _ = write!(out, "\x1b_Gi={},a=t,f=32,t=d,q=2,s={},v={},m={};", id, w, h, m);
        } else {
            let _ = write!(out, "\x1b_Gm={};", m);
        }
        let _ = out.write_all(ch);
        let _ = out.write_all(b"\x1b\\");
    }
}

/// Display an already-uploaded image at the cursor (a=p). Instant — the pixels
/// are already in the terminal, so this is the flicker-free swap.
fn kitty_put(out: &mut impl Write, id: u32) {
    let _ = write!(out, "\x1b_Gi={},a=p,q=2\x1b\\", id);
}

/// Delete an image and its placements by id.
fn kitty_delete(out: &mut impl Write, id: u32) {
    let _ = write!(out, "\x1b_Ga=d,d=i,i={},q=2\x1b\\", id);
}

fn run_analog(retro: bool) {
    unsafe {
        libc::signal(libc::SIGINT, on_signal as libc::sighandler_t);
        libc::signal(libc::SIGTERM, on_signal as libc::sighandler_t);
    }
    let orig = set_raw();
    let mut out = std::io::stdout();
    let _ = out.write_all(b"\x1b[?1049h\x1b[?25l"); // alt screen, hide cursor
    let _ = out.flush();

    let mut last_second = u32::MAX;
    let mut in_buf = [0u8; 16];
    let mut stdin = std::io::stdin();
    // Double buffer: alternate between two image ids so we can upload the next
    // frame off-screen, then swap it in. 0 means "nothing shown yet".
    let mut cur_id: u32 = 0;

    while RUNNING.load(Ordering::SeqCst) {
        // Quit on q / Q / Ctrl-C. NOT bare ESC: terminal replies (e.g. the
        // kitty-graphics "OK" acknowledgement, cursor-position reports) begin
        // with ESC and would otherwise look like a quit keypress.
        if let Ok(n) = stdin.read(&mut in_buf) {
            if in_buf[..n].iter().any(|&b| b == b'q' || b == b'Q' || b == 0x03) {
                break;
            }
        }

        let now = Local::now();
        if now.second() != last_second {
            last_second = now.second();

            let (cols, rows, xpix, ypix) = term_size();
            let avail_w = if xpix > 0 { xpix as usize } else { cols as usize * 10 };
            let avail_h = if ypix > 0 { ypix as usize } else { rows as usize * 20 };
            let cell_w = (avail_w / cols.max(1) as usize).max(1);
            let cell_h = (avail_h / rows.max(1) as usize).max(1);
            // Leave a couple of rows for the date caption.
            let size = ((avail_w.min(avail_h.saturating_sub(2 * cell_h)) as f64) * 0.9)
                .min(700.0) as usize;
            let size = size.max(64);

            let img_cols = size.div_ceil(cell_w);
            let img_rows = size.div_ceil(cell_h);
            let col0 = ((cols as usize).saturating_sub(img_cols)) / 2 + 1;
            let row0 = ((rows as usize).saturating_sub(img_rows + 2)) / 2 + 1;

            let hour12 = (now.hour() % 12) as f64;
            let rgba = if retro {
                render_analog_retro(size, hour12, now.minute() as f64, now.second() as f64)
            } else {
                render_analog(size, hour12, now.minute() as f64, now.second() as f64)
            };

            // Double-buffered swap: upload the next frame off-screen (a=t),
            // then position and display it (a=p) and delete the old one. The
            // slow transmission happens with nothing changing on screen, so the
            // visible update is instant — no flicker.
            let next_id = if cur_id == 1 { 2 } else { 1 };
            kitty_upload(&mut out, &rgba, size, size, next_id);
            let _ = write!(out, "\x1b[{};{}H", row0, col0);
            kitty_put(&mut out, next_id);
            if cur_id != 0 {
                kitty_delete(&mut out, cur_id);
            }
            cur_id = next_id;

            // Date caption, centered below the clock (clear the line first so a
            // shorter day/month name doesn't leave stragglers).
            let date = now.format("%A, %B %d, %Y").to_string();
            let date_col = ((cols as usize).saturating_sub(date.chars().count())) / 2 + 1;
            let date_row = row0 + img_rows + 1;
            let _ = write!(out, "\x1b[{};1H\x1b[2K\x1b[{};{}H\x1b[35m{}\x1b[0m", date_row, date_row, date_col, date);
            let _ = out.flush();
        }

        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    // Teardown: delete images, show cursor, leave alt screen, restore tty.
    let _ = out.write_all(b"\x1b_Ga=d,d=A\x1b\\\x1b[?25h\x1b[?1049l");
    let _ = out.flush();
    restore_term(&orig);
}

/// High-frame-rate retro clock with a swinging pendulum. Redraws every frame
/// (not once per second) with a smoothly sweeping second hand and pendulum,
/// double-buffered for flicker-free updates. Shows a live FPS readout.
/// Image aspect ratio (height / width) of the cased pendulum clock.
const PENDULUM_ASPECT: f64 = 2.15;

/// Probe the terminal and compute the largest clock image that fills it (keeping
/// the aspect ratio), plus where to place it so the assembly is centered.
/// Returns (w, h, img_cols, img_rows, col0, row0, cols).
fn pendulum_layout() -> (usize, usize, usize, usize, usize, usize, u16) {
    let (cols, rows, xpix, ypix) = term_size();
    let avail_w = if xpix > 0 { xpix as usize } else { cols as usize * 10 };
    let avail_h = if ypix > 0 { ypix as usize } else { rows as usize * 20 };
    let cell_w = (avail_w / cols.max(1) as usize).max(1);
    let cell_h = (avail_h / rows.max(1) as usize).max(1);
    // Reserve two rows for the caption, then take the biggest image that fits
    // both dimensions at our aspect ratio — i.e. fill the terminal.
    let usable_h = avail_h.saturating_sub(2 * cell_h);
    let w = avail_w.min((usable_h as f64 / PENDULUM_ASPECT) as usize).max(80);
    let h = (w as f64 * PENDULUM_ASPECT) as usize;
    let img_cols = w.div_ceil(cell_w);
    let img_rows = h.div_ceil(cell_h);
    let col0 = ((cols as usize).saturating_sub(img_cols)) / 2 + 1;
    // Center the image (+caption) vertically so its center of gravity sits at
    // the middle of the terminal.
    let row0 = ((rows as usize).saturating_sub(img_rows + 1)) / 2 + 1;
    (w, h, img_cols, img_rows, col0, row0.max(1), cols)
}

fn run_pendulum() {
    unsafe {
        libc::signal(libc::SIGINT, on_signal as libc::sighandler_t);
        libc::signal(libc::SIGTERM, on_signal as libc::sighandler_t);
    }
    let orig = set_raw();
    let mut out = std::io::stdout();
    let _ = out.write_all(b"\x1b[?1049h\x1b[?25l");
    let _ = out.flush();

    let (mut w, mut h, _, mut img_rows, mut col0, mut row0, mut cols) = pendulum_layout();

    let start = std::time::Instant::now();
    let mut last = std::time::Instant::now();
    let mut last_probe = std::time::Instant::now();
    let mut fps: f64 = 0.0;
    let mut cur_id: u32 = 0;
    let mut in_buf = [0u8; 16];
    let mut stdin = std::io::stdin();

    // Cap at ~60 fps so we don't spin pointlessly on a fast terminal.
    let frame_target = std::time::Duration::from_micros(16_666);

    while RUNNING.load(Ordering::SeqCst) {
        let frame_start = std::time::Instant::now();
        if let Ok(n) = stdin.read(&mut in_buf) {
            if in_buf[..n].iter().any(|&b| b == b'q' || b == b'Q' || b == 0x03) {
                break;
            }
        }

        // Re-probe the terminal once a second so the clock tracks resizes.
        if last_probe.elapsed().as_secs_f64() >= 1.0 {
            last_probe = std::time::Instant::now();
            let (nw, nh, _, nir, ncol0, nrow0, ncols) = pendulum_layout();
            if (nw, nh, ncol0, nrow0, ncols) != (w, h, col0, row0, cols) {
                if cur_id != 0 {
                    kitty_delete(&mut out, cur_id);
                    cur_id = 0;
                }
                let _ = out.write_all(b"\x1b[2J");
                w = nw; h = nh; img_rows = nir;
                col0 = ncol0; row0 = nrow0; cols = ncols;
            }
        }

        let t = start.elapsed().as_secs_f64();
        let now = Local::now();
        // Pendulum swings smoothly (full swing every 2s, ~9 deg amplitude); the
        // second hand ticks once per second like a normal clock.
        let theta = 0.15 * (2.0 * PI * t / 2.0).cos();
        let rgba = render_pendulum_clock(w, h, (now.hour() % 12) as f64, now.minute() as f64, now.second() as f64, theta);

        let next_id = if cur_id == 1 { 2 } else { 1 };
        kitty_upload(&mut out, &rgba, w, h, next_id);
        let _ = write!(out, "\x1b[{};{}H", row0, col0);
        kitty_put(&mut out, next_id);
        if cur_id != 0 {
            kitty_delete(&mut out, cur_id);
        }
        cur_id = next_id;

        let caption = format!("{}    {:.0} fps", now.format("%H:%M:%S"), fps);
        let cap_col = ((cols as usize).saturating_sub(caption.chars().count())) / 2 + 1;
        let cap_row = row0 + img_rows + 1;
        let _ = write!(out, "\x1b[{};1H\x1b[2K\x1b[{};{}H\x1b[38;2;198;152;74m{}\x1b[0m", cap_row, cap_row, cap_col, caption);
        let _ = out.flush();

        let el = frame_start.elapsed();
        if el < frame_target {
            std::thread::sleep(frame_target - el);
        }

        let dt = last.elapsed().as_secs_f64();
        last = std::time::Instant::now();
        if dt > 0.0 {
            let inst = 1.0 / dt;
            fps = if fps == 0.0 { inst } else { 0.9 * fps + 0.1 * inst };
        }
    }

    let _ = out.write_all(b"\x1b_Ga=d,d=A\x1b\\\x1b[?25h\x1b[?1049l");
    let _ = out.flush();
    restore_term(&orig);
}

fn main() {
    if env::args().any(|a| a == "--retro-with-pendulum" || a == "-P") {
        run_pendulum();
        return;
    }
    if env::args().any(|a| a == "--analog-retro" || a == "-A") {
        run_analog(true);
        return;
    }
    if env::args().any(|a| a == "--analog" || a == "-a") {
        run_analog(false);
        return;
    }

    let style = if env::args().any(|a| a == "--double" || a == "-d") {
        Style::Double
    } else if env::args().any(|a| a == "--fancy" || a == "-f") {
        Style::Fancy
    } else {
        Style::Block
    };

    // Enable UTF-8 in ncurses
    let empty = CString::new("").unwrap();
    unsafe {
        libc::setlocale(libc::LC_ALL, empty.as_ptr());
    }

    ncurses::initscr();
    ncurses::noecho();
    ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    ncurses::timeout(200);
    ncurses::keypad(ncurses::stdscr(), true);

    if ncurses::has_colors() {
        ncurses::start_color();
        ncurses::use_default_colors();
        ncurses::init_pair(1, ncurses::COLOR_CYAN, -1);
        ncurses::init_pair(2, ncurses::COLOR_WHITE, -1);
        ncurses::init_pair(3, ncurses::COLOR_GREEN, -1);
        ncurses::init_pair(4, ncurses::COLOR_YELLOW, -1);
        ncurses::init_pair(5, ncurses::COLOR_MAGENTA, -1);
        ncurses::init_pair(6, ncurses::COLOR_BLUE, -1);
    }

    loop {
        let now = Local::now();
        let time_str = now.format("%H:%M:%S").to_string();
        let date_str = now.format("%A, %B %d, %Y").to_string();

        draw_clock(&time_str, &date_str, style);

        let ch = ncurses::getch();
        if ch == b'q' as i32 || ch == b'Q' as i32 {
            break;
        }
    }

    ncurses::endwin();
}
