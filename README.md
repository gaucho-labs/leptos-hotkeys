# [_leptos-hotkeys_](https://github.com/gaucho-labs/leptos-hotkeys)
<!-- markdownlint-disable -->
<!-- ALL-CONTRIBUTORS-BADGE:START - Do not remove or modify this section -->
[![All Contributors](https://img.shields.io/badge/all_contributors-8-orange.svg?style=flat-square)](#contributors-)
<!-- ALL-CONTRIBUTORS-BADGE:END -->
<!-- markdownlint-restore -->

Declaratively create and pair keybindings with callbacks for Leptos applications.

[![Crates.io](https://img.shields.io/crates/v/leptos_hotkeys)](https://crates.io/crates/leptos_hotkeys)
[![discord](https://img.shields.io/badge/Join-Discord-%235865F2.svg)](https://discord.gg/XhVbKk38ux)

<!-- markdownlint-disable -->
<a href="https://github.com/gaucho-labs/leptos-hotkeys">
    <img width="570" height="211"
         alt="A person playing a burning piano at a sandy beach under a cloudy sky"
         src="https://raw.githubusercontent.com/gaucho-labs/leptos-hotkeys/main/images/readme-cover.png" />
</a>
<!-- markdownlint-restore -->

Leptos-hotkeys creates and manages keyboard shortcuts. It provides macros and functions that simplify the definition of
keybindings, since the management of event lifecycle associated with keyboard interactions has been done for you!

## Live example

Curious to see how it works? [See the demo](https://leptos-hotkeys.vercel.app).

To get started, follow the [Quick Start](#quick-start) section.

## Features

> [!NOTE]
> This crate has three types of hotkeys: global, scoped, and focus-trapped.


### The `use_hotkeys!` Macro

Use this macro to declare global and scoped hotkeys. This macro has js idioms while preserving Leptos standards. [More about the macro.](](#macro-api).)

### Global Hotkeys

This example creates two global hotkeys: `W` and `F`.

```rust
use leptos_hotkeys::use_hotkeys;

#[component]
pub fn SomeComponent() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    // creating a global scope for the W key
    use_hotkeys!(("keyw") => move |_| {
        logging::log!("w has been pressed");
        set_count.update(|c| *c += 1);
    });

    // this is also a global scope for the F key!
    use_hotkeys!(("keyf", "*") => move |_| {
        logging::log!("f has been pressed");
        set_count.update(|c| *c -= 1);
    });

    view! { <p>Num Respects: {count}</p> }
}
```

> [!TIP]
> How do I write certain keys? See [Key Grammar](#keybinding-grammar).

> [!NOTE]
> The `*` symbol is reserved for the global scope_.
>
> The `W` hotkey omitted the scope parameter, implicitly making it global.

### Scoped Hotkeys
Scopes provide context behind hotkeys. This context can be chained to a component, a state, or logic.

This example shows an inner and outer scope and hotkeys that toggle scopes.

```rust
use leptos_hotkeys::{use_hotkeys, use_hotkeys_context, HotkeysContext};

#[component]
pub fn SomeComponent() -> impl IntoView {

    let HotkeysContext { enable_scope, disable_scope, .. } = use_hotkeys_context();

    // switch into the inner scope
    use_hotkeys!(("keyi", "outer") => move |_| {
        disable_scope("outer");
        enable_scope("inner");
    });

    // switch into the outer scope
    use_hotkeys!(("keyo", "inner") => move |_| {
        disable_scope("inner");
        enable_scope("outer");
    });

    view! {
        <div id="outer">
            // outer logic residing...
            <div id="inner">
            // inner logic
            </div>
        </div>
    }
}
```

> [!NOTE]
> Scopes are case-insensitive. That means `my_scope` and `mY_sCoPe` are considered the same scope.

### Focus trapped Hotkeys (the `use_hotkeys_ref!` macro)

This example embeds a hotkey to a `<p>` tag. This hotkey will fire iff the element is focused and the scope is correct.

```rust
use leptos_hotkeys::use_hotkeys_ref;

#[component]
pub fn SomeComponent() -> impl IntoView {

    let p_ref = use_hotkeys_ref!(("keyk", "*") => move |_| {
        // some logic
    });

    view! {
        <p tabIndex=-1 _ref=p_ref>
            p tag with node ref
        </p>
    }
}
```

## Quick Start

### Installation

```shell
cargo add leptos_hotkeys
```

We also offer other feature flags that enhance developer experience, see [features](#features).

### `provide_hotkeys_context()`

Call `provide_hotkeys_context()` in the `App()` component. This will provide the `HotkeysContext` for the current reactive node and all of its descendents. This function takes three parameters, the `node_ref`, a flag to disable blur events and a list of `initially_active_scopes`.
`provide_hotkeys_context()` returns a `HotkeyContext`. To manage hotkeys, you can pull necessary signals out of `HotkeysContext`.

```rust
use leptos_hotkeys::{provide_hotkeys_context, HotkeysContext, scopes};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let main_ref = create_node_ref::<html::Main>();
    let HotkeysContext { .. } = provide_hotkeys_context(main_ref, false, scopes!());

    view! {
        <Router>
            <main _ref=main_ref>  // <-- attach main ref here!
                <Routes>
                    <Route path="/" view=HomePage/>
                    <Route path="/:else" view=ErrorPage/>
                </Routes>
            </main>
        </Router>
    }
}
```

> [!NOTE]
> If you're using [scopes](#scoped-hotkeys), you can initialize with a specific scope.

## That's it! [You can create global, scoped, and focus-trapped hotkeys!](#features)

### Keybinding Grammar

`leptos_hotkeys` matches code values from [KeyboardEvent's](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/code) `code` property. For reference, here's a list of [all code values for keyboard events](https://developer.mozilla.org/en-US/docs/Web/API/UI_Events/Keyboard_event_code_values).

You can bind multiple hotkeys to a callback. For example:

```txt
"KeyG+KeyR,MetaLeft+KeyO,ControlLeft+keyK"
```

Keys are case-agnostic and whitspace-agnostic. For a hotkey with multiple keys, use the `,` as a delimiter in a sequence of keys.

### `scopes!()`

Maybe you want to initialize a certain scope upon load, that's where the prop `initially_active_scopes` comes into play.
Instead of having to create a `vec!["scope_name".to_string()]`, use the `scopes!()` macro.

```rust
use leptos_hotkeys::{provide_hotkeys_context, scopes};

#[component]
pub fn App() -> impl IntoView {
    let main_ref = create_node_ref::<html::Main>();
    provide_hotkeys_context(main_ref, false, scopes!("scope_a", "settings_scope"));

    view! {
        <Router>
            <main _ref=main_ref>
                <Routes>
                    // ... routes
                </Routes>
            </main>
        </Router>
    }
}
```

## The `debug` feature flag

Improve developer experience by introducing the `debug` flag which adds logging to your console in CSR. It logs the current pressed key `code` values, hotkeys fires, and scopes toggling.

Just simply:

```toml
leptos_hotkeys = { path = "0.2.1", features = ["debug"] }
```

## The `event_key` feature flag

For improved accessibility options, this crate supports using either the event code or the event key for triggering events. This should improve compatibility with e.g. Dvorak keyboard layout. Note that this currently requires slight modification of hotkey definitions for alphanumeric keys. 

```rust
// with event code (without event_key feature flag)
use_hotkeys!(("keyw") => move |_| logging::log!("w has been pressed"));
use_hotkeys!(("digit1") => move |_| logging::log!("1 has been pressed"));

// with event_key feature flag
use_hotkeys!(("w") => move |_| logging::log!("w has been pressed"));
use_hotkeys!(("1") => move |_| logging::log!("1 has been pressed"));
```

To use event key as the identifier instead of event code, use:

```toml
leptos_hotkeys = { path = "0.2.1", features = ["event_key"] }
```

## Contributors
<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tbody>
    <tr>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/mondeja"><img src="https://avatars.githubusercontent.com/u/23049315?v=4?s=100" width="100px;" alt="Álvaro Mondéjar"/><br /><sub><b>Álvaro Mondéjar</b></sub></a><br /><a href="https://github.com/gaucho-labs/leptos-hotkeys/commits?author=mondeja" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/JustBobinAround"><img src="https://avatars.githubusercontent.com/u/67753581?v=4?s=100" width="100px;" alt="Robert Junkins"/><br /><sub><b>Robert Junkins</b></sub></a><br /><a href="https://github.com/gaucho-labs/leptos-hotkeys/commits?author=JustBobinAround" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/LeoniePhiline"><img src="https://avatars.githubusercontent.com/u/22329650?v=4?s=100" width="100px;" alt="LeoniePhiline"/><br /><sub><b>LeoniePhiline</b></sub></a><br /><a href="https://github.com/gaucho-labs/leptos-hotkeys/commits?author=LeoniePhiline" title="Documentation">📖</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://szabgab.com/"><img src="https://avatars.githubusercontent.com/u/48833?v=4?s=100" width="100px;" alt="Gábor Szabó"/><br /><sub><b>Gábor Szabó</b></sub></a><br /><a href="https://github.com/gaucho-labs/leptos-hotkeys/commits?author=szabgab" title="Documentation">📖</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/phillipbaird"><img src="https://avatars.githubusercontent.com/u/4003333?v=4?s=100" width="100px;" alt="Phillip Baird"/><br /><sub><b>Phillip Baird</b></sub></a><br /><a href="https://github.com/gaucho-labs/leptos-hotkeys/issues?q=author%3Aphillipbaird" title="Bug reports">🐛</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/zakstucke"><img src="https://avatars.githubusercontent.com/u/44890343?v=4?s=100" width="100px;" alt="zakstucke"/><br /><sub><b>zakstucke</b></sub></a><br /><a href="https://github.com/gaucho-labs/leptos-hotkeys/issues?q=author%3Azakstucke" title="Bug reports">🐛</a> <a href="https://github.com/gaucho-labs/leptos-hotkeys/commits?author=zakstucke" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="http://www.linkedin.com/in/ryangguk-kim"><img src="https://avatars.githubusercontent.com/u/13386712?v=4?s=100" width="100px;" alt="Ryangguk Kim"/><br /><sub><b>Ryangguk Kim</b></sub></a><br /><a href="https://github.com/gaucho-labs/leptos-hotkeys/commits?author=rkimoakbioinformatics" title="Code">💻</a></td>
    </tr>
    <tr>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/maxbergmark"><img src="https://avatars.githubusercontent.com/u/1706486?v=4?s=100" width="100px;" alt="Max Bergmark"/><br /><sub><b>Max Bergmark</b></sub></a><br /><a href="https://github.com/gaucho-labs/leptos-hotkeys/commits?author=maxbergmark" title="Code">💻</a></td>
    </tr>
  </tbody>
</table>

<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->

<!-- ALL-CONTRIBUTORS-LIST:END -->
<!-- markdownlint-disable -->

<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->

<!-- ALL-CONTRIBUTORS-LIST:END -->
