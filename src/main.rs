mod main_menu;

use bevy::prelude::*;
use bevy_asset_loader::{AssetLoader, AssetCollection};
use bevy_kira_audio::{AudioPlugin, AudioSource};

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    LoadingAssets,
    Loading,
    MainMenu,
}

#[derive(AssetCollection)]
struct FontAssets {
    #[asset(path = "fonts/open_sans/OpenSans-Bold.ttf")]
    open_sans_regular: Handle<Font>,
}

#[derive(AssetCollection)]
struct SoundAssets {
    #[asset(path = "sounds/main_menu_song.ogg")]
    main_menu: Handle<AudioSource>,
}

fn main() {
    let mut app = App::new();
    AssetLoader::new(GameState::LoadingAssets)
        .continue_to_state(GameState::Loading)
        .with_collection::<FontAssets>()
        .with_collection::<SoundAssets>()
        .build(&mut app);
    app.insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)));
    app.insert_resource(WindowDescriptor {
        title: "Gight".to_string(),
        width: (1280.0 / 2.0),
        height: (960.0 / 2.0),
        resizable: false,
        ..Default::default()
    });
    // this will tell the asset loader to load assets after loading assets it will change state loding to initiate setup
    app.add_state(GameState::LoadingAssets);
    app.add_plugins(DefaultPlugins);
    app.add_plugin(AudioPlugin);
    // main menu systems
    app.add_plugin(main_menu::MainMenuPlugin);
    // initiating setup
    app.add_system_set(SystemSet::on_enter(GameState::Loading).with_system(setup));


    app.run();
}

fn setup(
    mut commands: Commands,
    mut app_state: ResMut<State<GameState>>,
) {
    // camera 2d
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    // camera ui
    commands.spawn_bundle(UiCameraBundle::default());
    app_state.set(GameState::MainMenu).unwrap();
}