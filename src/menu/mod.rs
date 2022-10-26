use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::game::GameState;
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_camera)
            .add_event::<ButtonClickEvent>()
            .add_startup_system(button_setup.after(spawn_camera))
            .add_system(button_interact)
            .add_system(event_button_click);
    }
}

const TEXT_COLOR: Color = Color::SILVER;
const NORMAL_BUTTON: Color = Color::rgb(0.5, 0.25, 0.5);
const HOVERED_BUTTON: Color = Color::rgb(0.35, 0.35, 0.35);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

struct ButtonClickEvent;

/// placeholder camera here
fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn button_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Px(20.),
                    top: Val::Px(300.),
                    ..default()
                },
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Auto),
                size: Size::new(Val::Px(150.), Val::Px(35.)),
                ..default()
            },
            color: NORMAL_BUTTON.into(),
            ..default()
        })
        // text is a child component of this ButtonBundle
        // text can only exist inside ButtonBundle
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle::from_section(
                "enter combat",
                TextStyle {
                    font: asset_server.load("Halloween.ttf"),
                    font_size: 40.,
                    color: TEXT_COLOR,
                },
            ));
        });
}

fn button_interact(
    mut interaction_q: Query<
        (&Interaction, &mut UiColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_q: Query<&mut Text>,
    mut ev_buttonclick: EventWriter<ButtonClickEvent>,
) {
    for (interaction, mut color, children) in &mut interaction_q {
        // NOTE: grabbing children data here
        // TODO: read about get_mut() later
        let mut text_data = text_q.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text_data.sections[0].value = "clicked".to_string();
                *color = PRESSED_BUTTON.into();
                ev_buttonclick.send(ButtonClickEvent);
            }
            Interaction::Hovered => {
                text_data.sections[0].value = "hovered".to_string();
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                // still needs to set value for None case, otherwise the text
                // won't change back from Clicked or Hovered
                text_data.sections[0].value = "Click me".to_string();
            }
        }
    }
}

fn event_button_click(
    mut commands: Commands,
    mut ev_buttonclick: EventReader<ButtonClickEvent>,
) {
    for _ in ev_buttonclick.iter() {
        debug!("ButtonClickEvent");
        // changing state
        commands.insert_resource(NextState(GameState::InCombat));
    }
}
