use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident, Token, bracketed, ItemFn};
use syn::punctuated::Punctuated;
use syn::parse::{Parse, ParseStream, Result};

struct WrapperInput {
    wrapper_name: Ident,
    _comma: Token![,],
    _bracket_token: syn::token::Bracket,
    variants: Punctuated<Ident, Token![,]>,
    _comma2: Token![,],
    functions: Punctuated<ItemFn, Token![,]>,
}

impl Parse for WrapperInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let wrapper_name = input.parse()?;
        let _comma: Token![,] = input.parse()?;
        let content;
        let _bracket_token = bracketed!(content in input);
        let variants = Punctuated::parse_terminated(&content)?;
        let _comma2: Token![,] = input.parse()?;
        let functions = Punctuated::parse_terminated(input)?;
        Ok(WrapperInput { wrapper_name, _comma, _bracket_token, variants, _comma2, functions })
    }
}

#[proc_macro]
pub fn define_wrapper(input: TokenStream) -> TokenStream {
    let WrapperInput { wrapper_name, variants, functions, .. } = parse_macro_input!(input as WrapperInput);
    
    let enum_variants = variants.iter().map(|v| quote! { #v(#v) });

    // generate the functions to be implemented
    let method_impls = functions.iter().map(|func| {
        let sig = &func.sig;
        let name = &sig.ident;
        let params = &sig.inputs;
        // let return_type = &sig.output;

        // get all the call params for the function
        let call_params: Vec<_> = params.iter().skip(1).map(|p| {
            if let syn::FnArg::Typed(pat) = p {
                &pat.pat
            } else {
                panic!("Unexpected self argument");
            }
        }).collect();
        
        // loop through all the variants and get each ariance to call their function with the params 
        let match_arms = variants.iter().map(|variant| {
            let call_params_clone = call_params.clone(); // Clone here to avoid move
            quote! {
                Self::#variant(inner) => inner.#name(#(#call_params_clone),*).await,
            }
        });
        
        quote! {
            pub async #sig {
                match self {
                    #(#match_arms)*
                }
            }
        }
    });
    
    // construct the enum and methods associated with the enum
    let output = quote! {
        pub enum #wrapper_name {
            #(#enum_variants,)*
        }
        
        impl #wrapper_name {
            #(#method_impls)*
        }
    };
    
    TokenStream::from(output)
}
