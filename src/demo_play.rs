use super::*;

//プラグインの設定
pub struct DemoPlay;
impl Plugin for DemoPlay
{   fn build( &self, app: &mut App )
    {   //GameState::TitleDemo
        //------------------------------------------------------------------------------------------
        app
        .add_system_set
        (   SystemSet::on_enter( GameState::TitleDemo )         //<ENTER>
            .before( Mark::MakeMapNewData )                     //<label>
            .with_system( init_demoplay_record )                //demoでのrecordの初期化
        )
        .add_system_set
        (   SystemSet::on_enter( GameState::TitleDemo )         //<ENTER>
            .label( Mark::MakeMapNewData )                      //<label>
            .with_system( map::make_new_data )                  //新マップのデータ作成
        )
        .add_system_set
        (   SystemSet::on_enter( GameState::TitleDemo )         //<ENTER>
            .after( Mark::MakeMapNewData )                      //<after>
            .with_system( map::spawn_sprite )                   //スプライトをspawnする
            .with_system( player::spawn_sprite )                //スプライトをspawnする
            .with_system( chasers::spawn_sprite )               //スプライトをspawnする
        )
        .add_system_set
        (   SystemSet::on_update( GameState::TitleDemo )        //<UPDATE>
            .before( Mark::DetectCollisions )                   //<before>
            .with_system( player::scoring_and_clear_stage )     //スコアリング＆クリア判定⇒DemoLoop
        )
        .add_system_set
        (   SystemSet::on_update( GameState::TitleDemo )        //<UPDATE>
            .label( Mark::DetectCollisions )                    //<label>
            .with_system( chasers::detect_collisions )          //衝突判定⇒DemoLoop
        )
        .add_system_set
        (   SystemSet::on_update( GameState::TitleDemo )        //<UPDATE>
            .after( Mark::DetectCollisions )                    //<after>
            .with_system( player::move_sprite )                 //スプライト移動
            .with_system( chasers::move_sprite )                //スプライト移動
        )
        ;
        //------------------------------------------------------------------------------------------

        //debugy用スプライトの制御
        #[cfg( debug_assertions )]
        app
        .add_system_set
        (   SystemSet::on_enter( GameState::TitleDemo )         //<ENTER>
            .after( Mark::MakeMapNewData )                      //<after>
            .with_system( spawn_debug_sprite )                  //スプライトをspawnする
        )
        .add_system_set
        (   SystemSet::on_update( GameState::TitleDemo )        //<UPDATE>
            .after( Mark::DetectCollisions )                    //<after>
            .with_system( update_debug_sprite )                 //スプライト移動
        )
        .add_system_set
        (   SystemSet::on_exit( GameState::TitleDemo )          //<EXIT>
            .with_system( despawn_entity::<DotsRect> )          //スプライト削除
        )
        ;

        //GameState::DemoNext
        //------------------------------------------------------------------------------------------
        app
        .add_system_set
        (   SystemSet::on_update( GameState::DemoLoop )         //<UPDATE>
            .with_system( goto_title )                          //無条件⇒TitleDemo
        )
        ;
        //------------------------------------------------------------------------------------------
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//demoクリアを除き、recordを初期化する
fn init_demoplay_record
(   mut record: ResMut<Record>,
)
{   if ! record.demo.clear_flag
    {   //GameOver後replayしなかった場合、demoで追手につかまった場合
        record.score = 0;
        record.stage = 0;
    }
    else
    {   //demoでステージクリアした場合
        record.demo.clear_flag = false;
    }
}

//無条件でStateを更新⇒TitleDemo
fn goto_title
(   mut state: ResMut<State<GameState>>,
)
{   let _ = state.overwrite_set( GameState::TitleDemo );
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//debug用スプライトをspawnする
#[cfg( debug_assertions )]
fn spawn_debug_sprite
(   map: ResMut<Map>,
    mut cmds: Commands,
)
{   let ( x, y, w, h ) = map.demo.debug_pixel_rect();
    let custom_size = Some ( Pixel::new( w, h ) );
    let pixel3 = Pixel::new( x, y ).extend( _DEPTH_SPRITE_DEBUG_RECT );
    let color = _COLOR_SPRITE_DEBUG_RECT;

    cmds
    .spawn( ( SpriteBundle::default(), DotsRect ) )
    .insert( Sprite { color, custom_size, ..default() } )
    .insert( Transform::from_translation( pixel3 ) )
    ;
}

//debug用スプライトの表示を更新する
#[cfg( debug_assertions )]
fn update_debug_sprite
(   mut q: Query<( &mut Transform, &mut Sprite ), With<DotsRect>>,
    map: Res<Map>,
)
{   let Ok ( ( mut transform, mut sprite ) ) = q.get_single_mut() else { return };

    let ( x, y, w, h ) = map.demo.debug_pixel_rect();
    let custom_size = Some ( Pixel::new( w, h ) );
    let pixel3 = Pixel::new( x, y ).extend( _DEPTH_SPRITE_DEBUG_RECT );

    transform.translation = pixel3;
    sprite.custom_size = custom_size;
}

//debug用スプライトのpixel座標を求める
#[cfg( debug_assertions )]
impl DemoParams
{   pub fn debug_pixel_rect( &self ) -> ( f32, f32, f32, f32 )
    {   let px_min = self.dots_rect_min().into_pixel_map();
        let px_max = self.dots_rect_max().into_pixel_map();

        let px_w = px_max.x - px_min.x;
        let px_h = px_min.y - px_max.y; //pixelはY軸が逆向き
        let px_x = px_min.x + px_w / 2.0;
        let px_y = px_max.y + px_h / 2.0; //pixelはY軸が逆向き

        ( px_x, px_y, px_w + PIXELS_PER_GRID, px_h + PIXELS_PER_GRID )
    }
}

//End of code.