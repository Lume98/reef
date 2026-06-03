use reef_view::widget_host::{PaintContext, Widget};
use reef_core::geometry::{Point, Rect, Size};
use reef_layout::Constraints;
use reef_render::primitive::VisualPrimitive;

/// Pre-parsed sprite animation manifest with tile layout.
#[derive(Clone, Debug)]
pub struct SpriteSheet {
    pub path: String,
    pub columns: usize,
    pub rows: usize,
    pub cell_width: f64,
    pub cell_height: f64,
    pub pixel_ratio: f64,
}

/// Animation definition for a pose.
#[derive(Clone, Debug)]
pub struct SpriteAnimation {
    pub pose: MascotPoseKey,
    pub row: usize,
    pub frames: usize,
    pub frame_ms: u128,
    pub looped: bool,
    pub logical_width: f64,
    pub logical_height: f64,
    pub anchor_x: f64,
    pub anchor_y: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MascotPoseKey {
    Idle,
    Running,
    Approval,
    Question,
    Complete,
    Sleepy,
    WakeAngry,
    MessageBubble,
}

/// Sprite-based mascot renderer. Requires a pre-loaded SpriteSheet and animations.
#[derive(Clone)]
pub struct SpriteMascot {
    pub sheet: SpriteSheet,
    pub animations: Vec<SpriteAnimation>,
    pub current_pose: MascotPoseKey,
    pub center: Point,
    pub elapsed_ms: u128,
    pub opacity: f64,
}

impl SpriteMascot {
    pub fn new(sheet: SpriteSheet, animations: Vec<SpriteAnimation>) -> Self {
        Self {
            sheet,
            animations,
            current_pose: MascotPoseKey::Idle,
            center: Point { x: 0.0, y: 0.0 },
            elapsed_ms: 0,
            opacity: 1.0,
        }
    }

    /// Compute which frame to render for the current pose and elapsed time.
    fn resolve_frame(&self) -> Option<(&SpriteAnimation, usize, Rect, Rect)> {
        let anim = self
            .animations
            .iter()
            .find(|a| a.pose == self.current_pose)?;
        let frame_count = anim.frames.max(1);
        let cycle_ms = anim.frame_ms * frame_count as u128;
        let elapsed_in_cycle = if anim.looped {
            self.elapsed_ms % cycle_ms
        } else {
            self.elapsed_ms.min(cycle_ms.saturating_sub(1))
        };
        let frame_index = (elapsed_in_cycle / anim.frame_ms) as usize % frame_count;

        // Source rect in the sprite sheet
        let col = frame_index % self.sheet.columns;
        let row = anim.row;
        let src = Rect {
            x: col as f64 * self.sheet.cell_width,
            y: row as f64 * self.sheet.cell_height,
            width: self.sheet.cell_width,
            height: self.sheet.cell_height,
        };

        // Destination frame in logical space
        let logical_w = anim.logical_width / self.sheet.pixel_ratio;
        let logical_h = anim.logical_height / self.sheet.pixel_ratio;
        let dest = Rect {
            x: self.center.x - anim.anchor_x * logical_w,
            y: self.center.y - anim.anchor_y * logical_h,
            width: logical_w,
            height: logical_h,
        };

        Some((anim, frame_index, src, dest))
    }
}

impl Widget for SpriteMascot {
    fn measure(&self, _constraints: Constraints) -> Size {
        if let Some((anim, _, _, _)) = self.resolve_frame() {
            Size {
                width: anim.logical_width,
                height: anim.logical_height,
            }
        } else {
            Size {
                width: 0.0,
                height: 0.0,
            }
        }
    }

    fn paint(&self, _rect: Rect, ctx: &mut PaintContext) {
        if let Some((_anim, _frame_index, src, dest)) = self.resolve_frame() {
            ctx.primitives.push(VisualPrimitive::NineSliceImage {
                key: self.sheet.path.clone(),
                frame: dest,
                slice_left: src.x,
                slice_right: self.sheet.cell_width - src.x - src.width,
                slice_top: src.y,
                slice_bottom: self.sheet.cell_height - src.y - src.height,
                opacity: self.opacity,
            });
        }
    }
}
