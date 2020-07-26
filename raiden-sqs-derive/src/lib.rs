use proc_macro::TokenStream;
use quote::*;
use syn::*;

#[proc_macro_derive(RaidenSqs, attributes(sqs))]
pub fn derive_raiden_sqs(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;
    let attrs = input.attrs;
    let queue_name = find_queue_name(&attrs).unwrap_or_else(|| struct_name.to_string());
    let region = find_region(&attrs).unwrap_or_else(|| "us-east-1".into());
    let endpoint = find_endpoint(&attrs).unwrap_or_else(|| "".into());
    let queue_url = format!("{}/queue/{}", endpoint, queue_name);

    let expanded = quote! {
        impl #struct_name {
            pub async fn send(&self) -> Result<String, String> {
                let client = ::raiden_sqs::SqsClient::new(::raiden_sqs::Region::Custom {
                    endpoint: #endpoint.into(),
                    name: #region.into(),
                });
                let message_body = ::serde_json::to_string(&self).unwrap();
                let req = ::raiden_sqs::SendMessageRequest {
                    queue_url: #queue_url.into(),
                    message_body,
                    ..Default::default()
                };
                match client.send_message(req).await {
                    Ok(res) => {
                        Ok(res.message_id.unwrap()) // TODO
                    }
                    Err(e) => {
                        Err(format!("{:?}", e)) // TODO
                    }
                }
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

fn find_eq_string_from(attr: &syn::Attribute, name: &str) -> Option<String> {
    let mut tokens = match attr.tokens.clone().into_iter().next() {
        Some(proc_macro2::TokenTree::Group(g)) => g.stream().into_iter(),
        _ => return None,
    };

    // #[sqs(name )]
    match tokens.next() {
        Some(proc_macro2::TokenTree::Ident(ref ident)) if *ident == name => {}
        _ => return None,
    };

    // #[sqs(name = )]
    match tokens.next() {
        Some(proc_macro2::TokenTree::Punct(ref punct)) if punct.as_char() == '=' => {}
        _ => return None,
    };

    // #[sqs(name = value)]
    let lit = match tokens.next() {
        Some(proc_macro2::TokenTree::Literal(lit)) => syn::Lit::new(lit),
        _ => return None,
    };

    match &lit {
        syn::Lit::Str(lit_str) => {
            let value = lit_str.value();
            if value.trim().is_empty() {
                panic!()
            };
            Some(value)
        }
        _ => None,
    }
}

fn find_queue_name(attrs: &[syn::Attribute]) -> Option<String> {
    for attr in attrs {
        if let Some(lit) = find_eq_string_from(&attr, "queue_name") {
            return Some(lit);
        }
    }
    None
}

fn find_region(attrs: &[syn::Attribute]) -> Option<String> {
    for attr in attrs {
        if let Some(lit) = find_eq_string_from(&attr, "region") {
            return Some(lit);
        }
    }
    None
}

fn find_endpoint(attrs: &[syn::Attribute]) -> Option<String> {
    for attr in attrs {
        if let Some(lit) = find_eq_string_from(&attr, "endpoint") {
            return Some(lit);
        }
    }
    None
}
