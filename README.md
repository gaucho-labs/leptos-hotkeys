# [_leptos-hotkeys_](https://github.com/gaucho-labs/leptos-hotkeys)
<!-- markdownlint-disable -->
<!-- ALL-CONTRIBUTORS-BADGE:START - Do not remove or modify this section -->
[![All Contributors](https://img.shields.io/badge/all_contributors-6-orange.svg?style=flat-square)](#contributors-)
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

> [!NOTE]
> This library is ready for use. If you're curious about updates read the [CHANGELOG](https://github.com/gaucho-labs/leptos-hotkeys/blob/main/CHANGELOG.md).

## Live example

Curious to see how it works? [See the demo](https://leptos-hotkeys.vercel.app) and its [source code](https://github.com/gaucho-labs/leptos-hotkeys/tree/main/examples/demo)!

To get started, follow the [Quick Start](#quick-start) section. It's worth the read!

## Features

### `use_hotkeys!` Macro

For simplicity and ease, use the `use_hotkeys!` macro to declare global and scoped hotkeys. We brought some js idioms while maintaining the leptos look. [Learn more about the macro](#macro-api).

If you prefer writing out your callbacks the leptos way, we also have non-macro hotkeys. [Learn more about trad hotkeys](#trad-hotkeys).

### Global Hotkeys

This example creates two global hotkeys: `W` and `S`.

> [!TIP]
> For more information about how to write your keybindings, check out [Key Grammar](#keybinding-grammar).

> [!NOTE]
> The `*` symbol is reserved for the global scope_

```rust
use leptos_hotkeys::use_hotkeys;

#[component]
pub fn SomeComponent() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    // creating a global scope for the W key
    use_hotkeys!(("w") => move |_| {
        logging::log!("w has been pressed");
        set_count.update(|c| *c += 1);
    });

    // this is also a global scope for the S key!
    use_hotkeys!(("s", "*") => move |_| {
        logging::log!("s has been pressed");
        set_count.update(|c| *c -= 1);
    });

    view! {
        <p>Current count: {count}</p>
    }
}
```

### Scoped Hotkeys

This example shows an inner and outer scope and hotkeys that switch between the scopes.

> [!TIP]
> Assign hotkeys specific to individual sections without collisions using scopes. Use functions in `HotkeysContext` for scope management. For more information about how to write your keybindings, check out [Key Grammar](#keybinding-grammar).

> [!NOTE]
> Scopes are case-insensitive. That means `my_scope` and `mY_sCoPe` are considered the same scope.

```rust
use leptos_hotkeys::{use_hotkeys, use_hotkeys_context, HotkeysContext};

#[component]
pub fn SomeComponent() -> impl IntoView {

    let HotkeysContext { enable_scope, disable_scope, .. } = use_hotkeys_context();

    // switch into the inner scope
    use_hotkeys!(("i", "outer") => move |_| {
        disable_scope("outer");
        enable_scope("inner");
    });

    // switch into the outer scope
    use_hotkeys!(("o", "inner") => move |_| {
        disable_scope("inner");
        enable_scope("outer");
    });

    view! {
        <div id="outer">
            //...some outer scope html...
            <div id="inner">
            //...some inner scope html...
            </div>
            //...some outer scope html....
        </div>
    }
}
```

### Focus trapped Hotkeys

> [!TIP]
> Embed a hotkey with an html element and the hotkey will only fire if the element is focused and the scope is enabled.

```rust
use leptos_hotkeys::use_hotkeys_ref;

#[component]
pub fn SomeComponent() -> impl IntoView {

    let p_ref = use_hotkeys_ref!(("K", "*") => move |_| {
        // some logic
    });

    view! {
        <p
            tabIndex=-1
            _ref=p_ref
        >
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

> [!NOTE]
> `leptos-hotkeys` supports both client-side rendered and server-side rendered applications.

For client side rendered:

```toml
leptos_hotkeys = "0.2.0"
```

For server side rendered:

```toml
leptos_hotkeys = { version = "0.2.0", features = ["ssr"] }
```

For client side and server side rendered:

```toml
leptos_hotkeys = "0.2.0"

[features]
ssr = ["leptos_hotkeys/ssr"]
```

We also offer other feature flags that enhance developer experience, see [features](#features).

### `provide_hotkeys_context()`

Call `provide_hotkeys_context()` in the `App()` component. This will provide the `HotkeysContext` for the current reactive node and all of its descendents. This function takes three parameters, the `node_ref`, a flag to disable blur events and a list of `initially_active_scopes`.

> [!NOTE]
> `provide_hotkeys_context()` returns a `HotkeysContext`. See [HotkeysContext](#hotkeyscontext).

```rust
use leptos_hotkeys::{provide_hotkeys_context, scopes};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let main_ref = create_node_ref::<html::Main>();
    provide_hotkeys_context(main_ref, false, scopes!());

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

### Initialize scopes

If you're using [scopes](#scoped-hotkeys), you can initialize with a specific scope.

```rust
use leptos_hotkeys::{provide_hotkeys_context, scopes};

#[component]
pub fn App() -> impl IntoView {
    let main_ref = create_node_ref::<html::Main>();
    provide_hotkeys_context(main_ref, false, scopes!("some_scope_id"));

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

### Keybinding Grammar

`leptos_hotkeys` matches key values from [KeyboardEvent's](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/key) `key` property. For reference, here's a list of [all key values for keyboard events](https://developer.mozilla.org/en-US/docs/Web/API/UI_Events/Keyboard_event_key_values).

You can bind multiple hotkeys to a callback. For example:

```txt
"G+R,meta+O,control+k"
```

The above example creates three hotkeys: <kbd>G</kbd>+<kbd>R</kbd>, <kbd>[Meta](https://www.basketball-reference.com/players/a/artesro01.html)</kbd>+<kbd>O</kbd>, and <kbd>Ctrl</kbd>+<kbd>K</kbd>. The `+` symbol is used to create a combo hotkey. A combo hotkey is a keybinding requiring more than one key press.

> [!NOTE]
> Keys are case-agnostic and whitespace-agnostic. You use the `,` as a delimiter in a sequence of multiple hotkeys.

## Macro API

We wanted to strip the verbosity that comes with `str` and `String` type handling. We kept leptos best practices in mind, keeping the `move |_|` idiom in our macro.

### `use_hotkeys!()`

Here is a general look at the macro:

```rust
use leptos_hotkeys::use_hotkeys;

use_hotkeys!(("keys", "scope") => move |_| {
    // callback logic here
});
```

For global hotkeys, you can omit the second parameter as it will implicitly add the global scope.

```rust
use_hotkeys!(("key") => move |_| {
    // callback logic here
});
```

### `use_hotkeys_ref!()`

This macro is used when you want to focus trap with a specific html element.

```rust
use leptos_hotkeys::use_hotkeys_ref;

#[component]
pub fn SomeComponent() -> impl IntoView {
    let some_ref = use_hotkeys_ref!(("key", "scope") => move |_| {
        // callback logic here
    });

    view! {
        <div tabIndex=-1 _ref=some_ref>
        </div>
    }
}
```

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

## Feature Flags

### `debug`

We want to improve developer experience by introducing the `debug` flag which adds logging to your console in CSR. It logs the current pressed key values, hotkeys fires, and scopes toggling.

Just simply:

```toml
leptos_hotkeys = { path = "0.2.0", features = ["debug"] }
```

## API

### `HotkeysContext`

| Field Name          | Type                            | Description                                                                                           |
| ------------------- | ------------------------------- | ----------------------------------------------------------------------------------------------------- |
| `pressed_keys`      | `RwSignal<HashSet<String>>`     | A reactive signal tracking the set of keys currently pressed by the user.                             |
| `active_ref_target` | `RwSignal<Option<EventTarget>>` | A reactive signal holding the currently active event target, useful for focusing events.              |
| `set_ref_target`    | `Callback<Option<EventTarget>>` | A method to update the currently active event target.                                                 |
| `active_scopes`     | `RwSignal<HashSet<String>>`     | A reactive signal tracking the set of currently active scopes, allowing for scoped hotkey management. |
| `enable_scope`      | `Callback<String>`              | A method to activate a given hotkey scope.                                                            |
| `disable_scope`     | `Callback<String>`              | A method to deactivate a given hotkey scope.                                                          |
| `toggle_scope`      | `Callback<String>`              | A method to toggle the activation state of a given hotkey scope.                                      |

### Basic Types

#### Keyboard Modifiers

| Field Name | Type   | Description                                                                                                                                                                     |
| ---------- | ------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `alt`      | `bool` | Indicates if the Alt key modifier is active (true) or not (false).                                                                                                              |
| `ctrl`     | `bool` | Indicates if the Control (Ctrl) key modifier is active (true) or not (false).                                                                                                   |
| `meta`     | `bool` | Indicates if the [Meta](https://www.basketball-reference.com/players/a/artesro01.html) (Command on macOS, Windows key on Windows) key modifier is active (true) or not (false). |
| `shift`    | `bool` | Indicates if the Shift key modifier is active (true) or not (false).                                                                                                            |

#### Hotkey

| Field Name    | Type                | Description                                                                                                                                    |
| ------------- | ------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------- |
| `modifiers`   | `KeyboardModifiers` | The set of key modifiers (Alt, Ctrl, [Meta](https://www.basketball-reference.com/players/a/artesro01.html), Shift) associated with the hotkey. |
| `keys`        | `Vec<String>`       | The list of keys that, along with any modifiers, define the hotkey.                                                                            |
| `description` | `String`            | A human-readable description of what the hotkey does. Intended for future use with scopes.                                                     |

## Trad Hotkeys

If the macro isn't to your liking, we offer three hotkeys: global, scoped, and focus trapped.

### Global: `use_hotkeys_scoped()` where scope = `*`

```rust
use leptos_hotkeys::{use_hotkeys_scoped};

#[component]
fn Component() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    use_hotkeys_scoped(
        "F", // the F key
        Callback::new(move |_| {
            set_count.update(|count| { *count += 1 })
        }),
        vec!["*"]
    );

    view! {
        <p>
        Press 'F' to pay respect.
        {count} times
        </p>
    }
}
```

### Scoped - `use_hotkeys_scoped`

```rust
use leptos_hotkeys::{
    use_hotkeys_scoped, use_hotkeys_context, HotkeysContext
};

#[component]
fn Component() -> impl IntoView {
    let hotkeys_context: HotkeysContext = use_hotkeys_context();

    let toggle = hotkeys_context.toggle_scope;
    let enable = hotkeys_context.enable_scope;
    let disable = hotkeys_context.disable_scope;

    use_hotkeys_scoped(
        "arrowup",
        Callback::new(move |_| {
            // move character up
        }),
        vec!["game_scope"]
    );

    use_hotkeys_scoped(
        "arrowdown",
        Callback::new(move |_| {
            // move character down
        }),
        vec!["game_scope"]
    );

    view! {
        <button
        // activates the 'game_scope' scope
        on:click=move |_| enable("game_scope")
        >
            Start game
        </button>

        <button
        // toggles the 'game_scope' from enabled to disabled
        on:click=move |_| toggle("game_scope")
        >
            Pause game
        </button>


        <button
            // disables the 'game_scope' scope
            on:click=move |_| disable("game_scope")
        >
            End game
        </button>
    }
}
```

### Focus trapped - `use_hotkeys_ref()`

```rust
use leptos_hotkeys::use_hotkeys_ref;

#[component]
fn Component() -> impl IntoView {
    let node_ref = use_hotkeys_ref("l", Callback::new(move |_| {
        // some logic here
    }));

    view! {
        <body>
            <div _ref=node_ref>
            // when this div is focused, the "l" hotkey will fire
            </div>
        </body>
    }
}
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
      <td align="center" valign="top" width="14.28%"><a href="http://friendlymatthew.github.io"><img src="https://avatars.githubusercontent.com/u/38759997?v=4?s=100" width="100px;" alt="Matthew Kim"/><br /><sub><b>Matthew Kim</b></sub></a><br /><a href="https://github.com/gaucho-labs/leptos-hotkeys/commits?author=friendlymatthew" title="Code">💻</a></td>
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
