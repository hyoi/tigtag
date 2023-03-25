use super::*;

//プラグインの設定
pub struct DemoPlay;
impl Plugin for DemoPlay
{   fn build( &self, app: &mut App )
    {   //MyState::TitleDemo
        //------------------------------------------------------------------------------------------
        app
        .add_systems
        (   (   init_demoplay_record, //demoでのrecordの初期化
                map::make_new_data.in_set( MyLabel::MakeMapNewData ), //新マップのデータ作成
                map::spawn_sprite,     //スプライトをspawnする
                player::spawn_sprite,  //スプライトをspawnする
                chasers::spawn_sprite, //スプライトをspawnする
            )
            .chain()
            .in_schedule( OnEnter( MyState::TitleDemo ) )
        )
        .add_systems
        (   (   player::scoring_and_clear_stage, //スコアリング＆クリア判定⇒DemoLoop
                chasers::detect_collisions.in_set( MyLabel::DetectCollisions ), //衝突判定⇒DemoLoop
                player::move_sprite,  //スプライト移動
                chasers::move_sprite, //スプライト移動
            )
            .chain()
            .in_set( OnUpdate( MyState::TitleDemo ) )
        )
        ;
        //------------------------------------------------------------------------------------------

        //debugy用スプライトの制御
        #[cfg( debug_assertions )]
        app
        .add_system
        (   spawn_debug_sprite //スプライトをspawnする
            .after( MyLabel::MakeMapNewData )
            .in_schedule( OnEnter( MyState::TitleDemo ) )
        )
        .add_system
        (   update_debug_sprite //スプライト移動
            .after( MyLabel::DetectCollisions )
            .in_set( OnUpdate( MyState::TitleDemo ) )
        )
        .add_system
        (   despawn_entity::<DotsRect> //スプライト削除
            .in_schedule( OnExit( MyState::TitleDemo ) )
        )
        ;

        //MyState::DemoNext
        //------------------------------------------------------------------------------------------
        app
        .add_system
        (   goto_title //無条件⇒TitleDemo
            .in_set( OnUpdate( MyState::DemoLoop ) )
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
(   mut state: ResMut<NextState<MyState>>,
)
{   state.set( MyState::TitleDemo );
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