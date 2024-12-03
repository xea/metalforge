use bevy::asset::AssetServer;
use bevy::color::Color;
use bevy::color::palettes::basic::GRAY;
use bevy::prelude::{default, BorderRect, BuildChildren, ChildBuild, Commands, Res, SliceScaleMode, TextColor, TextFont, TextureSlicer};
use bevy_ui::{AlignItems, BackgroundColor, FlexDirection, JustifyContent, Node, UiRect, Val};
use bevy_ui::prelude::{Button, ImageNode, Text};
use bevy_ui::widget::NodeImageMode;
use crate::ui::menu::{MenuButtonAction, OnMainMenuScreen};

pub fn setup_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let image = asset_server.load("textures/panel-borders.png");

    let slicer = TextureSlicer {
        border: BorderRect::square(5.0),
        center_scale_mode: SliceScaleMode::Stretch,
        sides_scale_mode: SliceScaleMode::Stretch,
        max_corner_scale: 1.0
    };

    let button_text_font = TextFont {
        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
        font_size: 24.0,
        ..default()
    };

    let button_node = Node {
        width: Val::Px(220.),
        height: Val::Px(40.),
        // horizontally center child text
        justify_content: JustifyContent::Center,
        // vertically center child text
        align_items: AlignItems::Center,
        margin: UiRect::all(Val::Px(20.0)),
        ..default()
    };

    commands.spawn((Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        ..default()
    }, OnMainMenuScreen)).with_children(|parent| {
        // Create main menu title
        parent.spawn((
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(GRAY.into())
        ))
            .with_children(|parent| {
                // App title
                parent.spawn((
                    Text::new("Metalforge"),
                    TextFont { font_size: 48.0, ..default() },
                    //TextColor(CRIMSON.into())
                    // This node applies some padding to the box
                    Node { margin: UiRect::all(Val::Px(20.0)), ..default() }
                ));

                // Menu buttons
                parent.spawn((
                    Button,
                    ImageNode {
                        image: image.clone(),
                        image_mode: NodeImageMode::Sliced(slicer.clone()),
                        ..default()
                    },
                    button_node.clone(),
                    MenuButtonAction::ChooseSong
                ))
                    .with_child((
                        Text::new("Play song"),
                        button_text_font.clone(),
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    ));

                parent.spawn((
                    Button,
                    ImageNode {
                        image: image.clone(),
                        image_mode: NodeImageMode::Sliced(slicer.clone()),
                        ..default()
                    },
                    button_node.clone(),
                    MenuButtonAction::Settings
                ))
                    .with_child((
                        Text::new("Settings"),
                        button_text_font.clone(),
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    ));

                parent.spawn((
                    Button,
                    ImageNode {
                        image: image.clone(),
                        image_mode: NodeImageMode::Sliced(slicer.clone()),
                        ..default()
                    },
                    button_node.clone(),
                    MenuButtonAction::Quit
                ))
                    .with_child((
                        Text::new("Quit"),
                        button_text_font.clone(),
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    ));
            });
    });
}

