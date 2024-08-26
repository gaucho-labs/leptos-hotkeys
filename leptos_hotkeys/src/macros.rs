#[macro_export]
macro_rules! scopes {
    () => {
        {
            let mut set = std::collections::BTreeSet::<String>::new();
            set.insert("*".to_string());
            set
        }
    };

    ($($lit:literal),+ $(,)?) => {
        {
            let mut temp_set = std::collections::BTreeSet::<String>::new();
            temp_set.insert("*".to_string());
            $(
                temp_set.insert($lit.to_string());
            )+
            temp_set
        }
    };

    ($($expr:expr),+ $(,)?) => {
        {
            let mut temp_set = std::collections::BTreeSet::<String>::new();
            temp_set.insert("*".to_string());
            $(
                temp_set.insert($expr);
            )+
            temp_set
        }
    };
}

#[macro_export]
macro_rules! use_hotkeys {
    (($key_combo:literal) => $($code:tt)*) => {
        {
            $crate::use_hotkeys_scoped(
                $key_combo.to_string(),
                ::leptos::Callback::new(
                    $($code)*
                ),
                vec!["*".to_string()]
            );
        }
    };

    (($key_combo:expr) => $($code:tt)*) => {
        {
            use_hotkeys_scoped(
                $key_combo.to_string(),
                ::leptos::Callback::new(
                    $($code)*
                ),
                vec!["*".to_string()]
            );
        }
    };

    (($key_combo:expr $(, $scopes:literal)*) => $($code:tt)*) => {
        {
            $crate::use_hotkeys_scoped(
                $key_combo.to_string(),
                ::leptos::Callback::new(
                    $($code)*
                ),
                vec![$($scopes.to_string(),)*]
            );
        }
    };

    (($key_combo:literal $(, $scopes:literal)*) => $($code:tt)*) => {
        {
            $crate::use_hotkeys_scoped(
                $key_combo.to_string(),
                ::leptos::Callback::new(
                    $($code)*
                ),
                vec![$($scopes.to_string(),)*]
            );
        }
    };

    (($key_combo:literal $(, $scopes:expr)*) => $($code:tt)*) => {
        {
            $crate::use_hotkeys_scoped(
                $key_combo.to_string(),
                ::leptos::Callback::new(
                    $($code)*
                ),
                vec![$($scopes.to_string(),)*]
            );
        }
    };

    (($key_combo:expr $(, $scopes:expr)*) => $($code:tt)*) => {
        {
            $crate::use_hotkeys_scoped(
                $key_combo.to_string(),
                ::leptos::Callback::new(
                    $($code)*
                ),
                vec![$($scopes.to_string(),)*]
            );
        }
    };
}

#[macro_export]
macro_rules! use_hotkeys_ref {
    (($node_ref:expr, $key_combo:literal) => $($code:tt)*) => {
        {
            $crate::use_hotkeys_ref(
                $node_ref,
                $key_combo.to_string(),
                ::leptos::Callback::new(
                    $($code)*
                ),
                vec!["*".to_string()]
            )
        }
    };

    (($node_ref:expr, $key_combo:expr) => $($code:tt)*) => {
        {
            $crate::use_hotkeys_ref(
                $node_ref,
                $key_combo.to_string(),
                ::leptos::Callback::new(
                    $($code)*
                ),
                vec!["*".to_string()]
            )
        }
    };

    (($node_ref:expr, $key_combo:expr $(, $scopes:literal)*) => $($code:tt)*) => {
        {
            $crate::use_hotkeys_ref(
                $node_ref,
                $key_combo.to_string(),
                ::leptos::Callback::new(
                    $($code)*
                ),
                vec![$($scopes.to_string(),)*]
            )
        }
    };

    (($node_ref:expr, $key_combo:literal $(, $scopes:literal)*) => $($code:tt)*) => {
        {
            $crate::use_hotkeys_ref(
                $node_ref,
                $key_combo.to_string(),
                ::leptos::Callback::new(
                    $($code)*
                ),
                vec![$($scopes.to_string(),)*]
            )
        }
    };

    (($key_combo:literal $(, $scopes:expr)*) => $($code:tt)*) => {
        {
            $crate::use_hotkeys_ref(
                $key_combo.to_string(),
                ::leptos::Callback::new(
                    $($code)*
                ),
                vec![$($scopes.to_string(),)*]
            )
        }
    };

    (($key_combo:expr $(, $scopes:expr)*) => $($code:tt)*) => {
        {
            $crate::use_hotkeys_ref(
                $key_combo.to_string(),
                ::leptos::Callback::new(
                    $($code)*
                ),
                vec![$($scopes.to_string(),)*]
            )
        }
    };
}
