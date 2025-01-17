# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.10.1 (2021-05-31)
### Added
- Nightly-only `armv8` feature ([#318])

[#318]: https://github.com/RustCrypto/AEADs/pull/318

## 0.10.0 (2021-04-29)
### Added
- Wycheproof test vectors ([#274])

### Changed
- Bump `aead` crate dependency to v0.4 ([#270])
- Bump `aes` and `ctr` crate dependencies to v0.7 ([#283])
- Bump `polyval` to v0.5 ([#284])

### Fixed
- Interleaved buffer size ([#235])

[#235]: https://github.com/RustCrypto/AEADs/pull/235
[#270]: https://github.com/RustCrypto/AEADs/pull/270
[#274]: https://github.com/RustCrypto/AEADs/pull/274
[#283]: https://github.com/RustCrypto/AEADs/pull/283
[#284]: https://github.com/RustCrypto/AEADs/pull/284

## 0.9.0 (2020-10-16)
### Changed
- Replace `block-cipher`/`stream-cipher` with `cipher` crate ([#229])
- Bump `aes` dependency to v0.6 ([#229])
- Use `ctr::Ctr32LE` ([#227])

[#229]: https://github.com/RustCrypto/AEADs/pull/229
[#227]: https://github.com/RustCrypto/AEADs/pull/227

## 0.8.0 (2020-09-17)
### Added
- Optional `std` feature; disabled by default ([#217])

### Changed
- Upgrade `aes` to v0.5; `block-cipher` to v0.8 ([#209])

[#217]: https://github.com/RustCrypto/AEADs/pull/217
[#209]: https://github.com/RustCrypto/AEADs/pull/209

## 0.7.0 (skipped)

## 0.6.0 (skipped)

## 0.5.0 (2020-06-06)
### Changed
- Bump `aead` crate dependency to v0.3.0; MSRV 1.41+ ([#142])

[#142]: https://github.com/RustCrypto/AEADs/pull/143

## 0.4.1 (2020-03-09)
### Fixed
- Off-by-one error in `debug_assert` for `BlockCipher::ParBlocks` ([#104])

[#104]: https://github.com/RustCrypto/AEADs/pull/104

## 0.4.0 (2020-03-07) - YANKED, see [#104]
### Added
- `aes` cargo feature; 3rd-party AES crate support ([#90])

### Changed
- Make generic around `BlockCipher::ParBlocks` ([#91], [#93])

[#90]: https://github.com/RustCrypto/AEADs/pull/90
[#91]: https://github.com/RustCrypto/AEADs/pull/91
[#93]: https://github.com/RustCrypto/AEADs/pull/93

## 0.3.0 (2019-11-26)
### Added
- `heapless` feature ([#51])

### Changed
- Upgrade `aead` crate to v0.2; `alloc` now optional ([#43])

[#51]: https://github.com/RustCrypto/AEADs/pull/51
[#43]: https://github.com/RustCrypto/AEADs/pull/43

## 0.2.1 (2019-11-14)
### Changed
- Upgrade to `zeroize` 1.0 ([#36])

[#36]: https://github.com/RustCrypto/AEADs/pull/36

## 0.2.0 (2019-10-06)
### Added
- Expose "detached" in-place encryption/decryption APIs ([#21])

### Changed
- Upgrade to `polyval` v0.3 ([#25])

[#25]: https://github.com/RustCrypto/AEADs/pull/25
[#21]: https://github.com/RustCrypto/AEADs/pull/21

## 0.1.0 (2019-09-28)
- Initial release
