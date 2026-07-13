# Arsitektur yang Didukung
**LightVM** mendukung berbagai platform dan arsitektur untuk memastikan fleksibilitas operasional maksimal. Berikut daftar kompatibilitas saat ini.

| OS / Runtime | Arsitektur | Toolchain | Rust | Node.js | Web |
|--------------|--------------|-----------|-------|---------|-----|
| Windows      | x64, ia32    | MSVC      | ✓ | ✓ | - |
| Linux        | x64, ia32, arm64 | GNU (glibc) | ✓ | ✓ | - |
| Linux (musl) | x64, ia32, arm64 | musl      | ✓ | ✓ | - |
| macOS (Darwin) | x64      | Apple Clang | ✓ | ✓ | - |
| Android      | arm64, arm   | NDK       | ✓ | ✓ | - |
| FreeBSD      | x64          | Clang     | ✓ | ✓ | - |
| Web / Browser | wasm32 | wasm-pack / wasm-bindgen | - | - | ✓ |

::: warning
**Dukungan Nightly**: Dukungan untuk `wasm32` pada browser masih dalam tahap eksperimental.
:::