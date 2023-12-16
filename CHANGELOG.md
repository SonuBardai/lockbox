All notable changes to Lockbox are documented in this file.
The sections should follow the order `Packaging`, `Added`, `Changed`, `Fixed` and `Removed`.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

---

## 0.1.2

### Packaging
-

### Added
-

### Changed
- Replace `colored` crate with `crossterm` for colored output. [Issue](https://github.com/SonuBardai/lockbox/issues/71)
- Replace `clipboard` crate with `copypasta` for copying to clipboard. [Issue](https://github.com/SonuBardai/lockbox/issues/60)
- Setting up a password store or updating the master password will now prompt the user to re-enter the master password [Issue](https://github.com/SonuBardai/lockbox/issues/56)

### Fixed
- Duplicate print statement in remove password command [Issue](https://github.com/SonuBardai/lockbox/issues/66)
- Missing linux dependencies added by `clipboard` crate [Issue](https://github.com/SonuBardai/lockbox/issues/73)
- Copy password to clipboard on the show command [Issue](https://github.com/SonuBardai/lockbox/issues/87)

---

## 0.1.1
### Changed
- Set default password store path relative to $HOME dir [Issue](https://github.com/SonuBardai/lockbox/issues/59)
