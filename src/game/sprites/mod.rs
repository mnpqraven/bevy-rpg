use bevy::prelude::*;
use crate::game::component::*;
// TODO: refactor
pub struct SpritePlugin;
impl Plugin for SpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_ascii)
        ;
    }
}
/// bevy logo
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
pub fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>) {
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
}
