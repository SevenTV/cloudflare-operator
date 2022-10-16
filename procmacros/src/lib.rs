extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn generated_mod(input: TokenStream) -> TokenStream {
    let binding = input.to_string();
    let vec = binding.split(' ').collect::<Vec<&str>>();

    let vis = vec[0].to_string();
    let name = vec[1].to_string();
    let file = vec[2].replace('"', "");

    // get out dir
    let out_dir = std::env::var("OUT_DIR").unwrap();

    let path = format!("#[allow(dead_code)]\n#[path = \"{}/{}\"]", out_dir, file);

    let q = format!("{}\n{} mod {};", path, vis, name);

    q.parse().unwrap()
}
