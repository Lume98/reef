use reef_app::widget_host::WidgetHost;
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
use reef_widgets::{
    card::{BodyLine, Card, CardStyle},
    container::Container,
    label::Label,
};

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

    let mut painter: Direct2DPainter = Direct2DPainter::new();
    painter.set_window(window.hwnd());

    // --- Old-style: manual primitives ---
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

    // --- New-style: widget system demo ---
    println!();
    println!("=== Widget System Demo ===");

    // Container with a Label child
    let container = Container::new(Color::rgb(18, 18, 22))
        .radius(24.0)
        .border(Color::rgb(44, 44, 50), 1.0)
        .padding(10.0)
        .child(Box::new(Label::new("Hello from Widget!").color(Color::rgb(230, 235, 245)).font_size(16)));

    let mut host = WidgetHost::new();
    host.set_size(reef_core::geometry::Size { width: 320.0, height: 48.0 });
    host.set_root(Box::new(container));
    let plan = host.render();
    println!("Container+Label produced {} primitives", plan.primitives.len());

    // Card widget demo
    let card = Card::new(CardStyle::PendingApproval)
        .title("Allow command?")
        .status_badge("Waiting")
        .body_line(BodyLine { prefix: Some("$ ".into()), text: "rm -rf /tmp".into() })
        .action_hint("Allow / Deny")
        .height(120.0);

    let mut host2 = WidgetHost::new();
    host2.set_size(reef_core::geometry::Size { width: 300.0, height: 120.0 });
    host2.set_root(Box::new(card));
    let plan2 = host2.render();
    println!("Card produced {} primitives", plan2.primitives.len());

    println!("Close the window or press Ctrl+C to exit.");
    while window.poll_message() {}
}
