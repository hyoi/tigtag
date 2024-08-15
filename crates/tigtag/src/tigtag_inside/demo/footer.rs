use super::*;

////////////////////////////////////////////////////////////////////////////////

//プラグインの設定
pub struct Schedule;
impl Plugin for Schedule
{   fn build( &self, app: &mut App )
    {   app
        .add_systems
        (   OnEnter ( MyState::InitGame ),
            (   add_sections_to_footerleft, //既存のフッターを改造する
            )
        )
        .add_systems
        (   Update,
            (   update_demo_record, //demo record表示の更新
            )
            .run_if( in_state( MyState::TitleDemo ) )
        )
        ;
    }
}

////////////////////////////////////////////////////////////////////////////////

//追加するセクション
const ADDITIONAL_TEXT_DEMO_RECORD: &[ MessageSect ] =
&[  ( " demo "  , ASSETS_FONT_ORBITRON_BLACK      , PIXELS_PER_GRID * 0.45, Color::TEAL   ),
    ( "##-#####", ASSETS_FONT_PRESSSTART2P_REGULAR, PIXELS_PER_GRID * 0.25, Color::SILVER ),
];
const PLACE_HOLDER_DEMO_RECORD: usize = 3; //text.sections[ 3 ]

////////////////////////////////////////////////////////////////////////////////

//フッターのUIを改造する
fn add_sections_to_footerleft
(   mut qry_text: Query<&mut Text, With<UiFooterLeft>>,
    asset_svr: Res<AssetServer>,
)
{   let Ok ( mut text ) = qry_text.get_single_mut() else { return };

    //demo record表示sectionを、既存のtext.sectionsに追加する
    let bundle = misc::text_ui( ADDITIONAL_TEXT_DEMO_RECORD, &asset_svr );
    text.sections.extend( bundle.text.sections );
}

////////////////////////////////////////////////////////////////////////////////

//UIの表示を更新する(demo record)
fn update_demo_record
(   mut qry_text: Query<&mut Text, With<UiFooterLeft>>,
    opt_record: Option<ResMut<Record>>,
)
{   let Ok ( mut text ) = qry_text.get_single_mut() else { return };
    let Some ( mut record ) = opt_record else { return };

    //demo中のスコアがdemoのハイスコアを超えた場合 記録を更新する
    if record.score() > record.demo_hi_score()
    {   *record.demo_hi_score_mut() = record.score();
        *record.demo_stage_mut()    = record.stage();
    }

    //表示を更新する
    let value = format!( "{:02}-{:05}", record.demo_stage(), record.demo_hi_score(), );
    text.sections[ PLACE_HOLDER_DEMO_RECORD ].value = value;
}

////////////////////////////////////////////////////////////////////////////////

//End of code.