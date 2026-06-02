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

    // Card widget demo: showcase all card styles and features
    println!();
    println!("=== Card Widget Showcase ===");

    // 1. PendingApproval — command approval card
    demo_card(
        "PendingApproval",
        Card::new(CardStyle::PendingApproval)
            .title("Allow command?")
            .status_badge("Waiting")
            .source_badge("terminal")
            .body_line(BodyLine { prefix: Some("$ ".into()), text: "rm -rf /tmp/cache".into() })
            .action_hint("Allow / Deny")
            .height(120.0),
        300.0,
        120.0,
    );

    // 2. PendingQuestion — question card with tool pill
    demo_card(
        "PendingQuestion",
        Card::new(CardStyle::PendingQuestion)
            .title("Read this file?")
            .subtitle("Claude wants to access a file")
            .status_badge("Question")
            .source_badge("fs")
            .body_line(BodyLine { prefix: None, text: "File: /etc/config.json".into() })
            .tool("file_read", Some("Read /etc/config.json".into()))
            .action_hint("Allow / Deny / Approve All")
            .height(150.0),
        300.0,
        150.0,
    );

    // 3. Completion — session summary card
    demo_card(
        "Completion",
        Card::new(CardStyle::Completion)
            .title("Claude完成")
            .status_badge("完成")
            .source_badge("chat")
            .body_line(BodyLine { prefix: None, text: "已完成代码审查，发现 3 处优化建议".into() })
            .body_line(BodyLine { prefix: None, text: "性能提升约 15%".into() })
            .height(120.0),
        300.0,
        120.0,
    );

    // 4. Settings — settings card with rows
    demo_card(
        "Settings",
        Card::new(CardStyle::Settings)
            .title("Settings")
            .settings_rows(vec![
                ("Auto-approve".into(), "On".into(), true),
                ("Theme".into(), "Dark".into(), false),
                ("Font Size".into(), "14px".into(), false),
            ])
            .height(140.0),
        300.0,
        140.0,
    );

    // 5. PromptAssist — prompt suggestion card
    demo_card(
        "PromptAssist",
        Card::new(CardStyle::PromptAssist)
            .title("Generate commit message?")
            .subtitle("Based on staged changes")
            .status_badge("Suggestion")
            .body_line(BodyLine { prefix: Some("> ".into()), text: "feat: add user auth middleware".into() })
            .action_hint("Accept / Edit / Dismiss")
            .height(130.0),
        300.0,
        130.0,
    );

    // 6. Empty — placeholder card
    demo_card(
        "Empty",
        Card::new(CardStyle::Empty)
            .title("No content")
            .height(80.0),
        300.0,
        80.0,
    );

    println!("Close the window or press Ctrl+C to exit.");
    while window.poll_message() {}

    fn demo_card(label: &str, card: Card, width: f64, height: f64) {
        let mut host = WidgetHost::new();
        host.set_size(reef_core::geometry::Size { width, height });
        host.set_root(Box::new(card));
        let plan = host.render();
        println!("  {label}: {} primitives ({}x{})", plan.primitives.len(), width as u32, height as u32);
    }
}
