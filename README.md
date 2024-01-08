## ♨ *leptos-hotkeys* ♨
`leptos-hotkeys` provides an easy way to manage keyboard shortcuts (hotkeys) in your Leptos application.

[Check out the demo!](https://leptos-hotkeys.vercel.app/)

## Disclosure
*this is still a work in progress*

See todo for my plan: [todo](#todo)

## Installation 
```shell
cargo install leptos_hotkeys
```

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

Then you can pair keybindings with logic:
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


## Features
- **Global hotkey support:** Define hotkeys that work anywhere in your application
- **Modifier keys:** Support for Control, Alt, [*Meta*](https://www.nba.com/stats/player/1897/career), and Shift keys
- **Custom hotkey combinations**: Define complex hotkey combinations
- **Efficient state management**: Uses a global state to track pressed keys, abstracts the hard work away!



## Contributing
Omg really? that would be amazing.


## Todo!
- [ ] add scopes 
- [ ] test coverage
- [ ] more public utility functions for ease of life

<img width="570" alt="Screen Shot 2024-01-07 at 4 13 48 PM" src="https://github.com/friendlymatthew/leptos_hotkeys/assets/38759997/f3c7b6ee-e6fd-4c0d-90be-ad26ca4e2ec6">

