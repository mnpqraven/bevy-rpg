use crate::game::component::*;
use bevy::prelude::*;

pub struct SpritePlugin;
impl Plugin for SpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_ascii)
            .add_startup_system_set_to_stage(
                StartupStage::PostStartup,
                SystemSet::new()
                    .with_system(spawn_friendlies)
                    // TODO: conditional spawning later
                    .with_system(spawn_enemy),
            );
    }
}
/// bevy logo
/// TODO: can use this as placeholder skill icon
fn _load_single_ascii(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("icon.png"),
            ..default()
        })
        .insert(Player)
        .insert(IsMoving(false));
}
/// Resource
/// contains ascii sheets in assets folder,
/// can be accessed with `texture_atlas` in `SpriteSheetBundle`
pub struct AsciiSheet(Handle<TextureAtlas>);
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
pub fn spawn_friendlies(mut commands: Commands, ascii: Res<AsciiSheet>) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 1,
                ..default()
            },
            texture_atlas: ascii.0.clone(),
            transform: Transform::from_scale(Vec3::splat(8.)),
            ..default()
        })
        .insert(Player)
        .insert(LabelName {
            name: "Othi".to_string(),
        })
        .insert(Health { value: 100 })
        .insert(MaxHealth { value: 100 })
        .insert(Block::default())
        .insert(IsMoving(false));

    // ally for debug
    commands
        .spawn()
        .insert(Ally)
        .insert(LabelName {
            name: "Test ally".to_string(),
        })
        .insert(Health { value: 80 })
        .insert(MaxHealth { value: 80 })
        .insert(Mana { value: 30 })
        .insert(Block::default())
        .insert(IsMoving(false));
}

fn spawn_enemy(mut commands: Commands) {
    commands
        .spawn()
        .insert(Enemy)
        .insert(LabelName {
            name: "training dummy".to_string(),
        })
        .insert(Health { value: 40 })
        .insert(MaxHealth { value: 40 })
        .insert(Mana { value: 100 })
        .insert(Block { value: 4 });
    commands
        .spawn()
        .insert(Enemy)
        .insert(LabelName {
            name: "training dummy 2".to_string(),
        })
        .insert(Health { value: 100000000 })
        .insert(MaxHealth { value: 100000000 })
        .insert(Mana { value: 100 })
        .insert(Block::default());
}
