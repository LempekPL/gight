use bevy::prelude::*;
use crate::{GameState, FontAssets, SoundAssets};
use bevy_kira_audio::Audio;

pub struct MainMenuPlugin;

#[derive(Component)]
struct MainMenuButton;

#[derive(Component)]
struct MainMenuText;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(spawn_main_menu))
            .add_system_set(SystemSet::on_update(GameState::MainMenu).with_system(handle_main_menu));
    }
}

fn spawn_main_menu(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    sound_assets: Res<SoundAssets>,
    audio: Res<Audio>
) {
    // play song
    audio.play_looped(sound_assets.main_menu.clone());
    // spawn buttons
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: Rect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: Color::WHITE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "PLAY",
                    TextStyle {
                        font: font_assets.open_sans_regular.clone(),
                        font_size: 40.0,
                        color: Color::WHITE,
                    },
                    Default::default(),
                ),
                ..Default::default()
            }).insert(MainMenuText);
        })
        .insert(MainMenuButton);
}

fn handle_main_menu(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<MainMenuButton>),
    >,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = Color::YELLOW.into();
            }
            Interaction::Hovered => {
                *color = Color::GREEN.into();
            }
            Interaction::None => {
                *color = Color::WHITE.into();
            }
        }
    }
}