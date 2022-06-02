use imagine::*;
use termion::input::TermRead;

use std::io::{stdin, Write};

const WIDTH: usize = 8;
const HEIGHT: usize = 8;

/// Returns an alternate raw screen.
fn get_screen() -> impl Write {
    use std::io::stdout;
    use termion::raw::IntoRawMode;
    use termion::screen::*;

    AlternateScreen::from(stdout().into_raw_mode().unwrap())
}

/// Draws the given canvas.
fn draw_canvas(screen: &mut impl Write, canvas: &Canvas, cursor: &Cursor) {
    let cursor_linear = cursor.x + cursor.y * canvas.height;
    write!(screen, "{}", termion::cursor::Goto(1, 3)).unwrap();

    for i in 0..canvas.width * canvas.height {
        use termion::color::*;

        if i % canvas.width == 0 && i != 0 {
            write!(screen, "\r\n").unwrap();
        }

        let pixel_color = canvas.get_linear(i);

        let pixel_content = if i == cursor_linear {
            if is_dark(pixel_color) {
                format!("{}<>{}", Fg(White), Fg(Reset))
            } else {
                format!("{}<>{}", Fg(Black), Fg(Reset))
            }
        } else {
            String::from("  ")
        };

        write!(
            screen,
            "{}{}{}",
            Bg(Rgb(pixel_color.r, pixel_color.g, pixel_color.b)),
            pixel_content,
            Bg(Reset)
        )
        .unwrap();
    }
}

fn main() {
    let mut canvas = Canvas::new(WIDTH, HEIGHT);
    let mut cursor = Cursor::new(WIDTH, HEIGHT);
    let mut palette = Palette::new(vec![
        rgb(59, 59, 59),
        rgb(239, 239, 239),
        rgb(68, 102, 221),
        rgb(224, 50, 148),
        rgb(241, 154, 62),
    ]);

    let mut screen = get_screen();
    let stdin = stdin();

    write!(
        screen,
        "{}{}Welcome to Imagine. Press 'q' to exit.",
        termion::cursor::Hide,
        termion::cursor::Goto(1, 1),
    )
    .unwrap();
    draw_canvas(&mut screen, &canvas, &cursor);
    write!(screen, "\r\n\n{}", palette).unwrap();

    screen.flush().unwrap();

    // Loop
    for c in stdin.keys() {
        use termion::event::Key;

        match c.unwrap() {
            Key::Up => cursor.up(1),
            Key::Down => cursor.down(1),
            Key::Left => cursor.left(1),
            Key::Right => cursor.right(1),

            Key::Char('\n') => canvas.paint(cursor.as_tuple(), palette.get()),

            Key::Char('1') => palette.select(0).unwrap(),
            Key::Char('2') => palette.select(1).unwrap(),
            Key::Char('3') => palette.select(2).unwrap(),
            Key::Char('4') => palette.select(3).unwrap(),
            Key::Char('5') => palette.select(4).unwrap(),

            Key::Char('q') | Key::Esc | Key::Ctrl('d') => break,
            Key::Ctrl('c') => {
                write!(screen, "{}", termion::cursor::Show).unwrap();
                screen.flush().unwrap();
                return;
            }
            _ => {}
        }

        draw_canvas(&mut screen, &canvas, &cursor);
        write!(screen, "\r\n\n{}", palette).unwrap();
        screen.flush().unwrap();
    }

    write!(screen, "{}", termion::cursor::Show).unwrap();
    screen.flush().unwrap();

    lodepng::encode_file(
        "out.png",
        canvas.get_buffer(),
        WIDTH,
        HEIGHT,
        lodepng::ColorType::RGB,
        8,
    )
    .unwrap();
}
