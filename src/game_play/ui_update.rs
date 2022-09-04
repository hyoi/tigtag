use super::*;

//import external modules
use bevy::diagnostic::*;

//プラグインの設定
pub struct UiUpdate;
impl Plugin for UiUpdate
{   fn build( &self, app: &mut App )
    {   //常時表示を更新するSystem
        app
        .add_plugin( FrameTimeDiagnosticsPlugin )   //FPSプラグイン
        .add_system( ui_update::header_left   )     //UI表示更新(STAGE)
        .add_system( ui_update::header_center )     //UI表示更新(SCORE)
        .add_system( ui_update::header_right  )     //UI表示更新(Hi-SCORE)
        .add_system( ui_update::footer_left   )     //UI表示更新(FPS)
        ;
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//UIの表示を更新する(FPS)
fn footer_left
(   mut q: Query<&mut Text, With<FooterLeft>>,
    diag: Res<Diagnostics>,
)
{   if let Ok( mut ui ) = q.get_single_mut()
    {   let fps_avr = diag.get( FrameTimeDiagnosticsPlugin::FPS ).map_or
        (   NA2_2.to_string(),
            | fps | fps.average().map_or( NA2_2.to_string(), | avg | format!( "{:02.02}", avg ) )
        );
        ui.sections[ 1 ].value = fps_avr;
    }
}

//UIの表示を更新する(STAGE)
fn header_left
(   mut q: Query<&mut Text, With<HeaderLeft>>,
    o_record: Option<Res<Record>>,
)
{   if let Ok( mut ui ) = q.get_single_mut()
    {   let x = o_record.map_or( NA2.to_string(), | record | format!( "{:02}", record.stage ) );
        ui.sections[ 1 ].value = x;
    }
}

//UIの表示を更新する(SCORE)
#[allow( unused_variables )]
fn header_center
(   mut q: Query<&mut Text, With<HeaderCenter>>,
    o_record: Option<Res<Record>>,
    o_map   : Option<Res<Map>>, //デバッグ用
)
{   if let Ok( mut ui ) = q.get_single_mut()
    {   let x = o_record.map_or( NA5.to_string(), | record | format!( "{:05}", record.score ) );
        ui.sections[ 1 ].value = x;

        //デバッグ時、残ドット数を表示する
        #[cfg( debug_assertions )]
        {   let x = o_map.map_or( NA3.to_string(), | map | format!( "/{:03}", map.remaining_dots ) );
            ui.sections[ 2 ].value = x;
        }
    }
}

//UIの表示を更新する(HI-SCORE)
fn header_right
(   mut q: Query<&mut Text, With<HeaderRight>>,
    o_record: Option<Res<Record>>,
)
{   if let Ok( mut ui ) = q.get_single_mut()
    {   let x = o_record.map_or( NA5.to_string(), | record | format!( "{:05}", record.hi_score ) );
        ui.sections[ 1 ].value = x;
    }
}

//End of code.