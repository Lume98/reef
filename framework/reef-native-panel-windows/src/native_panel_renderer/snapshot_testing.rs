// Phase 3: 快照测试框架
//
// 为 native panel 渲染器提供视觉回归测试能力

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use reef_draw::primitive::{DrawPlan, DrawPrimitive};

/// 可序列化的渲染快照
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RenderSnapshot {
    /// 测试场景名称
    pub scenario: String,

    /// 渲染的图元列表
    pub primitives: Vec<SerializablePrimitive>,

    /// 元数据
    pub metadata: SnapshotMetadata,
}

/// 可序列化的图元
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SerializablePrimitive {
    #[serde(rename = "type")]
    pub primitive_type: String,

    #[serde(flatten)]
    pub data: serde_json::Value,
}

/// 快照元数据
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SnapshotMetadata {
    pub timestamp: String,
    pub display_mode: String,
    pub animation_progress: f64,
}

impl RenderSnapshot {
    /// 从渲染计划创建快照
    pub fn from_visual_plan(scenario: &str, plan: &DrawPlan, metadata: SnapshotMetadata) -> Self {
        let primitives = plan
            .primitives
            .iter()
            .map(SerializablePrimitive::from_primitive)
            .collect();

        Self {
            scenario: scenario.to_string(),
            primitives,
            metadata,
        }
    }

    /// 保存快照到文件
    pub fn save(&self, path: &PathBuf) -> Result<(), String> {
        let json = serde_json::to_string_pretty(self).map_err(|e| format!("序列化失败: {}", e))?;

        std::fs::write(path, json).map_err(|e| format!("写入文件失败: {}", e))?;

        Ok(())
    }

    /// 从文件加载快照
    pub fn load(path: &PathBuf) -> Result<Self, String> {
        let json = std::fs::read_to_string(path).map_err(|e| format!("读取文件失败: {}", e))?;

        serde_json::from_str(&json).map_err(|e| format!("反序列化失败: {}", e))
    }

    /// 比较两个快照的差异
    pub fn diff(&self, other: &Self) -> Vec<SnapshotDiff> {
        let mut diffs = Vec::new();

        if self.primitives.len() != other.primitives.len() {
            diffs.push(SnapshotDiff::PrimitiveCountMismatch {
                expected: self.primitives.len(),
                actual: other.primitives.len(),
            });
        }

        for (i, (expected, actual)) in self
            .primitives
            .iter()
            .zip(other.primitives.iter())
            .enumerate()
        {
            if expected != actual {
                diffs.push(SnapshotDiff::PrimitiveMismatch {
                    index: i,
                    expected: expected.clone(),
                    actual: actual.clone(),
                });
            }
        }

        diffs
    }
}

/// 快照差异类型
#[derive(Debug, Clone, PartialEq)]
pub enum SnapshotDiff {
    PrimitiveCountMismatch {
        expected: usize,
        actual: usize,
    },
    PrimitiveMismatch {
        index: usize,
        expected: SerializablePrimitive,
        actual: SerializablePrimitive,
    },
}

impl SerializablePrimitive {
    fn from_primitive(p: &DrawPrimitive) -> Self {
        match p {
            DrawPrimitive::RoundRect {
                frame,
                radius,
                color,
                alpha,
            } => Self {
                primitive_type: "RoundRect".to_string(),
                data: serde_json::json!({
                    "frame": {
                        "x": frame.x,
                        "y": frame.y,
                        "width": frame.width,
                        "height": frame.height,
                    },
                    "radius": radius,
                    "color": format!("rgb({}, {}, {})", color.r, color.g, color.b),
                    "alpha": alpha,
                }),
            },

            DrawPrimitive::Text {
                frame,
                text,
                color,
                size,
                weight,
                alignment,
                alpha,
            } => Self {
                primitive_type: "Text".to_string(),
                data: serde_json::json!({
                    "frame": {
                        "x": frame.x,
                        "y": frame.y,
                        "width": frame.width,
                        "height": frame.height,
                    },
                    "text": text,
                    "color": format!("rgb({}, {}, {})", color.r, color.g, color.b),
                    "size": size,
                    "weight": format!("{:?}", weight),
                    "alignment": format!("{:?}", alignment),
                    "alpha": alpha,
                }),
            },

            DrawPrimitive::Rect {
                frame,
                color,
                alpha,
            } => Self {
                primitive_type: "Rect".to_string(),
                data: serde_json::json!({
                    "frame": {
                        "x": frame.x,
                        "y": frame.y,
                        "width": frame.width,
                        "height": frame.height,
                    },
                    "color": format!("rgb({}, {}, {})", color.r, color.g, color.b),
                    "alpha": alpha,
                }),
            },

            DrawPrimitive::Path {
                segments,
                fill,
                alpha,
            } => Self {
                primitive_type: "Path".to_string(),
                data: serde_json::json!({
                    "fill": format!("rgb({}, {}, {})", fill.r, fill.g, fill.b),
                    "alpha": alpha,
                    "segments": segments.len(),
                }),
            },

            _ => Self {
                primitive_type: "Unknown".to_string(),
                data: serde_json::json!({}),
            },
        }
    }
}

/// 快照测试宏
#[macro_export]
macro_rules! assert_snapshot {
    ($snapshot:expr, $name:expr) => {{
        let snapshot_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("native_panel_renderer")
            .join("snapshots");

        std::fs::create_dir_all(&snapshot_dir).unwrap();

        let snapshot_path = snapshot_dir.join(format!("{}.json", $name));

        if snapshot_path.exists() {
            // 比较模式
            let expected = $crate::native_panel_renderer::snapshot_testing::RenderSnapshot::load(
                &snapshot_path,
            )
            .expect("加载快照失败");

            let diffs = expected.diff(&$snapshot);

            if !diffs.is_empty() {
                panic!("快照不匹配: {}\n差异:\n{:#?}", $name, diffs);
            }
        } else {
            // 记录模式
            $snapshot.save(&snapshot_path).expect("保存快照失败");

            println!("✓ 已创建快照: {}", $name);
        }
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use reef_core::{
        color::Color,
        geometry::{Rect, Size},
    };
    use reef_draw::primitive::{TextAlignment, TextWeight};

    #[test]
    fn test_snapshot_serialization() {
        let plan = DrawPlan {
            hidden: false,
            viewport: Size {
                width: 208.0,
                height: 44.0,
            },
            primitives: vec![
                DrawPrimitive::RoundRect {
                    frame: Rect {
                        x: 0.0,
                        y: 0.0,
                        width: 208.0,
                        height: 44.0,
                    },
                    radius: 22.0,
                    color: Color::rgb(18, 18, 22),
                    alpha: 1.0,
                },
                DrawPrimitive::Text {
                    frame: Rect {
                        x: 52.0,
                        y: 15.0,
                        width: 156.0,
                        height: 24.0,
                    },
                    text: "AI Gateway".to_string(),
                    color: Color::rgb(255, 255, 255),
                    size: 13,
                    weight: TextWeight::Semibold,
                    alignment: TextAlignment::Center,
                    alpha: 1.0,
                },
            ],
        };

        let snapshot = RenderSnapshot::from_visual_plan(
            "test_compact",
            &plan,
            SnapshotMetadata {
                timestamp: "2026-05-30".to_string(),
                display_mode: "Compact".to_string(),
                animation_progress: 0.0,
            },
        );

        assert_eq!(snapshot.scenario, "test_compact");
        assert_eq!(snapshot.primitives.len(), 2);
        assert_eq!(snapshot.primitives[0].primitive_type, "RoundRect");
        assert_eq!(snapshot.primitives[1].primitive_type, "Text");
    }

    #[test]
    fn test_snapshot_diff() {
        let snapshot1 = RenderSnapshot {
            scenario: "test".to_string(),
            primitives: vec![SerializablePrimitive {
                primitive_type: "RoundRect".to_string(),
                data: serde_json::json!({"width": 100}),
            }],
            metadata: SnapshotMetadata {
                timestamp: "2026-05-30".to_string(),
                display_mode: "Compact".to_string(),
                animation_progress: 0.0,
            },
        };

        let snapshot2 = RenderSnapshot {
            scenario: "test".to_string(),
            primitives: vec![SerializablePrimitive {
                primitive_type: "RoundRect".to_string(),
                data: serde_json::json!({"width": 200}),
            }],
            metadata: SnapshotMetadata {
                timestamp: "2026-05-30".to_string(),
                display_mode: "Compact".to_string(),
                animation_progress: 0.0,
            },
        };

        let diffs = snapshot1.diff(&snapshot2);
        assert_eq!(diffs.len(), 1);

        match &diffs[0] {
            SnapshotDiff::PrimitiveMismatch { index, .. } => {
                assert_eq!(*index, 0);
            }
            _ => panic!("Expected PrimitiveMismatch"),
        }
    }
}
