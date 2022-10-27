use bevy::{log::LogSettings, prelude::*};

// modules
mod game;
use game::component::{Direction, *}; // fix bevy name conflict
mod menu;
mod skills;

fn main() {
    App::new()
        // .add_plugins(DefaultPlugins)
        .insert_resource(WindowDescriptor {
            title: "bevy-rpg".to_string(),
            width: 800.,
            height: 600.,
            ..default()
        })
        .insert_resource(LogSettings {
            filter: "info,wgpu_core=warn,wgpu_hal=warn,othirpg=debug".into(),
            level: bevy::log::Level::DEBUG,
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(crate::menu::MenuPlugin)
        .add_plugin(crate::skills::SkillPlugin)
        .add_plugin(crate::game::GamePlugin)
        .add_startup_system_to_stage(StartupStage::PreStartup, load_ascii)
        .add_system(bevy::window::close_on_esc)
        .add_startup_system_set_to_stage(
            StartupStage::PostStartup,
            SystemSet::new()
                // unit archetype
                .with_system(setup)
                .with_system(spawn_player)
                .with_system(spawn_enemy),
        )
        .add_system_set(SystemSet::new().with_system(logic_input_movement))
        // TODO: turn base queueing
        // .add_system(get_player_name)
        // .add_system_set(
        //     SystemSet::new()
        //         .with_system(calc_block)
        //         .with_system(calc_damage.after(calc_block))
        //         .with_system(calc_heal),
        // )
        .run();
}


// ENTITY =====================================================================
// RESOURCE ===================================================================
#[derive(Bundle)]
struct CombatStatBundle {
    base_health: Health,
    base_damage: Damage,
    base_block: Block,
}
// is this actually correct, not very sure
#[derive(Bundle)]
struct CharacterBundle {
    name: LabelName,
    current_health: Health,
    current_block: Block,
}

// SYSTEM =====================================================================
fn setup(mut commands: Commands) {}
/// bevy logo
fn load_single_ascii(mut commands: Commands, asset_server: Res<AssetServer>) {
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
struct AsciiSheet(Handle<TextureAtlas>);
/// load the ascii sheets, this must be done in the system startup @`PreStartup` stage
fn load_ascii(
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
fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>) {
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
fn spawn_enemy(mut commands: Commands) {
    commands
        .spawn()
        .insert(Enemy)
        .insert(LabelName {
            name: "training dummy".to_string(),
        })
        .insert(Health { value: 40 })
        .insert(MaxHealth { value: 40 })
        .insert(Block::default());
}

fn get_player_name(mut players: Query<(&Health, &LabelName, Option<&Player>)>) {
    for (health, name, player) in players.iter_mut() {
        println!(
            "STARTUP: entity {} has {} health points",
            name.name, health.value
        );
        if let Some(_) = player {
            // user controlled
            println!(
                "STARTUP: entity {} is special because it is controlled by a player",
                name.name
            );
        }
    }
}
fn calc_block(
    mut player: Query<(&LabelName, &Health, &mut Block), With<Player>>,
    skills: Query<(&LabelName, &Block), (With<Skill>, Without<Player>)>,
    // Without for disjoined query
) {
    let (player_name, player_health, mut player_block) = player.single_mut();
    print_player_stat(&player_name, &player_block, &player_health);
    for (skill_name, block) in skills.iter() {
        println!(
            "{}'s using defensive skill {} ({})",
            player_name.name, skill_name.name, block.value
        );
        player_block.value += block.value;
    }
    print_player_stat(&player_name, &player_block, &player_health);
    println!("====================");
}
fn calc_damage(
    mut player: Query<(&LabelName, &mut Block, &mut Health), With<Player>>,
    skills: Query<(&LabelName, &Damage), (With<Skill>, Without<Player>)>,
) {
    let (player_name, mut player_block, mut player_health) = player.single_mut();
    print_player_stat(&player_name, &player_block, &player_health);
    for (skill_name, damage) in skills.iter() {
        println!(
            "{}'s receiving offensive skill {} ({})",
            player_name.name, skill_name.name, damage.value
        );
        player_health.value -= damage.value - player_block.value;
        match damage.value > player_block.value {
            true => player_block.value = 0,
            false => player_block.value -= damage.value,
        }
    }
    print_player_stat(&player_name, &player_block, &player_health);
    println!("====================");
}
fn calc_heal(
    mut player: Query<(&LabelName, &Block, &mut Health, &MaxHealth), With<Player>>,
    skills: Query<(&LabelName, &Heal), (With<Skill>, Without<Player>)>,
) {
    let (player_name, player_block, mut player_health, player_max_health) = player.single_mut();
    print_player_stat(&player_name, &player_block, &player_health);
    for (skill_name, heal) in skills.iter() {
        println!(
            "{}'s casting healing skill {} ({})",
            player_name.name, skill_name.name, heal.value
        );

        // can only heal when alive
        // NOTE: potential refactor
        if player_health.value > 1 {
            player_health.value = match player_health.value + heal.value > player_max_health.value {
                true => player_max_health.value,
                false => player_health.value + heal.value,
            };
        }
    }
    print_player_stat(&player_name, &player_block, &player_health);
    println!("====================");
}

fn print_player_stat(name: &LabelName, current_block: &Block, current_health: &Health) {
    println!(
        "system: {}'s health: {}, block: {}",
        name.name, current_health.value, current_block.value
    );
}
fn _game_over() {
    // triggers GameOverEvent if hp drops to 0
}
fn _test_loop() {
    // player getting attacked by "Attack"
    // automatically casts "Bandaid" when health is < 50%
    // automatically casts "Block" when health is < 25%
}

/// transform sprite based on position in the last frame
/// TODO: refactor
fn logic_input_movement(
    time: Res<Time>,
    mut sprite_pos: Query<(&mut IsMoving, &mut Transform), With<Player>>,
    input: Res<Input<KeyCode>>,
) {
    for (mut is_moving, mut transform) in &mut sprite_pos {
        // let mut dir: Direction = Direction::Down; // facing the screen
        let dir: crate::game::component::Direction = match input.get_pressed().next() {
            Some(KeyCode::W) => {
                is_moving.0 = true;
                Direction::Up
            }
            Some(KeyCode::R) => {
                is_moving.0 = true;
                Direction::Down
            }
            Some(KeyCode::A) => {
                is_moving.0 = true;
                Direction::Left
            }
            Some(KeyCode::S) => {
                is_moving.0 = true;
                Direction::Right
            }
            _ => {
                is_moving.0 = false;
                Direction::Down
            } // stops moving
        };
        if is_moving.0 {
            match dir {
                Direction::Up => transform.translation.y += 150. * time.delta_seconds(),
                Direction::Down => transform.translation.y -= 150. * time.delta_seconds(),
                Direction::Left => transform.translation.x -= 150. * time.delta_seconds(),
                Direction::Right => transform.translation.x += 150. * time.delta_seconds(),
            }
        }
    }
}
