# Changelog

## [0.2.0-alpha.5](https://github.com/soteenstudio/lightvm/compare/projek-core-rust-v0.1.0-alpha.5...projek-core-rust-v0.2.0-alpha.5) (2026-05-25)


### Features

* add error messages to comparison category opcodes ([0fdea66](https://github.com/soteenstudio/lightvm/commit/0fdea66bd58f5249584c7cf70e7660b816a9c7ce))
* add error messages to math category opcodes ([4f812d4](https://github.com/soteenstudio/lightvm/commit/4f812d452132c58da49b0600ce9b52679bcac61e))
* add error messages to opcodes in metadata and stack categories ([7b06441](https://github.com/soteenstudio/lightvm/commit/7b06441c08b562f1e13dc7a2179a1b6f0a7eb83a))
* add internal opcode push_string, push_array, push_object, push_null, and push_nan ([67685d2](https://github.com/soteenstudio/lightvm/commit/67685d2123103aed9ef7ae54f0c58b204dac622c))
* add opcode cleae_screen ([31a3129](https://github.com/soteenstudio/lightvm/commit/31a3129655f2b7b93d2245a013e1950430f485df))
* add opcode neg ([ac7f6b7](https://github.com/soteenstudio/lightvm/commit/ac7f6b75a35f243013e5a6595df557786bf34097))
* add opcode swap ([2a862d6](https://github.com/soteenstudio/lightvm/commit/2a862d62d7ed42770505ea7438d74af67529598c))
* implement init_stack opcode parsing ([5c73489](https://github.com/soteenstudio/lightvm/commit/5c7348919c834b6e453ef724705a0c3d9a6b153f))
* standardize wrapping arithmetic and float nan fallback ([657f275](https://github.com/soteenstudio/lightvm/commit/657f275c5aba9e220667e6ddd84882a431b419bb))


### Bug Fixes

* fix dead loop elimination ([f8ad544](https://github.com/soteenstudio/lightvm/commit/f8ad54475cdb725829031883a45927f501168bdb))
* fix error message typo ([1521d26](https://github.com/soteenstudio/lightvm/commit/1521d261a92f8bcce657ee353c2efb4e0c1dbeaa))
* fix filtered writer ([9198a6c](https://github.com/soteenstudio/lightvm/commit/9198a6cdbe8143cf7b2ab245967295527af3ae37))
* fix the data type that the powi opcode accepts ([48c103a](https://github.com/soteenstudio/lightvm/commit/48c103a49a2126578cca1a2d9d3efbc464298b81))
* fixed dead_redundant_loads.rs logic ([173e3f1](https://github.com/soteenstudio/lightvm/commit/173e3f10dbc3fc3fe8b298f4e95fc0bed3256082))
* fixed instructions.rs in handling sin, cos, tan opcodes ([a56eba4](https://github.com/soteenstudio/lightvm/commit/a56eba44cda0c8fa44efd4990847585ee890a6e3))
* math identity and expand DSE opcode matching ([2c8a93b](https://github.com/soteenstudio/lightvm/commit/2c8a93b6f8c1ec4aba4f56aeea06ddeb1a729475))
* no bytecode error ([d42b153](https://github.com/soteenstudio/lightvm/commit/d42b1530da1a988282f4a7d07c6d072ca131b925))
* number representation error in data type hlf(f16) ([163e8c0](https://github.com/soteenstudio/lightvm/commit/163e8c0504c608976fd2d64efa2b2297d8e0d9a0))
* prevent stack underflow crash on PRINT and PRINTLN ([1758074](https://github.com/soteenstudio/lightvm/commit/1758074e8807a0cd86324ea7ac48a1440e16063d))
* unify error messages and refactor N-API loading ([65452e7](https://github.com/soteenstudio/lightvm/commit/65452e7c947f3cfb5ade4cf38eabacca95f3214a))


### Performance Improvements

* add #[inline(always)] ([fff1ef6](https://github.com/soteenstudio/lightvm/commit/fff1ef61924d44ccd00ca0d5f7b1c37e2acd79ba))
* improve fold conversion performance ([dbfc112](https://github.com/soteenstudio/lightvm/commit/dbfc11223b793a236035b6f33b7a2a0f9069c7bc))
* improve performance with munggukana Cow ([e3f538c](https://github.com/soteenstudio/lightvm/commit/e3f538c113c537f230dc3ff5654cb5eb90b19492))
* optimize code ([7886d4e](https://github.com/soteenstudio/lightvm/commit/7886d4e1b3123c04ffefc29683ac69fae8112760))
* optimize code ([e682775](https://github.com/soteenstudio/lightvm/commit/e682775473102d058d7b7c01204624c5854a8dbc))
* optimize performance by creating a standalone category for dispatch collections ([5809966](https://github.com/soteenstudio/lightvm/commit/5809966f1969ec383ef70271313414d75c237be7))
* optimize performance in several parts that are suspected of causing poor performance ([5a37022](https://github.com/soteenstudio/lightvm/commit/5a3702213f1e993c2f25eed5e541d7ebf9146a3c))
* provide data type support to push opcodes to improve performance ([745412b](https://github.com/soteenstudio/lightvm/commit/745412b97ba3d92bdfc88a070e0930be21218756))
