use reef_app::app::App;
use reef_backend_d2d::painter::Direct2DPainter;
use reef_core::{
    color::Color,
    geometry::{Rect, Size},
};
use reef_render::primitive::VisualPrimitive;
use std::any::Any;
use reef_app::widget_host::{MeasureContext, PaintContext, Widget};
use reef_layout::Constraints;

struct HelloWidget {
    width: f64,
    height: f64,
}

impl Widget for HelloWidget {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn measure(&self, constraints: Constraints, _ctx: &mut MeasureContext) -> Size {
        constraints.constrain(Size {
            width: self.width,
            height: self.height,
        })
    }
    fn paint(&self, rect: Rect, ctx: &mut PaintContext) {
        // Background pill
        ctx.primitives.push(VisualPrimitive::RoundRect {
            frame: rect,
            radius: 24.0,
            color: Color::rgb(18, 18, 22),
            alpha: 1.0,
        });
        // Border
        ctx.primitives.push(VisualPrimitive::RoundRect {
            frame: Rect {
                x: rect.x + 0.5,
                y: rect.y + 0.5,
                width: rect.width - 1.0,
                height: rect.height - 1.0,
            },
            radius: 23.5,
            color: Color::rgb(44, 44, 50),
            alpha: 0.6,
        });
        // Text
        ctx.primitives.push(VisualPrimitive::Text {
            origin: reef_core::geometry::Point {
                x: rect.x + 12.0,
                y: rect.y + 10.0,
            },
            max_width: rect.width - 24.0,
            text: "Hello, Reef!".to_string(),
            color: Color::rgb(230, 235, 245),
            size: 16,
            weight: reef_render::primitive::FontWeight::Semibold,
            alignment: reef_render::primitive::TextAlignment::Center,
            alpha: 1.0,
        });
    }
}

fn main() {
    println!("=== Hello Reef ===");
    println!("Building a visual plan with the reef framework...");

    let backend = Direct2DPainter::new();
    let mut app = App::new(backend).with_size(320.0, 48.0);
    app.host_mut()
        .set_root(Box::new(HelloWidget {
            width: 320.0,
            height: 48.0,
        }));

    match app.render() {
        Ok(()) => println!("Frame submitted successfully."),
        Err(e) => eprintln!("Render error: {e}"),
    }

    let plan = app.host_mut().render();
    println!("Visual plan: {} primitives, hidden={}", plan.primitives.len(), plan.hidden);
    println!(
        "  Primitives: {:?}",
        plan.primitives
            .iter()
            .map(|p| match p {
                VisualPrimitive::RoundRect { .. } => "RoundRect".to_string(),
                VisualPrimitive::Text { text, .. } => format!("Text(\"{text}\")"),
                _ => "Other".to_string(),
            })
            .collect::<Vec<_>>()
    );
}
