use super::*;
use crate::combat::process::gen_target_bucket;
use crate::game::despawn_with;
use crate::{combat::ControlMutex, ecs::component::*};
use bevy::prelude::*;
use iyes_loopless::prelude::*;
use style::*;

pub struct CombatUIPlugin;
impl Plugin for CombatUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CombatButtonEvent>()
            .add_event::<OpenSkillContextEvent>()
            .add_event::<TargetPromptEvent>()
            .add_event::<TargetSelectEvent>()
            .insert_resource(SelectingSkill(None))
            .insert_resource(CurrentCaster(None))
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
            .add_enter_system_set(
                SkillWheelStatus::Open,
                ConditionSet::new()
                    .with_system(draw_skill_icons)
                    .with_system(draw_hp_bars)
                    .with_system(draw_mp_bars)
                    .into(),
            )
            .add_system(skill_button_interact.run_in_state(GameState::InCombat))
            // SkillContextStatus
            .add_loopless_state(SkillContextStatus::Closed)
            .add_enter_system(SkillContextStatus::Open, draw_skill_context)
            .add_system(mouse_input_interact)
            .add_system_set(
                SystemSet::new()
                    .with_system(prompt_window_interact.run_in_state(TargetPromptStatus::Open))
                    .with_system(evread_targetselect),
            )
            // TargetPrompt
            .add_loopless_state(TargetPromptStatus::Closed)
            .add_enter_system(TargetPromptStatus::Open, draw_prompt_window)
            // despawning draws
            .add_exit_system_set(
                SkillWheelStatus::Open,
                ConditionSet::new()
                    .with_system(despawn_with::<SkillIcon>)
                    .into(),
            )
            .add_exit_system(SkillContextStatus::Open, despawn_with::<ContextWindow>)
            .add_exit_system(TargetPromptStatus::Open, despawn_with::<PromptWindow>);
    }
}

/// Draws "enter combat" button (debugging)
fn draw_combat_button(mut commands: Commands, font_handle: Res<FontSheet>) {
    commands
        .spawn(ButtonBundle {
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
            background_color: NORMAL_BUTTON.into(),
            ..default()
        })
        // text is a child component of this ButtonBundle
        // text can only exist inside ButtonBundle
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "enter combat",
                textstyle_skill_label(&font_handle),
            ));
        });
}

/// Draw skill wheel
pub fn draw_skill_icons(
    mut commands: Commands,
    font_handle: Res<FontSheet>,
    skills_q: Query<(Entity, &LabelName), (With<Skill>, With<Learned>)>,
) {
    for (index, (skill_ent, name)) in skills_q.iter().enumerate() {
        commands
            .spawn(ButtonBundle {
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
                background_color: NORMAL_BUTTON.into(),
                ..default()
            })
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    &name.0,
                    textstyle_skill_label(&font_handle),
                ));
            })
            // add button specific component meta
            .insert((Skill, SkillEnt(skill_ent), SkillIcon));
    }
}

/// Draws target selection window
fn draw_prompt_window(
    mut commands: Commands,
    unit_q: Query<
        (Entity, Option<&Player>, Option<&Ally>, Option<&Enemy>),
        Or<(With<Player>, With<Ally>, With<Enemy>)>,
    >,
    name_q: Query<&LabelName>,
    skill_q: Query<&Target, With<Skill>>,
    selecting_skill: Res<SelectingSkill>,
    current_caster: Res<CurrentCaster>,
    font_handle: Res<FontSheet>,
) {
    let mut index: f32 = 0.;
    let target_type = skill_q
        .get(selecting_skill.0.expect("SelectingSkill resource is emtpy"))
        .expect("ui::combat.rs: can't get target type")
        .clone();
    let targets = gen_target_bucket(unit_q.to_readonly(), target_type, current_caster.0);
    for unit_ent in targets {
        commands
            .spawn(ButtonBundle {
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
                background_color: Color::PINK.into(),
                ..default()
            })
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    name_q.get(unit_ent).unwrap().0.clone(),
                    textstyle_skill_label(&font_handle),
                ));
            })
            .insert((TargetEnt(unit_ent), PromptWindow));
        index += 1.;
    }
}
/// Draw hp bars of units in combat
fn draw_hp_bars(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    unit_q: Query<
        (&Health, Option<&Player>, Option<&Ally>, Option<&Enemy>),
        Or<(With<Player>, With<Enemy>, With<Ally>)>,
    >,
) {
    // left and right
    for (index, (unit_health, _, _, enemy_tag)) in unit_q.iter().enumerate() {
        let pos: UiRect = match enemy_tag {
            Some(_) => UiRect {
                right: Val::Percent(5.),
                top: Val::Px(100. + index as f32 * 50.),
                ..default()
            },
            None => UiRect {
                left: Val::Percent(5.),
                top: Val::Px(100. + index as f32 * 50.),
                ..default()
            },
        };
        commands
            // root
            .spawn(NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: pos,
                    size: Size::new(Val::Px(50.), Val::Px(50.)),
                    border: UiRect::all(Val::Px(2.)),
                    flex_direction: FlexDirection::ColumnReverse, // top to bottom
                    ..default()
                },
                background_color: Color::NONE.into(),
                ..default()
            })
            // node/text title
            // 20% height, center div
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    unit_health.0.to_string(),
                    TextStyle {
                        font: asset_server.load("font.ttf"),
                        font_size: 20.,
                        color: Color::PINK,
                    },
                ));
            })
            .insert(HPBar);
    }
}

/// Draws mp bars of units in combat
fn draw_mp_bars(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    unit_q: Query<
        (&Mana, Option<&Player>, Option<&Ally>, Option<&Enemy>),
        Or<(With<Player>, With<Enemy>, With<Ally>)>,
    >,
) {
    // left and right
    for (index, (unit_mana, _, _, enemy_tag)) in unit_q.iter().enumerate() {
        let pos: UiRect = match enemy_tag {
            Some(_) => UiRect {
                right: Val::Percent(6.),
                top: Val::Px(110. + index as f32 * 50.),
                ..default()
            },
            None => UiRect {
                left: Val::Percent(6.),
                top: Val::Px(110. + index as f32 * 50.),
                ..default()
            },
        };
        commands
            // root
            .spawn(NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: pos,
                    size: Size::new(Val::Px(50.), Val::Px(50.)),
                    border: UiRect::all(Val::Px(2.)),
                    flex_direction: FlexDirection::ColumnReverse, // top to bottom
                    ..default()
                },
                background_color: Color::NONE.into(),
                ..default()
            })
            // node/text title
            // 20% height, center div
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    unit_mana.0.to_string(),
                    TextStyle {
                        font: asset_server.load("font.ttf"),
                        font_size: 20.,
                        color: Color::BLUE,
                    },
                ));
            })
            .insert(MPBar);
    }
}
/// Interaction logic for targetting window
fn prompt_window_interact(
    mut prompt_window_interaction_q: Query<
        (&Interaction, &mut BackgroundColor, &TargetEnt),
        (Changed<Interaction>, With<PromptWindow>),
    >,
    mut ev_targetselect: EventWriter<TargetSelectEvent>,
    mut commands: Commands,
) {
    for (interaction, mut color, target_ent) in &mut prompt_window_interaction_q {
        *color = match *interaction {
            Interaction::Clicked => {
                ev_targetselect.send(TargetSelectEvent(target_ent.0));
                commands.insert_resource(NextState(TargetPromptStatus::Closed));
                Color::RED.into()
            }
            Interaction::Hovered => Color::ORANGE_RED.into(),
            Interaction::None => Color::PINK.into(),
        }
    }
}

/// Listens @a target  is selected from the prompt window
fn evread_targetselect(
    mut ev_targetselect: EventReader<TargetSelectEvent>,
    mut ev_castskill: EventWriter<CastSkillEvent>,
    selecting_skill: Res<SelectingSkill>,
    current_caster: Res<CurrentCaster>,
) {
    for target_ent in ev_targetselect.iter() {
        ev_castskill.send(CastSkillEvent {
            skill_ent: SkillEnt(selecting_skill.0.unwrap()),
            caster: current_caster.0.unwrap(),
            target: target_ent.0,
        });
    }
}

/// interaction logic for combat button (debug)
fn combat_button_interact(
    mut interaction_q: Query<
        (&Interaction, &mut BackgroundColor, &Children),
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
    mut commands: Commands,
    current_control_mutex: Res<CurrentState<ControlMutex>>,
) {
    for (interaction, mut color, children) in &mut interaction_q {
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
                let lmao = match current_control_mutex.0 {
                    ControlMutex::Unit => "unit".to_string(),
                    ControlMutex::System => "system".to_string(),
                    ControlMutex::Startup => "none".to_string(),
                };
                text_data.sections[0].value = lmao;
            }
        }
    }
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
/// Shows skill context window on 1st click
///
/// Opens up prompt window on 2nd click
fn skill_button_interact(
    context_state: Res<CurrentState<SkillContextStatus>>,
    mut commands: Commands,
    mut button_interaction_q: Query<
        (&Interaction, &mut BackgroundColor, &SkillEnt),
        (Changed<Interaction>, With<Button>, With<Skill>),
    >,
    mut ev_skillcontext: EventWriter<OpenSkillContextEvent>,
    mut history: ResMut<ContextHistory>,
    player_q: Query<Entity, With<Player>>,
) {
    for (interaction, mut color, skill_ent) in &mut button_interaction_q {
        match *interaction {
            Interaction::Clicked => {
                // if a context window is already opened
                match context_state.0 {
                    // same skill selected > open prompt window
                    SkillContextStatus::Open if history.0 == Some(*skill_ent) => {
                        commands.insert_resource(SelectingSkill(Some(skill_ent.0)));
                        // NOTE: caster resource is hardcoded to only player rn
                        // TODO: modular
                        commands
                            .insert_resource(CurrentCaster(Some(player_q.get_single().unwrap())));
                        commands.insert_resource(NextState(SkillContextStatus::Closed));
                        commands.insert_resource(NextState(SkillWheelStatus::Closed));
                        commands.insert_resource(NextState(TargetPromptStatus::Open));
                    }
                    // different skill selected > despawn and redraw
                    SkillContextStatus::Open => {
                        commands.insert_resource(NextState(SkillContextStatus::Closed));
                        commands.insert_resource(NextState(SkillContextStatus::Open));
                        ev_skillcontext.send(OpenSkillContextEvent {
                            skill_ent: *skill_ent,
                        });
                    }
                    // fresh context window
                    SkillContextStatus::Closed => {
                        commands.insert_resource(NextState(SkillContextStatus::Open));
                        ev_skillcontext.send(OpenSkillContextEvent {
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

/// debug
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

/// Draw skill context window, showing more infomation about the selected skill
fn draw_skill_context(
    mut commands: Commands,
    mut ev_skillcontext: EventReader<OpenSkillContextEvent>,
    skill_q: Query<
        (
            &LabelName,
            Option<&Damage>,
            Option<&Block>,
            Option<&Heal>,
            Option<&Channel>,
            &Target,
        ),
        With<Skill>,
    >,
    font_handle: Res<FontSheet>,
) {
    // TODO: complete with info text and window size + placements
    // TODO: this block should be processed in parser.rs and reused
    for ev in ev_skillcontext.iter() {
        if let Ok((name, _dmg, _block, _heal, channel, target_type)) = skill_q.get(ev.skill_ent.0) {
            let mut d = String::new();
            // TODO: generic refactor done -> implement builder
            match channel {
                Some(x) if x.0 > 1 => d = format!("Channels for {} turns\n", channel.unwrap().0),
                Some(_) => d = format!("Channels for {} turn\n", channel.unwrap().0),
                None => {}
            }
            let target = format!("{:?}", target_type);
            let skill_description = format!("{}\n{}", d, target);

            // TODO: can be refactored as events
            // root note < <Node/Text>(title) <Node/Text>(info)>
            // 20/80, center alignment title
            commands
                // root
                .spawn(NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        position: UiRect {
                            left: Val::Px(100.),
                            top: Val::Px(300.),
                            ..default()
                        },
                        size: Size::new(Val::Px(400.), Val::Px(400.)),
                        border: UiRect::all(Val::Px(2.)),
                        flex_direction: FlexDirection::Column, // top to bottom
                        ..default()
                    },
                    background_color: Color::NONE.into(),
                    ..default()
                })
                // node/text title
                // 20% height, center div
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.), Val::Percent(20.)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: Color::MIDNIGHT_BLUE.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                name.0.clone(),
                                textstyle_skill_label(&font_handle),
                            ));
                        });
                })
                // node/text info
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.), Val::Percent(80.)),
                                align_items: AlignItems::FlexStart,
                                ..default()
                            },
                            background_color: Color::PURPLE.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                skill_description,
                                textstyle_skill_label(&font_handle),
                            ));
                        });
                })
                .insert(ContextWindow);
        }
    }
}
/// Status of the skill context window that opens up when use selects a skill
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SkillContextStatus {
    Open,
    Closed,
}
/// Event { SkillEnt }
/// opens the skill context window with the bound skill_ent when the event is called
struct OpenSkillContextEvent {
    skill_ent: SkillEnt,
}
struct CombatButtonEvent;
struct TargetPromptEvent;
#[derive(Component)]
struct HPBar;
#[derive(Component)]
struct MPBar;
