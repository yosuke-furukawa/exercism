#[macro_export]
macro_rules! hashmap {
    () => {
        {
            let mut hashmap = ::std::collections::HashMap::new();
            hashmap
        }
    };
    ($x:expr => $y:expr) => {
        {
            let mut hashmap = ::std::collections::HashMap::new();
            hashmap.insert($x, $y);
            hashmap
        }
    };
    ($($x:expr => $y:expr,)*) => {
        {
            let mut hashmap = ::std::collections::HashMap::new();
            $(
                hashmap.insert($x, $y);
            )*
            hashmap
        }
    };
    ($($x:expr => $y:expr),*) => {
        {
            let mut hashmap = ::std::collections::HashMap::new();
            $(
                hashmap.insert($x, $y);
            )*
            hashmap
        }
    };
}
