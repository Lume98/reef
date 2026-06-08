# Dynamic Island UI Notes

灵动岛原生 UI 现在收口在 `framework/reef-native-panel`。

## 模块结构

- `reef_native_panel::state`
  - 面板状态、布局常量、交互状态和刷新队列等纯逻辑。
- `reef_native_panel::scene`
  - 将 `RuntimeSnapshot` 和面板状态转换成 Surface、Card、Settings 等场景模型。
- `reef_native_panel::presentation`
  - 宿主窗口描述、表现模型、视觉计划和渲染命令。
- `reef_native_panel::runtime`
  - 平台无关的运行时协调逻辑。
- `reef_native_panel::platform::windows`
  - Windows 窗口、Direct2D、DirectWrite 和平台事件循环实现。

## 示例入口

`examples/dynamic-island` 只依赖：

- `echoisland-runtime`
- `reef-native-panel`

示例先构建一份 preview snapshot 和 scene，用于验证状态到场景的转换；随后调用
`reef_native_panel::run_dynamic_island_preview_standalone()` 启动 Windows standalone 预览。

```rust
use reef_native_panel::{
    scene::{build_panel_scene, PanelSceneBuildInput},
    state::PanelState,
};

let scene = build_panel_scene(
    &PanelState::default(),
    &snapshot,
    &PanelSceneBuildInput::default(),
);
```
