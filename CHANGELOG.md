## Changelog

## [0.0.3] - 2021-12-26

### Added
- Additional error variants and improved handling
- General refactors
- Tests for additional `Note` error cases
- `NoteId` can now parse spaces

### Fixed
- Note preview error

## [0.0.2] - 2021-07-02

### Added
- Removal of dead links when using the `rm` command
- Tests for directory operations in a temporary directory

### Fixed
- Panics after calls to unwrap() on `entry.extension()`. Now use an empty
  string as default

## [0.0.1] - 2021-07-02

Initial release
