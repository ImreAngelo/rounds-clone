use bevy::prelude::*;
use bevy_lunex::UiLunexPlugins;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UiLunexPlugins);
        app.add_systems(Startup, setup);
    }
}


fn setup(
    mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // 1. Load the font
    // let font_handle = asset_server.load("fonts/promptfont/promptfont.ttf");

    // 2. Spawn text with the custom font
    commands.spawn((
        // Mesh2d(meshes.add(Rectangle::new(500.0, 500.0))),
        // MeshMaterial2d(materials.add(Color::srgb(0.1, 0.4, 0.1))),
        // Transform::IDENTITY,
        // children![(
        Text2d::new("⟶ Drop here ⊁"),
        TextColor(Color::BLACK),
        Pickable::IGNORE,
        Transform::from_translation(Vec3::Z),
        TextFont {
            // This font is loaded and will be used instead of the default font.
            font: asset_server.load("fonts/promptfont/promptfont.ttf"),
            font_size: 42.0,
            ..default()
        },
        // )],
    ));
}

// TextShadow::default(),
// // Set the justification of the Text
// TextLayout::new_with_justify(Justify::Center),
