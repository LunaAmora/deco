use proc_macro::TokenStream;
use quote::quote;
use proc_macro2::{TokenStream as TS2, Ident, Span};
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn deco(attr: TokenStream, item: TokenStream) -> TokenStream {
    if attr.is_empty() {
        let ast = TS2::from(item);
        TokenStream::from(quote! {
            decorators::make_decorator! {
                #ast
            }
        })
    } else {
        let ast = parse_macro_input!(item as ItemFn);
        let deco = TS2::from(attr);
        let ident = Ident::new(&format!("___{}_{}", ast.sig.ident, deco.to_string()), Span::call_site());
        
        TokenStream::from(quote! {
            decorators::decorator! {
                @ #deco #ident
                #ast
            }
        })
    }
}