use bevy::prelude::*;

use super::FontSheet;

pub const TEXT_COLOR: Color = Color::SILVER;
pub const NORMAL_BUTTON: Color = Color::rgb(0.5, 0.25, 0.5);
pub const HOVERED_BUTTON: Color = Color::rgb(0.35, 0.35, 0.35);
pub const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

// default skill label TextStyle struct
pub fn textstyle_skill_label(
    // only borrow the res cause font_handle will get iterated later on
    font_handle: &Res<FontSheet>
) -> TextStyle {
    TextStyle {
        font: font_handle.0.clone(),
        font_size: 20.,
        color: TEXT_COLOR,
    }
}
/// load fonts to FontSheet as resource
pub fn load_fonts(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let font_handle: Handle<Font> = asset_server.load("font.ttf");
    commands.insert_resource(FontSheet(font_handle));
}