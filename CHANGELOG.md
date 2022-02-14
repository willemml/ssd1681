# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

All dates are in the format MM/DD/YYYY

## [0.3.0] 2/14/2022
### Added
- Full and partial refresh LUTs and a way to swap between them.
- Added experimental window support to the driver.
- Gray4 LUT for the SSD1681 thanks to GoodDisplay's examples.
- The main buffer now supports embedded-graphics' `Gray2` type as the color, and constants are exported from the color module with the correct values for the 4 gray values.

### Changed
- Updated `embedded-hal` to `1.0.0-alpha6` for compatibility with the future 1.0.0 release (and `esp-idf-hal`).
- Buffers no longer use `BinaryColor` and now use `Gray2`. They also store the colors in 2 5000 byte buffers. This may not be desirable, but it is alright for my application (ESP32 smartwatch).
