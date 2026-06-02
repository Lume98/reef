# Island

这个目录是从 `ai-gateway/extensions` 拆出来的 Rust workspace，常用入口包括：

- `examples/dynamic-island`：灵动岛原生窗口扩展，带可执行入口 `island-ui`
- `echoisland-runtime`：共享运行时类型定义
- `framework/reef-ui`：Reef UI 模型与场景包

## 目录结构

```text
island/
  Cargo.toml
  echoisland-runtime/
  examples/
    dynamic-island/
    hello-reef/
  framework/
```

## 运行

在这个目录下直接运行：

```powershell
cd D:\github\island
cargo run -p dynamic_island --bin island-ui --features tauri-host
```

## 构建

```powershell
cd D:\github\island
cargo build
```

## 测试

```powershell
cd D:\github\island
cargo test
```

## 说明

- 这个 workspace 的根配置在 [`Cargo.toml`](./Cargo.toml)
- 真正的启动入口在 [`examples/dynamic-island/src/bin/island-ui.rs`](./examples/dynamic-island/src/bin/island-ui.rs)
- 构建产物默认会输出到 `D:\github\island\target`
