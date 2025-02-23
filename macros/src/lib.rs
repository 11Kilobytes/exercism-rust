#[macro_export]
macro_rules! hashmap {
    ( $($x:expr => $y:expr,)+ ) => { {
        use crate::hashmap;
        hashmap!($($x => $y),+)
    }
    };
    ( $($x:expr => $y:expr),* ) => {
        {
            use ::std::collections::HashMap;
            let mut temp_map = HashMap::new();
            $(
                temp_map.insert($x, $y);
            )*
            temp_map
        }
    };
}

