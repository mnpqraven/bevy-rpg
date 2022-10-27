use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::game::component::*;
use crate::game::GameState;
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_camera)
            .add_event::<CastSkillEvent>()
            .add_system(spawn_skill_buttons)
            .add_system(skill_button_interact)
            .add_system(cast_skill)
            .add_event::<ButtonClickEvent>()
            .add_startup_system(spawn_combat_button.after(spawn_camera))
            .add_system(combat_button_interact)
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

fn spawn_combat_button(mut commands: Commands, asset_server: Res<AssetServer>) {
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
                    font: asset_server.load("font.ttf"),
                    font_size: 30.,
                    color: TEXT_COLOR,
                },
            ));
        });
}

// TODO: refactor
#[derive(Bundle)]
struct SkillBundle {
    name: LabelName,
    block: Block,
    heal: Heal,
    damage: Damage,
}
#[derive(Component, Debug, Copy, Clone)]
struct SkillEnt(Entity);

/// spawns buttons of every skills the player has learned and can use
fn spawn_skill_buttons(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    skills_q: Query< (Entity, &LabelName), (With<Skill>, With<Learned>)>,
) {
    let mut index = 0;
    for (skill_ent, name) in skills_q.iter() {
        commands
            .spawn_bundle(ButtonBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        left: Val::Px(10. + index as f32 * 170.),
                        top: Val::Px(700.),
                        ..default()
                    },
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect::all(Val::Auto),
                    size: Size::new(Val::Px(160.), Val::Px(35.)),
                    ..default()
                },
                color: NORMAL_BUTTON.into(),
                ..default()
            })
            .insert(Skill)
            .with_children(|parent| {
                parent.spawn_bundle(TextBundle::from_section(
                    &name.name,
                    TextStyle {
                        font: asset_server.load("font.ttf"),
                        font_size: 20.,
                        color: TEXT_COLOR,
                    },
                ));
            })
            // dump skill data here
            .insert(SkillEnt(skill_ent));
        index += 1;
    }
}

fn combat_button_interact(
    mut interaction_q: Query<
        (&Interaction, &mut UiColor, &Children),
        (Changed<Interaction>, (With<Button>, Without<crate::Skill>)),
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
                *color = NORMAL_BUTTON.into();
                text_data.sections[0].value = "state debug".to_string();
            }
        }
    }
}

/// carries skill ent
pub struct CastSkillEvent {
    skill_ent: SkillEnt,
}
/// shows info on hover, send event on click
fn skill_button_interact(
    mut button_interaction_q: Query<
        (&Interaction, &mut UiColor, &SkillEnt),
        (Changed<Interaction>, With<Button>, With<Skill>),
    >,
    mut ev_castskill: EventWriter<CastSkillEvent>,
    skill_q: Query<(Entity, &LabelName, Option<&Mana>,Option<&Damage>, Option<&Block>, Option<&Heal>), With<Skill>>,
) {
    // TODO: match skill_ent with corresponding button
    for (interaction, color, skill_ent) in &mut button_interaction_q {
        match *interaction {
            // TODO: other fancy stuff with color
            Interaction::Clicked => ev_castskill.send(CastSkillEvent {
                skill_ent: *skill_ent,
            }),
            Interaction::Hovered => {
                for (ent, name, mana_cost, damage, block, heal) in skill_q.iter() {
                    if ent == skill_ent.0 {
                        info!("{}", name.name);
                        if damage.is_some() {
                            info!("Deals {} damage", damage.unwrap().value);
                        }
                        if mana_cost.is_some() {
                            info!("Costs {} mana", mana_cost.unwrap().value);
                        }
                        if block.is_some() {
                            info!("Gains {} Block", block.unwrap().value);
                        }
                        if heal.is_some() {
                            info!("Heals for {}", heal.unwrap().value);
                        }
                    }
                }
                // debug!("TODO: skill context window on hover\nprobably change state, pass skill_ent should be\nenough for current information flow");
            }
            Interaction::None => {
                // TODO: despawns skill context
            }
        }
    }
}

fn cast_skill(
    mut ev_castskill: EventReader<CastSkillEvent>,
    skill_q: Query<(Entity, &LabelName), With<Skill>>,
) {
    for ev in ev_castskill.iter() {
        for (skill_ent, skill_name) in skill_q.iter() {
            if skill_ent == ev.skill_ent.0 {
                info!("CastSkillEvent {:?}", skill_name.name);
            }
        }
    }
}

fn event_button_click(mut commands: Commands, mut ev_buttonclick: EventReader<ButtonClickEvent>) {
    for _ in ev_buttonclick.iter() {
        debug!("ButtonClickEvent");
        // changing state
        commands.insert_resource(NextState(GameState::InCombat));
    }
}
