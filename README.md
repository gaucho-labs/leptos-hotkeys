<br />

<h1>
    <a href="https://github.com/friendlymatthew/leptos-hotkeys">
    <em>leptos-hotkeys</em>    
</a>
</h1>

[![crates](https://img.shields.io/badge/📦_crates-install-%20green)](https://crates.io/crates/leptos_hotkeys)
[![version](https://img.shields.io/badge/version-0.1.0-purple)](https://materialize.com/s/chat)

<a href="https://github.com/friendlymatthew/leptos-hotkeys">
    <img width="570" alt="Screen Shot 2024-01-07 at 4 13 48 PM" src="https://github.com/friendlymatthew/leptos_hotkeys/assets/38759997/f3c7b6ee-e6fd-4c0d-90be-ad26ca4e2ec6">
</a>

leptos_hotkeys is a library designed to simplify the process of integrating keyboard shortcuts into leptos web applications.

## Live example 
Curious to see how it works? [See the demo!](https://leptos-hotkeys.vercel.app/)

To get started, follow the [Quick Start](#quick-start) section.
## Features
### Global
The quickest way to create a keybinding is to declare a global hotkey.
```rust
use leptos_hotkeys::{use_hotkeys};

#[component]
fn Component() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    
    use_hotkeys(
        "F", // the F key
        Callback::new(move |_| {
            set_count.update(|count| {
                *count += 1; 
            }) 
        })
    );

    view! {
        <p>
            Press 'F' to pay respect. 
        
            {count} times    
        </p>
    } 
}
```

### Scopes
Assign hotkeys specific to individual sections without collisions using `use_hotkeys_scoped`. 

This ensures that keybindings are active only in their designated scope. Dynamically activate or deactivate entire groups of hotkeys using `toggle_scope`, `enable_scope`, `disable_scope`.
```rust
use leptos_hotkeys::{
    use_hotkeys_scoped,
    use_hotkeys_context,
    HotkeysContext
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

### Focus trapping
Pair a keybinding with a `html` element. The hotkey will trigger if the element is focused.  
```rust
use leptos_hotkeys::{
    use_hotkeys_ref 
};

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


## Quick Start

### Installation
```shell
cargo install leptos_hotkeys
```

### Hotkey Provider
Wrap your project with `<HotkeysProvider />`:
```html
view! {
    <HotkeysProvider>
        <Router>
            <Routes>
                <Route path="/" view=HomePage/>
                <Route path="/:else" view=ErrorPage/>
            </Routes>
        </Router>
    </HotkeysProvider>
}
```

#### Initialize scopes
If you're using [scopes](#scopes), you can initialize with a specific scope.
```html
view! {
    <HotkeysProvider
        initially_active_scopes=scopes!("some_scope_id") 
    >
        <Router>
            <Routes>
                <Route path="/" view=HomePage/>
                <Route path="/:else" view=ErrorPage/>
            </Routes>
        </Router>
    </HotkeysProvider>
}
```

### Pair keybindings
Then you can [pair keybindings](#features):
```rust
// in some component (e.g. <HomePage />)
use leptos_hotkeys::use_hotkeys;

let (count, set_count) = create_signal(0);

use_hotkeys(
    "ctrl+k",
    Callback::new(move |_| {
        set_count.update(|count| {
            *count += 1;
        });
    })
);
```

That's it!


## API
// todo!

