use reef_backend_d2d::{
    dpi::ensure_process_dpi_awareness,
    painter::Direct2DPainter,
    window::{NativeWindow, WindowConfig, WindowStyle},
};
use reef_core::{
    color::Color,
    geometry::{Point, Rect},
};
use reef_render::primitive::{FontWeight, TextAlignment, VisualPrimitive};

fn main() {
    ensure_process_dpi_awareness();

    let width = 320.0;
    let height = 48.0;
    let x = 400.0;
    let y = 100.0;

    let config = WindowConfig::new(Rect { x, y, width, height }).style(WindowStyle::LayeredTopmost);

    let window = match NativeWindow::create(&config) {
        Ok(w) => w,
        Err(e) => {
            eprintln!("Failed to create window: {e}");
            return;
        }
    };

    let mut painter = Direct2DPainter::new();
    painter.set_window(window.hwnd());

    let primitives = vec![
        VisualPrimitive::RoundRect {
            frame: Rect {
                x: 0.0,
                y: 0.0,
                width,
                height,
            },
            radius: 24.0,
            color: Color::rgb(18, 18, 22),
            alpha: 1.0,
        },
        VisualPrimitive::RoundRect {
            frame: Rect {
                x: 0.5,
                y: 0.5,
                width: width - 1.0,
                height: height - 1.0,
            },
            radius: 23.5,
            color: Color::rgb(44, 44, 50),
            alpha: 0.6,
        },
        VisualPrimitive::Text {
            origin: Point { x: 12.0, y: 10.0 },
            max_width: width - 24.0,
            text: "Hello, Reef!".to_string(),
            color: Color::rgb(230, 235, 245),
            size: 16,
            weight: FontWeight::Semibold,
            alignment: TextAlignment::Center,
            alpha: 1.0,
        },
    ];

    let screen_rect = Rect { x, y, width, height };

    if let Err(e) = painter.render_to_window(&primitives, screen_rect) {
        eprintln!("Render error: {e}");
    }

    window.show();

    println!("Hello Reef! Window displayed at ({x}, {y}), size {width}x{height}");
    println!("Close the window or press Ctrl+C to exit.");

    while window.poll_message() {}
}
