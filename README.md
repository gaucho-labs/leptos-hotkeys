# [_leptos-hotkeys_](https://github.com/friendlymatthew/leptos-hotkeys)

Declaratively create and pair keybindings with callbacks for Leptos applications.

[![crates](https://img.shields.io/badge/📦_crates-0.1.6-%20green)](https://crates.io/crates/leptos_hotkeys)
[![discord](https://img.shields.io/badge/Join-Discord-%235865F2.svg)](https://discord.gg/XhVbKk38ux)

<a href="https://github.com/friendlymatthew/leptos-hotkeys">
    <img width="570" height="211"
         alt="A person playing a burning piano at a sandy beach under a cloudy sky"
         src="https://github.com/friendlymatthew/leptos_hotkeys/assets/38759997/f3c7b6ee-e6fd-4c0d-90be-ad26ca4e2ec6" />
</a>

> [!NOTE]
>
> This library is ready for use.
> If you're curious about updates, please read the [CHANGELOG](#changelog).

## Live example

Curious to see how it works? [See the demo!](https://leptos-hotkeys.vercel.app/)

To get started, follow the [Quick Start](#quick-start) section. It's worth the read!

## Features

### `use_hotkeys!` Macro

For simplicity and ease, use the `use_hotkeys!` macro to declare global and scoped hotkeys.

We brought some js idioms while maintaining the leptos look.
[Learn more about the macro](#macro-api).

If you prefer writing out your callbacks the leptos way, we also have non-macro hotkeys. [Learn more about trad hotkeys](#trad-hotkeys).

### Global Hotkeys

This example creates two global hotkeys: `W` and `S`.

> [!TIP]
>
> For more information about how to write your keybindings, check out [Key Grammar](#keybinding-grammar).
>
> _Note: the `*` symbol is reserved for the global scope_

```rust
use leptos_hotkeys::prelude::*;

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
>
> Assign hotkeys specific to individual sections without collisions using scopes. Use functions in `HotkeysContext` for scope management.
>
> For more information about how to write your keybindings, check out [Key Grammar](#keybinding-grammar).
>
> _Note: scopes are case-insensitive. That means `wef_scope` and `WEf_sCoPe` are considered the same scope._

```rust
use leptos_hotkeys::prelude::*;

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
>
> Embed a hotkey with an html element and the hotkey will only fire if the element is focused and the scope is enabled.

```rust
use leptos_hotkeys::prelude::*;

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
>
> `leptos-hotkeys` supports both client-side rendered and server-side rendered applications. Depending on your application, make sure to include the `csr` or `hydrate` feature flag.

For client side rendered:

```toml
leptos_hotkeys = { path = "0.1.6", features = ["csr"] }
```

For server side rendered:

```toml
leptos_hotkeys = { version = "0.1.6", features = ["hydrate"] }
```

We also other feature flags that enhance developer experience, to learn more read [feature-flags](#feature-flags).

### `provide_hotkeys_context()`

Call `provide_hotkeys_context()` in the `App()` component. This will provide the `HotkeysContext` for the current reactive node and all of its descendents. 
This function takes three parameters, the `node_ref`, a flag to disable blur events (*you should go `false`*), and a list of `initially_active_scopes`.

> [!NOTE]
>
> `provide_hotkeys_context()` returns a `HotkeysContext`. To learn more, see [HotkeysContext](#hotkeyscontext).

```rust
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    
    let main_ref = create_node_ref::<html::Main>();
    provide_hotkeys_context(main_ref, false, scopes!()); 
    
    view! {
        <HotkeysProvider>
            <Router>
                <main _ref=main_ref>  // <-- attach main ref here!
                    <Routes>
                        <Route path="/" view=HomePage/>
                        <Route path="/:else" view=ErrorPage/>
                    </Routes>
                </main>
            </Router>
        </HotkeysProvider>
    }
}
```

### Initialize scopes

If you're using [scopes](#scoped-hotkeys), you can initialize with a specific scope.

```rust
use leptos_hotkeys::scopes;

view! {
    <HotkeysProvider
        initially_active_scopes=scopes!("some_scope_id")
    >
        <Router>
            //... routes
        </Router>
    </HotkeysProvider>
}
```

Thats it! Start creating [hotkeys](#features)!

### Keybinding Grammar

`leptos_hotkeys` matches key values from [KeyboardEvent's](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/key) `key` property.

For reference, here's a list of [all key values for keyboard events](https://developer.mozilla.org/en-US/docs/Web/API/UI_Events/Keyboard_event_key_values).

You can bind multiple hotkeys to a callback. For example:

```txt
"G+R,meta+O,control+k"
```

The above example creates three hotkeys: `G+R`, [Meta](https://www.basketball-reference.com/players/a/artesro01.html)`+O`, and `Control+K`.
The `+` symbol is used to create a combo hotkey. A combo hotkey is a keybinding requiring more than one key press.
Note that keys are case-agnostic and whitespace-agnostic. You use the `,` as a delimiter in a sequence of multiple hotkeys.

## Macro API

We wanted to strip the verbosity that comes with `str` and `String` type handling.<br>
We kept leptos best practices in mind, keeping the `move |_|` idiom in our macro.

### `use_hotkeys!()`

Here is a general look at the macro:

```rust
use leptos_hotkeys::prelude::*;

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
use leptos_hotkeys::prelude::*;

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
use leptos_hotkeys::prelude::*;

view! {
    <HotkeysProvider
        initially_active_scopes=scopes!("scope_a", "settings_scope");
    >
        // pages here...
    </HotkeysProvider>
}
```

## Feature Flags

In addition to the `CSR` and `Hydrate` feature flags, we want to improve developer experience by introducing the `debug` flag which adds logging to your console. It logs the current pressed key values, hotkeys fires, and scopes toggling.

Just simply:

```toml
leptos_hotkeys = { path = "0.1.6", features = ["hydrate", "debug"] }
```

## API

### `<HotkeysProvider />`

| Prop Name                 | Type              | Default Value                 | Description                                                                                                                                                                          |
| ------------------------- | ----------------- | ----------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `allow_blur_event`        | `bool`            | `false`                       | Determines if the component should reset `pressed_keys` when a blur event occurs on the window. This is useful for resetting the state when the user navigates away from the window. |
| `initially_active_scopes` | `HashSet<String>` | `scopes!("*")` (Global State) | Specifies the set of scopes that are active when the component mounts. Useful for initializing the component with a predefined set of active hotkey scopes.                          |

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
            set_count.update(|count| {
            *count += 1;
            })
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

<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->

<!-- ALL-CONTRIBUTORS-LIST:END -->
