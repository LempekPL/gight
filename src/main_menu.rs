use bevy::prelude::*;
use crate::{AppState, FontAssets, SoundAssets};
use bevy_kira_audio::Audio;
use bevy::app::AppExit;

pub struct MainMenuPlugin;

#[derive(Component)]
struct MainMenuText;

enum ButtonType {
    Play,
    Settings,
    Quit,
}

#[derive(Component)]
struct MainMenuButton(ButtonType);

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(spawn_main_menu))
            .add_system_set(SystemSet::on_update(AppState::MainMenu)
                .with_system(button_coloring_main_menu)
                .with_system(button_handling_main_menu)
            );
    }
}

fn spawn_main_menu(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    sound_assets: Res<SoundAssets>,
    audio: Res<Audio>,
) {
    // play song
    audio.play_looped(sound_assets.main_menu.clone());
    // spawn buttons
    let play_button = spawn_main_menu_button(&mut commands, &font_assets, "PLAY".to_string(), ButtonType::Play);
    let settings_button = spawn_main_menu_button(&mut commands, &font_assets, "Settings".to_string(), ButtonType::Settings);
    let quit_button = spawn_main_menu_button(&mut commands, &font_assets, "Quit".to_string(), ButtonType::Quit);
    let button_group = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .id();
    commands.entity(button_group).push_children(&[play_button, settings_button, quit_button]);
}

fn button_coloring_main_menu(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<MainMenuButton>),
    >,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match interaction {
            Interaction::Clicked => {
                *color = Color::YELLOW.into();
            }
            Interaction::Hovered => {
                *color = Color::GREEN.into();
            }
            Interaction::None => {
                *color = Color::DARK_GREEN.into();
            }
        }
    }
}

fn button_handling_main_menu(
    interaction_query: Query<
        (&Interaction, &MainMenuButton),
        Changed<Interaction>
    >,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, button_type) in interaction_query.iter() {
        match interaction {
            Interaction::Clicked => {
                match button_type.0 {
                    ButtonType::Quit => {
                        exit.send(AppExit);
                    }
                    _ => {
                        todo!();
                    }
                }
            }
            _ => {}
        }
    }
}

// HELPER function
// NOT SYSTEM
// commands is the Commands bevy struct
// font_assets are font assets loaded in main.rs struct
// name for display name
// type for button type
// returns entity
fn spawn_main_menu_button(commands: &mut Commands, font_assets: &Res<FontAssets>, name: String, button_type: ButtonType) -> Entity {
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                // size button
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
                    name,
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
        .insert(MainMenuButton(button_type))
        .id()
}