use crate::state::AppState;
use bevy::prelude::*;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<AssetsLoading>().add_system_set(
            SystemSet::on_update(AppState::Loading).with_system(check_assets_ready.system()),
        );
    }
}

pub struct AssetsLoading(Vec<HandleUntyped>);

impl FromWorld for AssetsLoading {
    fn from_world(_world: &mut World) -> Self {
        AssetsLoading::new()
    }
}

impl AssetsLoading {
    pub fn new() -> Self {
        AssetsLoading(Vec::new())
    }

    pub fn add(&mut self, handle_untyped: HandleUntyped) {
        self.0.push(handle_untyped);
    }
}

fn check_assets_ready(
    mut _cmds: Commands,
    mut app_state: ResMut<State<AppState>>,
    server: Res<AssetServer>,
    mut loading: ResMut<AssetsLoading>,
) {
    use bevy::asset::LoadState;

    match server.get_group_load_state(loading.0.iter().map(|h| h.id)) {
        LoadState::Failed => {
            // one of our assets had an error
        }
        LoadState::Loaded => {
            loading.0.clear();
            app_state.pop().unwrap();
        }
        _ => {
            // NotLoaded/Loading: not fully ready yet
        }
    }
}
