mod main_menu;

use bevy::prelude::*;
use bevy_asset_loader::{AssetLoader, AssetCollection};
use bevy_kira_audio::{AudioPlugin, AudioSource};

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum AppState {
    Preload,
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
    // sloooow loading,
    // when creator of bevy_asset_loader will make compatibility with bevy_loading then you could at least look at progress bar, now just stare at the loading text :/
    #[asset(path = "sounds/main_menu_song.ogg")]
    main_menu: Handle<AudioSource>,
}

#[derive(Component)]
struct LoadingText;

fn main() {
    let mut app = App::new();
    AssetLoader::new(AppState::LoadingAssets)
        .continue_to_state(AppState::Loading)
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
    app.add_startup_system(preload);
    // this will tell the asset loader to load assets after loading assets it will change state loding to initiate setup
    app.add_state(AppState::Preload);
    app.add_plugins(DefaultPlugins);
    app.add_plugin(AudioPlugin);
    // main menu systems
    app.add_plugin(main_menu::MainMenuPlugin);
    // initiating setup
    app.add_system_set(SystemSet::on_enter(AppState::Loading).with_system(setup));


    app.run();
}

fn preload(
    mut commands: Commands,
    mut app_state: ResMut<State<AppState>>,
    asset_server: Res<AssetServer>,
) {
    // camera ui
    commands.spawn_bundle(UiCameraBundle::default());

    // TODO: make some animation so it looks like something is actually doing
    // loading text so people would know that it's working
    commands.spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::FlexEnd,
            ..Default::default()
        },
        color: UiColor(Color::BLACK),
        ..Default::default()
    }).with_children(|parent| {
        parent.spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::Center,
                ..Default::default()
            },
            text: Text::with_section(
                "LOADING",
                TextStyle {
                    font: asset_server.load("fonts/open_sans/OpenSans-Bold.ttf"),
                    font_size: 80.0,
                    color: Color::WHITE,
                },
                Default::default()
            ),
            ..Default::default()
        });
    }).insert(LoadingText);
    // start loading assets
    app_state.set(AppState::LoadingAssets).unwrap();
}

fn setup(
    mut commands: Commands,
    mut app_state: ResMut<State<AppState>>,
    loading_text_query: Query<Entity, With<LoadingText>>
) {
    // remove loading text
    let loading_text_entity = loading_text_query.single();
    commands.entity(loading_text_entity).despawn_recursive();

    // camera 2d
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    // move user to main menu
    app_state.set(AppState::MainMenu).unwrap();
}