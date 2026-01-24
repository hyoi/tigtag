use super::*;

////////////////////////////////////////////////////////////////////////////////

// RenderPluginの初期化
pub trait InitRenderPlugin
{
    fn initialize(backend: Backends) -> Self;
}

impl InitRenderPlugin for RenderPlugin
{
    fn initialize(backend: Backends) -> Self
    {
        Self {
            // バックエンドの切替
            render_creation: RenderCreation::Automatic(WgpuSettings {
                backends: Some(backend),
                ..default()
            }),
            ..default()
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

// End of code.
