# Changelog

## Unreleased

- Reduce build size on release.
- Separate CHANGELOG from README.
- Add Cargo.lock to repository.
- Update dependencies to latest.

## 0.1.4

- Refactor drawing algorithms for readability.
- Fix comparison function for Date to avoid redrawing.

## 0.1.3

- Draw clock to alternate screen buffer to avoid clearing user information. Thanks [@Canop][8]!

## 0.1.2

- Fix README formatting (oops).

## 0.1.1

- Implement support for date formatting strings via `-f` option.
- Fix logic in Brush abstraction by only setting `dried` flag after writing.

## 0.1.0

- Initial release.
