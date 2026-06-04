use reef_core::geometry::Rect;
use reef_render::primitive::{VisualPlan, VisualPrimitive};
use reef_view::widget_host::{PaintContext, Widget};

use crate::island::ExpandedCardStack;

use super::{display_mode::DisplayMode, widget::IslandWidget};

pub fn render_island_widget(widget: &IslandWidget) -> VisualPlan {
    let rect = Rect {
        x: 0.0,
        y: 0.0,
        width: widget.width,
        height: widget.expanded_height.max(widget.compact_height),
    };
    let mut primitives = Vec::new();
    render_island_widget_primitives(widget, rect, &mut primitives);
    VisualPlan {
        hidden: widget.mode == DisplayMode::Hidden,
        primitives,
    }
}

pub(crate) fn render_island_widget_primitives(
    widget: &IslandWidget,
    rect: Rect,
    primitives: &mut Vec<VisualPrimitive>,
) {
    if widget.mode == DisplayMode::Hidden {
        return;
    }

    let mut ctx = reef_view::widget_host::PaintContext { primitives };
    paint_island_widget(widget, rect, &mut ctx);
}

pub(crate) fn paint_island_widget(widget: &IslandWidget, rect: Rect, ctx: &mut PaintContext) {
    if widget.mode == DisplayMode::Hidden {
        return;
    }

    if let Some(glow) = &widget.glow {
        glow.paint(rect, ctx);
    }

    if widget.mode == DisplayMode::Expanded {
        let shell_alpha = 1.0 - widget.chrome.collapsed_alpha;
        let mut shell = widget.expanded_shell.clone();
        shell.alpha = shell_alpha;

        let sep_vis = widget.chrome.separator_visibility.clamp(0.0, 1.0);
        if sep_vis > 0.0 {
            let bar_y = rect.height - widget.compact_height;
            shell.separator_y = Some(bar_y);
            shell.separator_color =
                reef_core::color::Color::rgba(40, 44, 54, (0.5 * sep_vis * 255.0) as u8);
        }
        shell.paint(rect, ctx);

        ExpandedCardStack::new(
            widget.cards.clone(),
            widget.compact_height,
            widget.reveal_progress,
            widget.entering,
        )
        .paint(rect, ctx);
    }

    let bar_rect = if widget.mode == DisplayMode::Compact {
        rect
    } else {
        Rect {
            x: rect.x,
            y: rect.y + rect.height - widget.compact_height,
            width: rect.width,
            height: widget.compact_height,
        }
    };

    if let Some(shoulder) = &widget.shoulder_left {
        shoulder.paint(bar_rect, ctx);
    }
    if let Some(shoulder) = &widget.shoulder_right {
        shoulder.paint(bar_rect, ctx);
    }

    ctx.primitives
        .push(VisualPrimitive::ClipStart { frame: bar_rect });
    let mut bar = widget.compact_bar.clone();
    if widget.mode == DisplayMode::Expanded {
        bar.chrome = widget.chrome;
    }
    bar.paint(bar_rect, ctx);
    ctx.primitives.push(VisualPrimitive::ClipEnd);

    if let Some(mascot) = &widget.mascot {
        mascot.paint(rect, ctx);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compact_bar::ChromeVisibility;

    #[test]
    fn render_island_widget_marks_hidden_mode_as_hidden() {
        let widget = IslandWidget::default();

        let plan = render_island_widget(&widget);

        assert!(plan.hidden);
        assert!(plan.primitives.is_empty());
    }

    #[test]
    fn render_island_widget_produces_visible_primitives() {
        let mut widget = IslandWidget::default();
        widget.mode = DisplayMode::Compact;
        widget.width = 320.0;
        widget.compact_height = 48.0;
        widget.expanded_height = 220.0;
        widget.chrome = ChromeVisibility::compact();

        let plan = render_island_widget(&widget);

        assert!(!plan.hidden);
        assert!(!plan.primitives.is_empty());
    }
}
