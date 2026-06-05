use reef_core::color::Color;
use reef_core::geometry::Size;
use reef_draw::DrawPrimitive;
use reef_dom::ReefRenderer;
use reef_vnode::{Container, Label};

fn main() {
    let mut renderer = ReefRenderer::new(Size { width: 360.0, height: 200.0 });

    // Use capitalized component names (Container, Label) — the rsx! macro
    // detects uppercase first letters and calls them as functions.
    // Lowercase names (<container>, <label>) create native elements directly.
    let ui = reef_view_macros::rsx! {
        <Container color={Color::rgb(18, 18, 22)} radius={16.0}>
            <Label text={"Hello, Reef!"} color={Color::WHITE} font_size={18}
                   weight={"bold"} alignment={"center"} />
        </Container>
    };

    let plan = renderer.render(ui);

    println!("═══ Reef UI DrawPlan (Capitalized Components) ═══");
    println!("viewport: {}x{}", plan.viewport.width, plan.viewport.height);
    println!("primitives: {}", plan.primitives.len());
    println!();

    for (i, p) in plan.primitives.iter().enumerate() {
        match p {
            DrawPrimitive::RoundRect { frame, radius, color, .. } => {
                println!("[{}] RoundRect  @ ({:.0},{:.0}) {:.0}x{:.0}  radius={:.0}  #{:02x}{:02x}{:02x}",
                    i, frame.x, frame.y, frame.width, frame.height, radius, color.r, color.g, color.b);
            }
            DrawPrimitive::Text { frame, text, size, alignment, .. } => {
                println!("[{}] Text       @ ({:.0},{:.0}) {:.0}x{:.0}  \"{}\" size={} {:?}",
                    i, frame.x, frame.y, frame.width, frame.height, text, size, alignment);
            }
            DrawPrimitive::ClipStart { frame } => {
                println!("[{}] ClipStart  @ ({:.0},{:.0}) {:.0}x{:.0}",
                    i, frame.x, frame.y, frame.width, frame.height);
            }
            DrawPrimitive::ClipEnd => println!("[{}] ClipEnd", i),
            other => println!("[{}] {:?}", i, other),
        }
    }

    println!();
    println!("═══ {} primitives generated ═══", plan.primitives.len());

    assert!(!plan.primitives.is_empty());
    assert!(plan.primitives.iter().any(|p| matches!(p, DrawPrimitive::RoundRect { .. })));
    assert!(plan.primitives.iter().any(|p| matches!(p, DrawPrimitive::Text { .. })));
    println!("✅ All assertions passed!");
}
