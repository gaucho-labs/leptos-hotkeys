#[macro_export]
macro_rules! scopes {
    () => {
        {
        HashSet::<String>::new()
        }
    };

    ($($lit:literal),+ $(,)?) => {
        {
            let mut temp_set = HashSet::<String>::new();
            $(
                temp_set.insert($lit.to_string());
            )+
            temp_set
        }
    };

    ($($expr:expr),+ $(,)?) => {
        {
            let mut temp_set = HashSet::<String>::new();
            $(
                temp_set.insert($expr);
            )+
            temp_set
        }
    };

}
