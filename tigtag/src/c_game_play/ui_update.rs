use super::*;

//プラグインの設定
pub struct UiUpdate;
impl Plugin for UiUpdate
{   fn build( &self, app: &mut App )
    {   //常時表示を更新するSystem
        app
        .add_plugins( FrameTimeDiagnosticsPlugin )   //FPSプラグイン
        .add_systems
        (   Update,
            (   ui_update::header_left,   //UI表示更新(STAGE)
                ui_update::header_center, //UI表示更新(SCORE)
                ui_update::header_right,  //UI表示更新(Hi-SCORE)
                ui_update::footer_left,   //UI表示更新(FPS)
            )
        )
        ;
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//UIの表示を更新する(FPS)
fn footer_left
(   mut q: Query<&mut Text, With<FooterLeft>>,
    diag: Res<DiagnosticsStore>,
    o_record: Option<Res<Record>>,
)
{   if let Ok( mut ui ) = q.get_single_mut()
    {   let fps_avr = diag.get( FrameTimeDiagnosticsPlugin::FPS ).map_or
        (   NA2_2.to_string(),
            | fps | fps.average().map_or( NA2_2.to_string(), | avg | format!( "{avg:02.02}" ) )
        );
        ui.sections[ 1 ].value = fps_avr;

        let stage_hi_score = o_record.map_or
        (   NA2_5.to_string(),
            | record | format!( "{:02}-{:05}", record.demo.stage, record.demo.hi_score )
        );
        ui.sections[ 3 ].value = stage_hi_score;
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
        if misc::DEBUG()
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