# AGENTS.md - bevy_steamworks

> 本文件是本仓库的工程约束单一来源，参考 `../maxio/AGENTS.md` 与 `../maxio/CLAUDE.md`，但内容已按 Rust / Bevy / Steamworks 插件裁剪。

## 项目目标

`bevy_steamworks` 是一个 Rust crate，用 `steamworks` crate 封装 Steamworks SDK，并以 Bevy plugin 的形式暴露给 Bevy 应用：

- 初始化 Steamworks client。
- 将 Steamworks client 作为 Bevy ECS resource 暴露。
- 在 Bevy schedule 中自动 pump Steam callbacks。
- 将 Steam callback 结果转成 Bevy message，方便系统读取。
- 在初始化失败时提供明确错误资源和结构化日志，不能静默失败。

## 工程约束

1. **统一契约优先**：对外稳定入口是 `SteamworksPlugin`、`SteamworksClient`、`SteamworksEvent`、`SteamworksUnavailable` 与 `SteamworksCallbackRegistry`。新增功能优先围绕这些契约扩展，不把 `steamworks` 原始细节散落进插件生命周期代码。
2. **插件边界清晰**：Steamworks SDK 绑定交给上游 `steamworks` crate；除非有明确证据证明必须下探，否则不要直接依赖 raw SDK / FFI。
3. **绝不静默失败**：默认初始化策略必须 fail-fast。允许开发期 `LogAndContinue`，但必须插入 `SteamworksUnavailable` resource，并通过 `tracing` 写结构化日志。
4. **单 client 生命周期**：Steamworks 官方客户端进程内只应初始化一次。不要设计会重复创建多个 `steamworks::Client` 的 API；手动 client 注入必须通过 `SteamworksClient` resource 或 `SteamworksPlugin::from_client`。
5. **回调时序显式**：Steam callbacks 由 `SteamworksSystem::RunCallbacks` 集中执行，默认放在 Bevy `First` schedule。需要依赖 Steam callback 的系统应显式排在该 set 之后。
6. **不阻塞帧循环**：插件系统不得执行长时间阻塞逻辑。异步 Steam API 结果通过 Steam callback / Bevy message 进入 ECS。
7. **可观测优先**：关键生命周期事件使用 `tracing`，至少包含 init mode、app id、错误信息等字段。
8. **资料优先**：Steamworks / Bevy API 不确定时，先查官方文档、docs.rs 或上游 crate 源码，再实现；不要凭记忆写 API。
9. **验证才算完成**：改动完成前至少运行 `cargo fmt --check`、`cargo test`，可行时运行 `cargo clippy --all-targets -- -D warnings`。涉及示例或公开 API 时运行 `cargo check --examples`。
10. **审查与实现分离**：可用 multi-agent 时，开发与审查必须由不同 agent 承担。若当前工具环境未获用户授权使用 subagent，不能声称已独立审查；只能报告实际执行过的验证命令和剩余风险。

## 依赖策略

- Bevy 依赖稳定版本线。不要默认追随 `rc` / prerelease。
- 插件核心只依赖 `bevy_app`、`bevy_ecs` 等模块化 crate，避免把完整渲染栈强加给库用户。
- 示例和 dev-dependencies 可以使用完整 `bevy`。

## 文件约定

- `src/lib.rs`：插件主体与公开契约。
- `examples/`：可运行集成示例；示例必须能在没有 Steam 客户端时通过 `LogAndContinue` 启动。
- `README.md`：安装、使用、失败策略、callback 行为和 Steam redistributable 注意事项。
