mod style;

use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{game::component::*, combat::WhoseTurn};
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_camera)
            .add_event::<CastSkillEvent>()
            .add_event::<CombatButtonEvent>()
            .add_event::<SkillContextEvent>()
            .insert_resource(ContextHistory(Vec::new()))
            .add_startup_system(spawn_combat_button)
            .add_system_set(
                SystemSet::new()
                    .with_system(combat_button_interact)
                    .with_system(event_combat_button),
            )
            // GameState
            .add_loopless_state(GameState::OutOfCombat)
            // see WhoseTurn::player in combat
            .add_enter_system(GameState::InCombat, draw_skill_icons)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::InCombat)
                    .with_system(skill_button_interact)
                    .into(),
            )
            // SkillContextStatus
            .add_loopless_state(SkillContextStatus::Closed)
            .add_enter_system(SkillContextStatus::Open, draw_skill_context)
            // despawning draws
            .add_exit_system(WhoseTurn::Player, despawn_with::<ContextWindow>)
            .add_exit_system(SkillContextStatus::Open, despawn_with::<ContextWindow>);
    }
}
/// State indicating whether the character is interacting with the open world or in combat
/// OutOfCombat: when character is in world, can move
/// InCombat: when character is in combat, can't move
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    InCombat,
    OutOfCombat,
}

const TEXT_COLOR: Color = Color::SILVER;
const NORMAL_BUTTON: Color = Color::rgb(0.5, 0.25, 0.5);
const HOVERED_BUTTON: Color = Color::rgb(0.35, 0.35, 0.35);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

struct CombatButtonEvent;

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
                size: Size::new(Val::Px(210.), Val::Px(30.)),
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
                    font_size: 20.,
                    color: TEXT_COLOR,
                },
            ));
        });
}

pub fn draw_skill_icons(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    skills_q: Query<(Entity, &LabelName), (With<Skill>, With<Learned>)>,
) {
    let mut index = 0;
    for (skill_ent, name) in skills_q.iter() {
        commands
            .spawn_bundle(ButtonBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        left: Val::Px(40. + index as f32 * 170.),
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
            .insert(SkillEnt(skill_ent))
            .insert(SkillIcon);
        index += 1;
    }
}

fn combat_button_interact(
    mut interaction_q: Query<
        (&Interaction, &mut UiColor, &Children),
        (Changed<Interaction>, (With<Button>, Without<crate::Skill>)),
    >,
    mut text_q: Query<&mut Text>,
    mut ev_buttonclick: EventWriter<CombatButtonEvent>,
) {
    for (interaction, mut color, children) in &mut interaction_q {
        // NOTE: grabbing children data here
        // TODO: read about get_mut() later
        let mut text_data = text_q.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text_data.sections[0].value = "clicked".to_string();
                *color = PRESSED_BUTTON.into();
                ev_buttonclick.send(CombatButtonEvent);
            }
            Interaction::Hovered => {
                text_data.sections[0].value = "hovered".to_string();
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                // still needs to set value for None case, otherwise the text
                // won't change back from Clicked or Hovered
                *color = NORMAL_BUTTON.into();
                text_data.sections[0].value = "enter combat debug".to_string();
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SkillContextStatus {
    Open,
    Closed,
}

/// Event { SkillEnt }
struct SkillContextEvent {
    skill_ent: SkillEnt,
}
/// shows info on hover, send event on click
fn skill_button_interact(
    context_state: Res<CurrentState<SkillContextStatus>>,
    mut commands: Commands,
    mut button_interaction_q: Query<
        (&Interaction, &mut UiColor, &SkillEnt),
        (Changed<Interaction>, With<Button>, With<Skill>),
    >,
    mut ev_castskill: EventWriter<CastSkillEvent>,
    mut ev_skillcontext: EventWriter<SkillContextEvent>,
    mut history: ResMut<ContextHistory>,
    enemy_q: Query<Entity, With<Enemy>>
) {
    for (interaction, mut color, skill_ent) in &mut button_interaction_q {
        match *interaction {
            Interaction::Clicked => {
                history.0.push(*skill_ent);
                if history.0.len() > 2 {
                    history.0.remove(0);
                }
                // if a context window is already opened
                if context_state.0 == SkillContextStatus::Open {
                    match history.0.get(1) {
                        Some(b) => {
                            if b.0 == history.0.get(0).unwrap().0 {
                                ev_castskill.send(CastSkillEvent {
                                    skill_ent: *skill_ent,
                                    target: enemy_q.single()
                                    // debug for blocking
                                    // target: player.single()
                                });
                                info!("CastSkillEvent {:?}", skill_ent);
                                // reset history
                                history.0 = Vec::new();
                                commands.insert_resource(NextState(SkillContextStatus::Closed));
                                commands.insert_resource(NextState(WhoseTurn::System));
                            }
                        }
                        None => {}
                    }
                } else {
                    // fresh context window
                    commands.insert_resource(NextState(SkillContextStatus::Open));
                    ev_skillcontext.send(SkillContextEvent {
                        skill_ent: *skill_ent,
                    });
                }
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn event_combat_button(mut commands: Commands, mut ev_buttonclick: EventReader<CombatButtonEvent>) {
    for _ in ev_buttonclick.iter() {
        // changing state
        info!("GameState::InCombat");
        commands.insert_resource(NextState(GameState::InCombat));
    }
}

fn draw_skill_context(
    mut commands: Commands,
    mut ev_skillcontext: EventReader<SkillContextEvent>,
    skill_q: Query<(&LabelName, Option<&Damage>, Option<&Block>, Option<&Heal>), With<Skill>>,
    asset_server: Res<AssetServer>,
) {
    // TODO: complete with info text and window size + placements
    for ev in ev_skillcontext.iter() {
        if let Ok((name, dmg, block, heal)) = skill_q.get(ev.skill_ent.0) {
            let (mut a, mut b, mut c): (String, String, String) = (String::new(), String::new(), String::new());
            if dmg.is_some() {
                a = format!("Deal {} points of Damage", dmg.unwrap().value)
            }
            if block.is_some() {
                b = format!("Grant {} points of Block", block.unwrap().value)
            }
            if heal.is_some() {
                c = format!("Heal the target for {} points", heal.unwrap().value)
            }
            let skill_description = format!("{}\n{}\n{}", a, b, c);

            // root note < <Node/Text>(title) <Node/Text>(info)>
            // 20/80, center alignment title
            commands
                // root
                .spawn_bundle(NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        position: UiRect {
                            left: Val::Px(100.),
                            top: Val::Px(300.),
                            ..default()
                        },
                        size: Size::new(Val::Px(400.), Val::Px(400.)),
                        border: UiRect::all(Val::Px(2.)),
                        flex_direction: FlexDirection::ColumnReverse, // top to bottom
                        ..default()
                    },
                    color: Color::NONE.into(),
                    ..default()
                })
                // node/text title
                // 20% height, center div
                .with_children(|parent| {
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.), Val::Percent(20.)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            color: Color::SILVER.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn_bundle(TextBundle::from_section(
                                name.name.clone(),
                                TextStyle {
                                    font: asset_server.load("font.ttf"),
                                    font_size: 20.,
                                    color: Color::PINK.into(),
                                },
                            ));
                        });
                })
                // node/text info
                .with_children(|parent| {
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.), Val::Percent(80.)),
                                align_items: AlignItems::FlexEnd,
                                ..default()
                            },
                            color: Color::PURPLE.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn_bundle(TextBundle::from_section(
                                skill_description,
                                TextStyle {
                                    font: asset_server.load("font.ttf"),
                                    font_size: 20.,
                                    color: Color::PINK.into(),
                                },
                            ));
                        });
                })
                .insert(ContextWindow);
        }
    }
}

pub fn despawn_with<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for ent in query.iter() {
        commands.entity(ent).despawn_recursive();
    }
}
