use crate::game::despawn_with;
use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::game::component::*;

use super::*;

pub struct CombatUIPlugin;

impl Plugin for CombatUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_camera)
            .add_event::<CombatButtonEvent>()
            .add_event::<SkillContextEvent>()
            .add_event::<TargetPromptEvent>()
            .add_event::<TargetSelectEvent>()
            .insert_resource(SelectingSkill(None))
            .insert_resource(ContextHistory(None))
            .add_startup_system(draw_combat_button)
            .add_system_set(
                SystemSet::new()
                    .with_system(combat_button_interact)
                    .with_system(evread_combat_button),
            )
            // GameState
            .add_loopless_state(GameState::OutOfCombat)
            .add_loopless_state(SkillWheelStatus::Closed)
            .add_enter_system(SkillWheelStatus::Open, draw_skill_icons)
            .add_system(skill_button_interact.run_in_state(GameState::InCombat))
            // SkillContextStatus
            .add_loopless_state(SkillContextStatus::Closed)
            .add_enter_system(SkillContextStatus::Open, draw_skill_context)
            .add_system(mouse_input_interact)
            .add_system_set(
                SystemSet::new()
                    .with_system(prompt_window_interact.run_in_state(TargetPromptStatus::Open))
                    .with_system(evread_targetselect)
                    .into(),
            )
            // TargetPrompt
            .add_loopless_state(TargetPromptStatus::Closed)
            .add_enter_system(TargetPromptStatus::Open, draw_prompt_window)
            // despawning draws
            .add_exit_system(SkillWheelStatus::Open, despawn_with::<SkillIcon>)
            .add_exit_system(SkillContextStatus::Open, despawn_with::<ContextWindow>)
            .add_exit_system(TargetPromptStatus::Open, despawn_with::<PromptWindow>);
    }
}

const TEXT_COLOR: Color = Color::SILVER;
const NORMAL_BUTTON: Color = Color::rgb(0.5, 0.25, 0.5);
const HOVERED_BUTTON: Color = Color::rgb(0.35, 0.35, 0.35);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

struct CombatButtonEvent;

// check skill detail, skills have built in targetting
// TODO: prompt for target selection first
struct TargetPromptEvent;

/// placeholder camera here
fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn draw_combat_button(mut commands: Commands, asset_server: Res<AssetServer>) {
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
            // add button specific component meta
            .insert(SkillEnt(skill_ent))
            .insert(SkillIcon);
        index += 1;
    }
}

/// TODO: selective drawing
fn draw_prompt_window(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    units_q: Query<
        (
            Entity,
            &LabelName,
            Option<&Player>,
            Option<&Ally>,
            Option<&Enemy>,
        ),
        Or<(With<Enemy>, With<Player>, With<Ally>)>,
    >,
    // queue skill table to get target type
    skill_q: Query<&Target, With<Skill>>,
    // apply skill from here to filter units
    selecting_skill: Res<SelectingSkill>,
) {
    let mut index: f32 = 0.;
    let target_type = skill_q.get(selecting_skill.0.unwrap()).unwrap();
    // filter out units not matching target type
    let filtered_units =
        units_q.iter().filter(
            |(_, _, player_tag, ally_tag, enemy_tag)| match target_type {
                Target::Player => player_tag.is_some(),
                Target::Ally | Target::AllyAOE => player_tag.is_some() || ally_tag.is_some(),
                Target::AllyButSelf => player_tag.is_none() && ally_tag.is_some(),
                Target::Enemy | Target::EnemyAOE => enemy_tag.is_some(),
                Target::Any => true,
                Target::AnyButSelf => player_tag.is_none(),
            },
        );
    for (unit_ent, unit_name, _, _, _) in filtered_units {
        commands
            .spawn_bundle(ButtonBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        right: Val::Px(200.),
                        top: Val::Px(200. + index * 60.),
                        ..default()
                    },
                    size: Size::new(Val::Px(50.), Val::Px(50.)),
                    border: UiRect::all(Val::Px(2.)),
                    ..default()
                },
                color: Color::PINK.into(),
                ..default()
            })
            .with_children(|parent| {
                parent.spawn_bundle(TextBundle::from_section(
                    &unit_name.name,
                    TextStyle {
                        font: asset_server.load("font.ttf"),
                        font_size: 20.,
                        color: Color::WHITE,
                    },
                ));
            })
            .insert(TargetEnt(unit_ent))
            .insert(PromptWindow);
        index += 1.;
    }
}
fn prompt_window_interact(
    mut prompt_window_interaction_q: Query<
        (&Interaction, &mut UiColor, &TargetEnt),
        (Changed<Interaction>, With<PromptWindow>),
    >,
    mut ev_targetselect: EventWriter<TargetSelectEvent>,
) {
    for (interaction, mut color, target_ent) in &mut prompt_window_interaction_q {
        match *interaction {
            Interaction::Clicked => {
                ev_targetselect.send(TargetSelectEvent(target_ent.0));
                *color = Color::RED.into();
            }
            Interaction::Hovered => {
                *color = Color::ORANGE_RED.into();
            }
            Interaction::None => {
                *color = Color::PINK.into();
            }
        }
    }
}

fn evread_targetselect(
    mut ev_targetselect: EventReader<TargetSelectEvent>,
    mut ev_castskill: EventWriter<CastSkillEvent>,
    selecting_skill: Res<SelectingSkill>,
) {
    for target_ent in ev_targetselect.iter() {
        debug!("should see");
        debug!("{:?}", selecting_skill);
        ev_castskill.send(CastSkillEvent {
            skill_ent: SkillEnt(selecting_skill.0.unwrap()),
            target: target_ent.0,
        });
    }
}

fn combat_button_interact(
    mut interaction_q: Query<
        (&Interaction, &mut UiColor, &Children),
        (
            Changed<Interaction>,
            (
                With<Button>,
                Without<Skill>,
                Without<PromptWindow>,
                Without<ContextWindow>,
            ),
        ),
    >,
    mut text_q: Query<&mut Text>,
    mut ev_buttonclick: EventWriter<CombatButtonEvent>,
    mut commands: Commands
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
                commands.insert_resource(NextState(SkillWheelStatus::Open));
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TargetPromptStatus {
    Open,
    Closed,
}

/// Event { SkillEnt }
struct SkillContextEvent {
    skill_ent: SkillEnt,
}

/// handles input for mouse
/// only right click for now, left click is already handled by context windows
fn mouse_input_interact(
    mut commands: Commands,
    buttons: Res<Input<MouseButton>>,
    context_state: Res<CurrentState<SkillContextStatus>>,
    prompt_state: Res<CurrentState<TargetPromptStatus>>,
) {
    if buttons.pressed(MouseButton::Right) {
        match prompt_state.0 {
            // hides prompt
            TargetPromptStatus::Open => {
                commands.insert_resource(NextState(TargetPromptStatus::Closed));
                commands.insert_resource(NextState(SkillWheelStatus::Open));
            }
            // hides context window
            TargetPromptStatus::Closed if context_state.0 == SkillContextStatus::Open => {
                commands.insert_resource(NextState(SkillContextStatus::Closed));
            }
            _ => {}
        }
    }
}
/// shows info on hover, send event on click
fn skill_button_interact(
    context_state: Res<CurrentState<SkillContextStatus>>,
    mut commands: Commands,
    mut button_interaction_q: Query<
        (&Interaction, &mut UiColor, &SkillEnt),
        (Changed<Interaction>, With<Button>, With<Skill>),
    >,
    mut ev_skillcontext: EventWriter<SkillContextEvent>,
    mut history: ResMut<ContextHistory>,
) {
    for (interaction, mut color, skill_ent) in &mut button_interaction_q {
        match *interaction {
            Interaction::Clicked => {
                // if a context window is already opened
                match context_state.0 {
                    // same skill selected > open prompt window
                    SkillContextStatus::Open if history.0 == Some(*skill_ent) => {
                        commands.insert_resource(SelectingSkill(Some(skill_ent.0)));
                        commands.insert_resource(NextState(SkillContextStatus::Closed));
                        commands.insert_resource(NextState(SkillWheelStatus::Closed));
                        commands.insert_resource(NextState(TargetPromptStatus::Open));
                    }
                    // different skill selected > despawn and redraw
                    SkillContextStatus::Open => {
                        commands.insert_resource(NextState(SkillContextStatus::Closed));
                        commands.insert_resource(NextState(SkillContextStatus::Open));
                        ev_skillcontext.send(SkillContextEvent {
                            skill_ent: *skill_ent,
                        });
                    }
                    // fresh context window
                    SkillContextStatus::Closed => {
                        commands.insert_resource(NextState(SkillContextStatus::Open));
                        ev_skillcontext.send(SkillContextEvent {
                            skill_ent: *skill_ent,
                        });
                    }
                }
                *color = PRESSED_BUTTON.into();
                history.0 = Some(*skill_ent);
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

fn evread_combat_button(
    mut commands: Commands,
    mut ev_buttonclick: EventReader<CombatButtonEvent>,
) {
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
            let (mut a, mut b, mut c): (String, String, String) =
                (String::new(), String::new(), String::new());
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