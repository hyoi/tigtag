//import external modules
use proc_macro::TokenStream;
use syn::*;
use quote::*;

//#[derive( MyState )]を作る
#[proc_macro_derive( MyState )]
pub fn derive( input: TokenStream ) -> TokenStream
{   //入力を分解する
    let ast = parse_macro_input!( input as DeriveInput );
    let enum_type = ast.ident;

    //バリアントの名前を保存する
    let mut variant = Vec::new();
    match ast.data
    {   Data::Enum( my_enum ) =>
        {   for my_variant in my_enum.variants.into_iter()
            {   {   variant.push( my_variant.ident );
                }
            }
        }
        _ => panic!( "Only Enum can be applied." )
    }

    //文字列を作成して出力する
    let expand = quote!
    {   //MyStateの遷移に使うTrait境界
        pub trait ChangeMyState { fn state( &self ) -> #enum_type; }

        //バリアントと同名のstructからバリアントを取得できるように仕込む
        #(  #[derive( Default )] pub struct #variant;
            impl ChangeMyState for #variant
            {   fn state( &self ) -> #enum_type { #enum_type::#variant }
            }
        )*

        //型(struct)によって指定されたMyStateへ遷移する
        pub fn change_state_to<T: Send + Sync + Default + ChangeMyState>
        (   next: Local<T>,
            mut next_state: ResMut<NextState< #enum_type >>
        )
        {   next_state.set( next.state() );
        }

        //ResourceにセットされたMyStateへ遷移する
        pub fn change_state_by<T: Resource + ChangeMyState>
        (   opt_state: Option<Res<T>>,
            mut next_state: ResMut<NextState< #enum_type >>
        )
        {   let Some ( next ) = opt_state else { warn!( "opt_state is None." ); return };
            next_state.set( next.state() );
        }
    };
    expand.into()
}

//End of code.