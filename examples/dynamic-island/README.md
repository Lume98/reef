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

- `src/business/`
  - 业务规则、显示器选择、面板输入构造和测试。
  - 这里不处理窗口绘制，只负责把应用设置和显示器信息归一化。

- `src/native_panel_renderer/`
  - 原生渲染协调层。
  - `visual_plan.rs` 负责把场景模型转换为 visual plan。
  - `facade.rs` 负责向 Windows 平台层集中导出命令、描述符、运行时和视觉能力。
  - 共享契约后续抽到了 `framework/reef-native-panel-core`，Windows 适配层对应 `framework/reef-native-panel-windows`。

- `framework/reef-ui/src/native_panel_ui/`
  - 共享的场景、表现模型、视觉计划和渲染计划定义。
  - `dynamic-island` 只消费这些模型，不在这里重复实现布局规则。

- `src/windows_native_panel/d2d_painter.rs`
  - Windows Direct2D 绘制实现。
  - 负责把 visual plan 转成实际窗口上的图形。

## 场景数据

- `src/native_panel_scene/build.rs`
  - 根据运行时状态构建面板场景。

- `src/native_panel_scene/`
  - 维护设置、状态、会话、卡片等场景结构。

- `src/native_panel_scene_input.rs`
  - 从 `business` 层薄封装而来的场景输入适配层。
  - 主要用于保留旧调用路径，同时让入口更集中。

## 前端关系

React 前端只负责监听和切换灵动岛状态，不是灵动岛 UI 的主要实现位置：

- `../../src/app.tsx`
- `../../src/api/system/dynamic-island.ts`
- `../../src/index.css`

## 声明式搭建

现在 `examples/dynamic-island` 可以直接按组件树声明 UI，再交给 root 渲染，入口风格接近 `React.createRoot(...).render(...)`：

```rust
use dynamic_island::view::{
    create_root, BodyLine, Card, CardStyle, CompactBar, DisplayMode, IslandWidget, MascotPose,
    MascotWidget,
};
use reef_core::geometry::Size;

let mut root = create_root(Size {
    width: 400.0,
    height: 300.0,
});

let plan = root.render(
    IslandWidget::new()
        .mode(DisplayMode::Expanded)
        .compact_bar(
            CompactBar::new()
                .headline("Reef")
                .counts("2", "5")
                .show_actions(true),
        )
        .card(
            Card::new(CardStyle::PendingApproval)
                .title("Allow command?")
                .body_line(BodyLine::plain(Some("$"), "cargo test -p dynamic_island"))
                .height(120.0),
        )
        .mascot(MascotWidget::new(200.0, 24.0, 14.0).pose(MascotPose::Running)),
);
```

运行时路径和这个声明式入口复用同一个 `IslandWidget` 组件树，只是内容来自 `RuntimeSnapshot -> IslandWidget` 适配层，尺寸和动画参数仍由 renderer 在最后一跳注入。

## 旧胶囊绘制代码

`src/native_window.rs` 中还保留了一套旧的胶囊绘制代码，例如：

- `paint_capsule`
- `draw_capsule_text`
- `render_capsule_pixels`

这部分属于较早的原生胶囊实现。当前 Windows 实现优先看 `windows_native_panel` 和 `native_panel_renderer` 这一套。
