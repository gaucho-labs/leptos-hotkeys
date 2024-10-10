# Changelog

## _Aug 16, 2024_ - [0.2.21]
- Upgraded wasm_bindgen to 0.2.93


## _July 2nd, 2024_ - [0.2.2]
- Fix bugs related to keycode management

## _May 27th, 2024_ - [0.2.1]
- Code movement, change to using keycodes


- Fix right/left modifiers not being recognized.

### Enhancements

- Add `FromStr` implementation for `Hotkey` struct.

## _April 18th, 2024_ - [0.2.0]

### Breaking changes

- Internal modules are no longer public.
- Remove `prelude` module.
- Make ref nodes listen to `keydown` Keyboard events.
- Remove uneeded features `csr` and `hydrate`.
- Use `code` instead of `key` property for `KeyboardEvent`.

### Enhancements

- Allow to import from root package instead of forcing the usage of `prelude` module.
- Do not depend on `log` if `debug` feature is not enabled.
- Relax dependency versions.
- Add compatibility with Leptos stable.

## _February 27th, 2024_ - 0.1.5

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

[0.2.1]: https://github.com/gaucho-labs/leptos-hotkeys/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/gaucho-labs/leptos-hotkeys/compare/b83afc96...v0.2.0
