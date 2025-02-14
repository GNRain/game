use bevy::ecs::system::Commands;
use bevy::core_pipeline::core_2d::Camera2dBundle;
use bevy::render::camera::ScalingMode;
pub const RESOLUTION: f32 = 16.0 / 9.0;

// Spawning the mmm mmm classic rpg camera
	pub fn fujifilm(mut commands: Commands) {
		let mut camera = Camera2dBundle::default();
		camera.projection.top = 1.0;
		camera.projection.bottom = -1.0;
		camera.projection.left = -1.0 * RESOLUTION;
		camera.projection.right = 1.0 * RESOLUTION;
		camera.projection.scaling_mode = ScalingMode::None;
		commands.spawn_bundle(camera);
	}
