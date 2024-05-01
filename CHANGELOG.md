# Changelog

## _April 18th, 2024_ - 0.2.0

### Breaking changes

- Internal modules are no longer public.
- Remove `prelude` module.
- Make ref nodes listen to `keydown` Keyboard events.
- Remove uneeded features `csr` and `hydrate`.

### Enhancements

- Allow to import from root package instead of forcing the usage of `prelude` module.
- Do not depend on `log` if `debug` feature is not enabled.
- Relax dependency versions.
- Add compatibility with Leptos stable.
- Use `code` instead of `key` property for `KeyboardEvent`.

## _February 27th, 2024_- 0.1.5

- Clean up macros

## _February 23th, 2024_ - 0.1.4

- Update README
- Add `debug` feature, which produces logs in your browser console upon hotkey fires and scope triggers.

## _February 12th, 2024_ - 0.1.3

- Recognize `meta` key.
- String cleaning.

## _February 8th, 2024_ - 0.1.1

- Elevate `leptos` to v0.6.5
- Added `event.preventDefault()`.
