# Test Driven Development (TDD) Guidelines for S3M2P

To ensure stability and prevent regressions (like runtime panics in WASM or 403 deployment errors), we adhere to the following TDD principles.

## 1. The Red-Green-Refactor Cycle
1.  **Red**: Write a failing test for the new feature or bug fix *before* writing any implementation code.
2.  **Green**: Write the minimum code necessary to make the test pass.
3.  **Refactor**: Clean up the code while ensuring tests remains green.

## 2. Critical Testing Areas
*   **Geometric Calculations**: Any code calculating radius, positions, or physics vectors MUST have unit tests to check for:
    *   Negative values (e.g., negative radius causing Canvas panics).
    *   NaN / Infinity values.
    *   Division by zero.
*   **State Transitions**: Logic that changes game state (e.g., Fungal Growth, Infection) must be tested for boundary conditions (max nodes, zero health).
*   **Serialization**: Data exchange formats (binary/JSON) between server and client must be tested for compatibility.

## 3. Project Structure for Testing
*   **Binaries (`main.rs`)**: Keep `main.rs` thin. Move logic into modules (`lib.rs` or specific files like `fungal.rs`) so they can be unit tested.
*   **Shared Logic**: Core physics belongs in `antimony-core` and must be fully covered.

## 4. CI/CD Enforcement
*   `cargo test` must pass in the CI pipeline before any deployment steps.
*   WASM targets should use `wasm-bindgen-test` if browser APIs are strictly required, but prefer headless logic tests where possible.

## 5. Example: Fixing a Panic
**Scenario**: `ctx.arc()` panics with negative radius.
**Test**:
```rust
#[test]
fn test_radius_clamping() {
    let mut node = FungalNode { health: -0.5, ..Default::default() };
    let radius = node.get_visual_radius();
    assert!(radius >= 0.0, "Radius must never be negative");
}
```
**Fix**: Implement `get_visual_radius` with `.max(0.0)`.

