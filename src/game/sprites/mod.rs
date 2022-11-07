use crate::ecs::component::*;
use bevy::prelude::*;
use iyes_loopless::prelude::*;

use super::despawn_with;

pub struct SpritePlugin;
impl Plugin for SpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_animate_for_skills)
            .add_startup_system_to_stage(StartupStage::PreStartup, load_ascii)
            .add_startup_system_set_to_stage(
                StartupStage::PostStartup,
                SystemSet::new().with_system(spawn_env_allysp),
            )
            .add_enter_system_set(
                GameState::InCombat,
                ConditionSet::new()
                    .with_system(spawn_combat_allysp)
                    .with_system(spawn_combat_enemysp)
                    .into(),
            )
            .add_exit_system(GameState::InCombat, despawn_with::<CombatSprite>)
            .add_exit_system(GameState::OutOfCombat, despawn_with::<EnvSprite>);
    }
}
/// load the ascii sheets, this must be done in the system startup @`PreStartup` stage
pub fn load_ascii(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("ascii.png");
    let atlas = TextureAtlas::from_grid_with_padding(
        texture_handle,
        Vec2::splat(9.),
        16,
        16,
        Vec2::splat(2.),
        Vec2::splat(0.),
    );
    let texture_atlas_handle = texture_atlas.add(atlas);
    commands.insert_resource(AsciiSheet(texture_atlas_handle));
}

/// spawn ally sprites in the env state, movable
fn spawn_env_allysp(mut commands: Commands, ascii: Res<AsciiSheet>) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 5,
                ..default()
            },
            texture_atlas: ascii.0.clone(),
            transform: Transform::from_scale(Vec3::splat(8.)),
            ..default()
        })
        .insert(Player)
        .insert(LabelName("Othi".to_string()))
        .insert(IsMoving(false))
        .insert(EnvSprite);
}
/// Spawn friendly units (only entities)
pub fn spawn_combat_allysp(mut commands: Commands, ascii: Res<AsciiSheet>) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 2,
                ..default()
            },
            texture_atlas: ascii.0.clone(),
            transform: Transform::from_scale(Vec3::splat(8.)),
            ..default()
        })
        .insert(Player)
        .insert(LabelName("Othi".to_string()))
        .insert(Health(100))
        .insert(MaxHealth(100))
        .insert(Mana(100))
        .insert(MaxMana(100))
        .insert(Block::default())
        .insert(IsMoving(false))
        .insert(CombatSprite);

    // ally for debug
    commands
        .spawn()
        .insert(Ally)
        .insert(LabelName("Test ally".to_string()))
        .insert(Health(80))
        .insert(MaxHealth(80))
        .insert(Mana(30))
        .insert(Block::default())
        .insert(IsMoving(false))
        .insert(CombatSprite);
}

/// Spawn enemies in combat game state (with sprites)
fn spawn_combat_enemysp(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("icon.png"),
            transform: Transform {
                translation: Vec3 {
                    x: 200.,
                    y: 100.,
                    z: 0.,
                },
                scale: Vec3::splat(0.3),
                ..default()
            },
            ..default()
        })
        .insert(Enemy)
        .insert(LabelName("training dummy".to_string()))
        .insert(Health(40))
        .insert(MaxHealth(40))
        .insert(Mana(100))
        .insert(Block(4))
        .insert(CombatSprite);
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("icon.png"),
            transform: Transform {
                translation: Vec3 {
                    x: 200.,
                    y: -100.,
                    z: 0.,
                },
                scale: Vec3::splat(0.3),
                ..default()
            },
            ..default()
        })
        .insert(Enemy)
        .insert(LabelName("training dummy 2".to_string()))
        .insert(Health(9999))
        .insert(MaxHealth(9999))
        .insert(Mana(100))
        .insert(Block(2))
        .insert(CombatSprite);
}

/// Resource
/// contains ascii sheets in assets folder,
/// can be accessed with `texture_atlas` in `SpriteSheetBundle`
pub struct AsciiSheet(Handle<TextureAtlas>);

/// setup sprite animation (dev)
fn setup_animate_for_skills(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let handle: Handle<Image> = asset_server.load("gabe-idle-run.png");
    let texture_atlas = TextureAtlas::from_grid(handle, Vec2::new(24., 24.), 7, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(5.)),
            ..default()
        })
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)));
}
