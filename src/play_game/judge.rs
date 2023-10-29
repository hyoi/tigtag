use super::*;

////////////////////////////////////////////////////////////////////////////////

//スコアの処理とクリア判定
#[allow(clippy::too_many_arguments)]
pub fn scoring_and_stageclear
(   qry_player: Query<&Player>,
    opt_map: Option<ResMut<Map>>,
    opt_score: Option<ResMut<Score>>,
    opt_hi_score: Option<ResMut<HiScore>>,
    state: Res<State<MyState>>,
    mut next_state: ResMut<NextState<MyState>>,
    mut evt_clear: EventWriter<EventClear>,
    mut cmds: Commands,
    asset_svr: Res<AssetServer>,
)
{   let Ok ( player ) = qry_player.get_single() else { return };
    let Some ( mut map ) = opt_map else { return };
    let Some ( dot ) = map.opt_entity( player.grid ) else { return };
    let Some ( mut score ) = opt_score else { return };
    let Some ( mut hi_score ) = opt_hi_score else { return };

    //ドットの削除
    cmds.entity( dot ).despawn();
    *map.opt_entity_mut( player.grid ) = None;

    //demoの場合、スプライト削除後(EntityにNone代入後)に残dots情報を更新する
    let is_demo = state.get().is_demoplay();
    // if is_demo { map.demo.update_params( player.grid ); }

    //スコア更新
    *score.get_mut() += 1;
    map.remaining_dots -= 1;

    //1度beepを鳴らす(despawn処理付き)
    let volume = Volume::Relative ( VolumeLevel::new( VOLUME_SOUND_BEEP ) );
    let sound_beep = AudioBundle
    {   source: asset_svr.load( ASSETS_SOUND_BEEP ),
        settings: PlaybackSettings::DESPAWN.with_volume( volume ),
    };
    cmds.spawn( sound_beep );

    //ハイスコアの更新
    if ! is_demo && score.get() > hi_score.get()
    {   *hi_score.get_mut() = score.get();
    }

    //全ドットを拾ったら、Clearへ遷移する
    if map.remaining_dots <= 0
    {   // record.is_clear = true;
        next_state.set
        (   match state.get()
            {   MyState::MainLoop  => MyState::StageClear,
                MyState::TitleDemo => MyState::DemoLoop,
                _ => unreachable!( "Bad state: {:?}", state.get() ),
            }
        );
        evt_clear.send( EventClear ); //後続の処理にクリアを伝える
    }
}

////////////////////////////////////////////////////////////////////////////////

//End of code.