#[macro_export(local_inner_macros)]
macro_rules! items {
    ($($item:item)*) => ($($item)*);
}

#[macro_export(local_inner_macros)]
macro_rules! trait_alias {
    ($v:vis $name:ident = $($base:tt)+) => {
        items! {
            $v trait $name: $($base)+ { }
            impl<T: $($base)+> $name for T { }
        }
    };
}
