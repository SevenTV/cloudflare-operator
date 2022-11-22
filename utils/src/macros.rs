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

#[macro_export]
macro_rules! vec_type_to_primitive {
    ($vec:ident, $builder:ident) => {{
        let mut builder = $builder;
        for (index, t) in $vec.iter().enumerate() {
            t.to_owned()
                .to_primitive(builder.reborrow().get(index as u32));
        }
    }};
}

#[macro_export]
macro_rules! vec_type_from_primitive {
    ($reader:ident, $t:ident) => {{
        let mut vec = Vec::new();
        for i in 0..$reader.len() {
            vec.push($t::from_primitive($reader.get(i))?);
        }

        vec
    }};
}
