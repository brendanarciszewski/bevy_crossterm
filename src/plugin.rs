use bevy::app::{App, Plugin, PluginGroup, PluginGroupBuilder};
use bevy::asset::{AddAsset, AssetPlugin, AssetStage};
use bevy::core::CorePlugin;
use bevy::diagnostic::DiagnosticsPlugin;
use bevy::hierarchy::HierarchyPlugin;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::transform::prelude::TransformPlugin;
use bevy::window::WindowPlugin;

use crossterm::event;

use super::{asset_loaders, components, cursor::Cursor, runner, systems};

pub const RENDER: &str = "render";

#[derive(Default)]
pub struct CrosstermPlugin;
impl Plugin for CrosstermPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Cursor::default())
            .insert_resource(components::PreviousEntityDetails::default())
            .insert_resource(components::EntitiesToRedraw::default())
            .insert_resource(components::PreviousWindowColors::default())
            .add_asset::<components::Sprite>()
            .add_asset::<components::StyleMap>()
            .init_asset_loader::<asset_loaders::SpriteLoader>()
            .init_asset_loader::<asset_loaders::StyleMapLoader>()
            .add_event::<event::KeyEvent>()
            .add_event::<event::MouseEvent>()
            .set_runner(runner::crossterm_runner)
            // Systems and stages
            // This must be before CoreStage::Last because change tracking is cleared then, but AssetEvents are
            // published after PostUpdate. The timing for all these things is pretty delicate
            .add_stage_after(AssetStage::AssetEvents, RENDER, SystemStage::parallel())
            .add_system(systems::add_previous_position)
            .add_system_set_to_stage(
                RENDER,
                SystemSet::new()
                    .with_system(systems::calculate_entities_to_redraw)
                    .with_system(systems::crossterm_render.after(systems::calculate_entities_to_redraw))
                    .with_system(systems::update_previous_position.after(systems::crossterm_render)),
            );
    }
}

pub struct DefaultCrosstermPlugins;

impl PluginGroup for DefaultCrosstermPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        // The crossterm plugin needs many of bevy's plugins, or there
        // will be runtime errors.  Each of the various bevy packages used
        // must include their bevy plugins.  Log and Diagnostics plugins
        // are added primarily just for extra debug information.
        group.add(LogPlugin::default());
        group.add(CorePlugin::default());
        group.add(TransformPlugin::default());
        group.add(HierarchyPlugin::default());
        group.add(DiagnosticsPlugin::default());
        group.add(WindowPlugin::default());
        group.add(AssetPlugin::default());
        // Add crossterm plugin last
        group.add(CrosstermPlugin::default());
    }
}
