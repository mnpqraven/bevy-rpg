use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::game::component::*;
use crate::game::GameState;
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_camera)
            .add_event::<CastSkillEvent>()
            // TODO: this should be a startup system
            .add_system(spawn_skill_buttons)
            .add_system(skill_button_interact)
            .add_event::<ButtonClickEvent>()
            .add_startup_system(spawn_combat_button.after(spawn_camera))
            .add_system(combat_button_interact)
            .add_system(event_button_click)
            // TODO: using loopless to manage state now, refactor event if
            // needed be
            .add_event::<SkillContextEvent>()
            .insert_resource(ContextHistory(Vec::new()))
            // conditional context menu
            .add_loopless_state(SkillContextStatus::Closed)
            .add_enter_system(SkillContextStatus::Open, spawn_skill_context_window)
            .add_exit_system(SkillContextStatus::Open, despawn_with::<ContextWindow>)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(SkillContextStatus::Open)
                    .with_system(cast_skill.run_if(same_skill_selected)) // only cast after you see the
                    // skill's details
                    .into(),
            );
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
    skills_q: Query<(Entity, &LabelName), (With<Skill>, With<Learned>)>,
) {
    let mut index = 0;
    for (skill_ent, name) in skills_q.iter() {
        commands
            .spawn_bundle(ButtonBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        left: Val::Px(10. + index as f32 * 170.),
                        top: Val::Px(500.),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SkillContextStatus {
    Open,
    Closed,
}

struct SkillContextEvent {
    skill_ent: SkillEnt,
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
    skill_q: Query<
        (
            Entity,
            &LabelName,
            Option<&Mana>,
            Option<&Damage>,
            Option<&Block>,
            Option<&Heal>,
        ),
        With<Skill>,
    >,
    mut ev_skillcontext: EventWriter<SkillContextEvent>,
    mut commands: Commands,
    state: Res<CurrentState<SkillContextStatus>>,
) {
    for (interaction, mut color, skill_ent) in &mut button_interaction_q {
        match *interaction {
            // TODO: other fancy stuff with color
            Interaction::Clicked => {
                // sends event data only when context is open
                if state.0 == SkillContextStatus::Open {
                    ev_castskill.send(CastSkillEvent {
                        skill_ent: *skill_ent,
                    });
                }
                commands.insert_resource(NextState(SkillContextStatus::Open));
                // context window
                ev_skillcontext.send(SkillContextEvent {
                    skill_ent: *skill_ent,
                });
                *color = PRESSED_BUTTON.into();
            },
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            },
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

// only 2 in vec, pass true to same_skill_selected if both are equal
#[derive(Component)]
struct ContextHistory(Vec<SkillEnt>);

/// returns whether if the skill the user click is the same as the context
/// window skill spawned on the screen
fn same_skill_selected(history: Res<ContextHistory>) -> bool {
    if history.0.get(0).is_some() && history.0.get(1).is_some() {
        if history.0[0].0 == history.0[1].0 {
            return true;
        }
    }
    false
}
fn cast_skill(
    mut ev_castskill: EventReader<CastSkillEvent>,
    skill_q: Query<(Entity, &LabelName), With<Skill>>,
    mut commands: Commands,
) {
    for ev in ev_castskill.iter() {
        for (skill_ent, skill_name) in skill_q.iter() {
            if skill_ent == ev.skill_ent.0 {
                info!("CastSkillEvent {:?}", skill_name.name);
                commands.insert_resource(NextState(SkillContextStatus::Closed));
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

fn spawn_skill_context_window(
    mut commands: Commands,
    mut ev_skillcontext: EventReader<SkillContextEvent>,
    skill_q: Query<(&LabelName, Option<&Damage>, Option<&Block>), With<Skill>>,
    mut context_history: ResMut<ContextHistory>,
) {
    // TODO: complete with info text and window size + placements
    for ev in ev_skillcontext.iter() {
        if let Ok((name, dmg, block)) = skill_q.get(ev.skill_ent.0) {
            // https://github.com/IyesGames/iyes_loopless/blob/main/examples/menu.rs
            context_history.0.push(ev.skill_ent);
            if context_history.0.len() > 2 {
                context_history.0.remove(0);
            }
            if dmg.is_some() {}
            if block.is_some() {}
            commands
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(40.), Val::Px(40.)),
                        border: UiRect::all(Val::Px(2.)),
                        ..default()
                    },
                    ..default()
                })
                .insert(ContextWindow);
        }
    }
}

fn despawn_with<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for ent in query.iter() {
        commands.entity(ent).despawn_recursive();
    }
}
