macro_rules! items {
    ($($item:item)*) => ($($item)*);
}

macro_rules! trait_alias {
    ($v:vis $name:ident = $($base:tt)+) => {
        $crate::utils::macros::items! {
            $v trait $name: $($base)+ { }
            impl<T: $($base)+> $name for T { }
        }
    };
}

pub(crate) use {trait_alias, items};
