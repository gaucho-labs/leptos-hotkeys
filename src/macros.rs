#[macro_export]
macro_rules! scopes {
    () => {
        {
            let mut set = HashSet::<String>::new();
            set.insert("*".to_string());
            set
        }
    };

    ($($lit:literal),+ $(,)?) => {
        {
            let mut temp_set = HashSet::<String>::new();
            temp_set.insert("*".to_string());
            $(
                temp_set.insert($lit.to_string());
            )+
            temp_set
        }
    };

    ($($expr:expr),+ $(,)?) => {
        {
            let mut temp_set = HashSet::<String>::new();
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
            use_hotkeys_scoped(
                $key_combo.to_string(),
                Callback::new(
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
                Callback::new(
                $($code)*
                ),
                vec!["*".to_string()]
            );
        }
    };

    (($key_combo:expr $(, $scopes:literal)*) => $($code:tt)*) => {
        {
            use_hotkeys_scoped(
                $key_combo.to_string(),
                Callback::new(
                    $($code)*
                ),
                vec![$($scopes.to_string(),)*]
            );
        }
    };

    (($key_combo:literal $(, $scopes:literal)*) => $($code:tt)*) => {
        {
            use_hotkeys_scoped(
                $key_combo.to_string(),
                Callback::new(
                    $($code)*
                ),
                vec![$($scopes.to_string(),)*]
            );
        }
    };

    (($key_combo:literal $(, $scopes:expr)*) => $($code:tt)*) => {
        {
            use_hotkeys_scoped(
                $key_combo.to_string(),
                Callback::new(
                    $($code)*
                ),
                vec![$($scopes.to_string(),)*]
            );
        }
    };

    (($key_combo:expr $(, $scopes:expr)*) => $($code:tt)*) => {
        {
            use_hotkeys_scoped(
                $key_combo.to_string(),
                Callback::new(
                    $($code)*
                ),
                vec![$($scopes.to_string(),)*]
            );
        }
    };

}

#[macro_export]
macro_rules! use_hotkeys_ref {

    (($key_combo:literal) => $($code:tt)*) => {
        {
            use_hotkeys_ref_scoped(
                $key_combo.to_string(),
                Callback::new(
                $($code)*
                ),
                vec!["*".to_string()]
            )
        }
    };

    (($key_combo:expr) => $($code:tt)*) => {
        {
            use_hotkeys_ref_scoped(
                $key_combo.to_string(),
                Callback::new(
                $($code)*
                ),
                vec!["*".to_string()]
            )
        }
    };

    (($key_combo:expr $(, $scopes:literal)*) => $($code:tt)*) => {
        {
            use_hotkeys_ref_scoped(
                $key_combo.to_string(),
                Callback::new(
                    $($code)*
                ),
                vec![$($scopes.to_string(),)*]
            )
        }
    };

    (($key_combo:literal $(, $scopes:literal)*) => $($code:tt)*) => {
        {
            use_hotkeys_ref_scoped(
                $key_combo.to_string(),
                Callback::new(
                    $($code)*
                ),
                vec![$($scopes.to_string(),)*]
            )
        }
    };

    (($key_combo:literal $(, $scopes:expr)*) => $($code:tt)*) => {
        {
            use_hotkeys_ref_scoped(
                $key_combo.to_string(),
                Callback::new(
                    $($code)*
                ),
                vec![$($scopes.to_string(),)*]
            )
        }
    };

    (($key_combo:expr $(, $scopes:expr)*) => $($code:tt)*) => {
        {
            use_hotkeys_ref_scoped(
                $key_combo.to_string(),
                Callback::new(
                    $($code)*
                ),
                vec![$($scopes.to_string(),)*]
            )
        }
    };

}
