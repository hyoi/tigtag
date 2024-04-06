//import external modules
use proc_macro2::TokenStream;
use syn::*;
use quote::*;

//#[derive( MyState )]
pub fn derive_mystate( input: TokenStream ) -> TokenStream
{   //入力を分解する
    //let ast = parse_macro_input!( input as DeriveInput );
    let ast: DeriveInput = syn::parse2( input ).unwrap();

    //データから識別子(名前)を抽出する
    let enum_type = ast.ident;
    let mut enum_variant = Vec::new();
    let mut is_variant = Vec::new();

    if let Data::Enum( my_enum ) = ast.data
    {   for my_variant in my_enum.variants.into_iter()
        {   {   let lower_ident = my_variant.ident.to_string().to_lowercase();
                enum_variant.push( my_variant.ident );
                is_variant.push( format_ident!( "is_{}", lower_ident ) );
            }
        }
    }
    else
    {   panic!( "Applicable to Enum only." )
    }

    //文字列を作成して出力する
    quote!
    {   //MyStateの遷移に使うTrait境界
        pub trait ChangeMyState
        {   fn state( &self ) -> #enum_type;
        }

        //バリアントと同名のstructからバリアントを取得させるための仕込み
        #(  #[derive( Default )]
            pub struct #enum_variant;
            impl ChangeMyState for #enum_variant
            {   fn state( &self ) -> #enum_type { #enum_type::#enum_variant }
            }
        )*

        //同名structによって指定されたMyStateへ遷移するSystem
        pub fn change_state_to<T: Send + Sync + Default + ChangeMyState>
        (   next: Local<T>,
            mut next_state: ResMut<NextState<#enum_type>>
        )
        {   next_state.set( next.state() );
        }

        //ResourceにセットされたMyStateへ遷移するSystem
        pub fn change_state_by<T: Resource + ChangeMyState>
        (   opt_state: Option<Res<T>>,
            mut next_state: ResMut<NextState<#enum_type>>
        )
        {   let Some ( next ) = opt_state else { warn!( "opt_state is None." ); return };
            next_state.set( next.state() );
        }

        // impl MyState
        // {   pub fn is_pause( &self ) -> bool { *self == MyState::Pause }
        //         :
        //         :
        // }
        impl #enum_type
        {   #(  pub fn #is_variant ( &self ) -> bool { *self == #enum_type::#enum_variant }
            )*
        }

    }
}

//End of code.