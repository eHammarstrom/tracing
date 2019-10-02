extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn traced_test(attr_args: TokenStream, input: TokenStream) -> TokenStream {
    let fun = parse_macro_input!(input as syn::ItemFn);
    let num_args = attr_args.clone().into_iter().count();

    /*
     * AFAICT we do not have type information to reason about
     * the Trait of the Struct that is passed to us.
     * We cannot verify that it is of Trait `Subscriber`,
     * it will be type-checked later when calling tracing functions.
     */
    type SubscriberStruct = syn::Ident;

    let subscriber: SubscriberStruct = if num_args == 1 {
        parse_macro_input!(attr_args as SubscriberStruct)
    } else {
        /*
         * Must specify a concrete Subscriber, throw compile time error.
         * Use function's signature span, because attribute parse failed.
         */
        return TokenStream::from(
            syn::Error::new(
                fun.sig.ident.span(),
                "Must specify a concrete subscriber attribute: \
                 #[traced_test(MySubscriber)]",
            )
            .to_compile_error(),
        );
    };

    let syn::ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = fun;

    let attrs = attrs.iter().map(|attr| quote! {#attr});
    let stmts = block.stmts.iter().map(|stmt| quote! {#stmt});

    let output = quote! {
        #[test]
        #[instrument]
        #(#attrs)*
        #vis #sig
        {
            let sub: #subscriber = #subscriber::new();
            tracing::subscriber::with_default(sub, || {
                #(#stmts)*
            });
        }
    };

    TokenStream::from(output)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert!(true);
    }
}
