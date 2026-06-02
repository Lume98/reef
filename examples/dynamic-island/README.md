# Dynamic Island UI Notes

灵动岛 UI 主要写在 Rust 原生扩展里，不在 React 前端组件里。

## 主要入口

- `src/lib.rs`
  - `enter_dynamic_island_mode` 负责进入灵动岛模式。
  - 进入后会保存主窗口快照、显示原生灵动岛窗口，并隐藏主窗口。

- `src/native_window.rs`
  - Windows 下当前 `show()` 会委托到 `windows_native_panel`：
    - `spawn_platform_loops`
    - `create_native_panel`
    - `update_native_panel_snapshot`

## UI 结构与绘制

- `src/native_panel_renderer/visual_plan.rs`
  - 主要 UI 布局和视觉元素生成位置。
  - 决定紧凑态/展开态的背景、标题、数字、按钮、卡片等 visual primitives。

- `src/native_panel_renderer/visual_primitives.rs`
  - 定义可绘制的 primitive 类型，例如圆角矩形、文本、图片、吉祥物、紧凑态肩部形状等。

- `src/native_panel_renderer/card_visual_spec.rs`
  - 卡片视觉规格。

- `src/native_panel_renderer/action_button_visual_spec.rs`
  - 操作按钮视觉规格。

- `src/native_panel_renderer/mascot_visual_spec.rs`
  - 吉祥物视觉规格。

- `src/windows_native_panel/d2d_painter.rs`
  - Windows Direct2D 绘制实现。
  - 负责把 visual plan 转成实际窗口上的图形。

## 场景数据

- `src/native_panel_scene/build.rs`
  - 根据运行时状态构建面板场景。

- `src/native_panel_scene/`
  - 维护设置、状态、会话、卡片等场景结构。

## 前端关系

React 前端只负责监听和切换灵动岛状态，不是灵动岛 UI 的主要实现位置：

- `../../src/app.tsx`
- `../../src/api/system/dynamic-island.ts`
- `../../src/index.css`

## 旧胶囊绘制代码

`src/native_window.rs` 中还保留了一套旧的胶囊绘制代码，例如：

- `paint_capsule`
- `draw_capsule_text`
- `render_capsule_pixels`

这部分属于较早的原生胶囊实现。当前 Windows 实现优先看 `windows_native_panel` 和 `native_panel_renderer` 这一套。
