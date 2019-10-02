extern crate proc_macro;
use proc_macro::TokenStream;
use syn;
use syn::{parse_macro_input};
use quote::{quote};



#[proc_macro_attribute]
pub fn traced_test(attr_args: TokenStream, input: TokenStream) -> TokenStream {
    if !attr_args.is_empty() {
        /*
         * We take no attributes, panic?
         */
    }
    let _o = input.clone();

    let syn::ItemFn { attrs, vis, sig, block } =
        parse_macro_input!(input as syn::ItemFn);

    let attrs = attrs.iter().map(|attr| quote!{#attr});
    let stmts = block.stmts.iter().map(|stmt| quote!{#stmt});

    let output = quote!{
        #[test]
        #[instrument]
        #(#attrs)*
        #vis #sig
        {
            let fmt_subscriber = tracing_subscriber::FmtSubscriber::new();
            tracing::subscriber::with_default(fmt_subscriber, || {
                #(#stmts)*
            });
        }
    };

    TokenStream::from(output)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() { assert!(true); }
}
