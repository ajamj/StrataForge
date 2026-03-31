# Design Doc: Parallel Debugging and Workspace Cleanup

## 1. Problem Statement
The workspace currently has multiple independent build failures and a high volume of technical debt in the form of warnings and deprecated methods.
- `seisly_fwi`: Compilation errors from incorrect `ndarray` method usage.
- `seisly_tracking`: Missing `candle` dependencies causing unresolved imports.
- `seisly_app`: Deprecation warnings in the UI layer.
- `seisly_ml_deep`: Redundant syntax and unused code.

## 2. Goals
- Resolve all compilation errors so `cargo check` passes workspace-wide.
- Eliminate high-priority warnings (deprecations, syntax).
- Verify the fixes through automated tests.
- Demonstrate parallel coordination between four AI agents (Codex, Gemini, OpenCode, and Lead).

## 3. Architecture & Coordination
We will operate as a distributed team:
1. **Codex (Surgical Fixes)**: Focuses on `crates/seisly_fwi/src/gradient.rs`.
2. **Gemini (Dependency Logic)**: Manages `crates/seisly_tracking/Cargo.toml` and imports.
3. **OpenCode (Frontend Polish)**: Updates `crates/seisly_app/src/widgets/viewport.rs`.
4. **Lead (Core Optimization)**: Handles `crates/seisly_ml_deep/src/training_dl.rs` and final integration.

## 4. Approach
- Use non-interactive CLI commands to dispatch agents.
- Each agent performs a single, focused task.
- Lead (Me) validates each change and runs a final workspace check.

## 5. Success Criteria
- `cargo check --workspace` returns 0 errors and significantly reduced warnings.
- `cargo test` runs without compilation failure.
