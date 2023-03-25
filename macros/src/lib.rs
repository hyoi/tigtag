use proc_macro::TokenStream;
use syn::*;
use quote::*;

//enum MyState のバリアントすべてを対象に３種類の定数を生成する
//（ENTER_VARIANT、UPDATE_VARIANT、ENTER_VARIANT）
#[proc_macro_derive( MyConstState )]
pub fn derive( input: TokenStream ) -> TokenStream
{   let ast = parse_macro_input!( input as DeriveInput );
    let type_name = ast.ident;

    let mut variant = Vec::new();
    let mut enter   = Vec::new();
    let mut update  = Vec::new();
    let mut exit    = Vec::new();
    match ast.data
    {   Data::Enum( x ) =>
            x.variants.into_iter().for_each
            (   | x |
                {   let upper = x.ident.to_string().to_uppercase();

                    variant.push( x.ident );
                    enter  .push( format_ident!( "ENTER_{}" , upper ) );
                    update .push( format_ident!( "UPDATE_{}", upper ) );
                    exit   .push( format_ident!( "EXIT_{}"  , upper ) );
                }
            ),
        _ =>
            panic!( "Not Data::Enum." ),
    }

    let expand = quote!
    {   #(  pub const #enter : OnEnter <#type_name> = OnEnter  ( #type_name::#variant );
            pub const #update: OnUpdate<#type_name> = OnUpdate ( #type_name::#variant );
            pub const #exit  : OnExit  <#type_name> = OnExit   ( #type_name::#variant );
        )*
    };
    expand.into()
}