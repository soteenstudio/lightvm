# Supported Architectures
**LightVM** supports a wide range of platforms and architectures to ensure maximum operational flexibility. Here's the current compatibility list

| OS / Runtime | Architecture | Toolchain | Rust | Node.js | Web |
|--------------|--------------|-----------|-------|---------|-----|
| Windows      | x64, ia32    | MSVC      | ✓ | ✓ | - |
| Linux        | x64, ia32, arm64 | GNU (glibc) | ✓ | ✓ | - |
| Linux (musl) | x64, ia32, arm64 | musl      | ✓ | ✓ | - |
| macOS (Darwin) | x64      | Apple Clang | ✓ | ✓ | - |
| Android      | arm64, arm   | NDK       | ✓ | ✓ | - |
| FreeBSD      | x64          | Clang     | ✓ | ✓ | - |
| Web / Browser | wasm32 | wasm-pack / wasm-bindgen | - | - | ✓ |

::: warning
__Nightly Support__: Support for `wasm32` for browsers is still in experimental stage.
:::