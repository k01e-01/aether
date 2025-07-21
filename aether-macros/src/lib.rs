use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn default_endpoint(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &input_fn.sig.ident;

    let gen_endpoint = quote! {
        #input_fn

        #[tokio::main]
        async fn main() -> Result<(), lambda_http::Error> {
            lambda_http::tracing::init_default_subscriber();
            lambda_http::run(lambda_http::service_fn(#fn_name)).await?;
            Ok(())
        }
    };

    gen_endpoint.into()
}
