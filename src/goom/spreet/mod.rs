use bevy::ecs::system::Commands;
use bevy::ecs::system::Res;
use bevy::ecs::system::ResMut;
use bevy::math::Vec2;
use bevy::math::Vec3;
use bevy::core::Name;
use bevy::asset::Assets;
use bevy::asset::AssetServer;
use bevy::asset::Handle;
use bevy::render::color::Color;
use bevy::sprite::TextureAtlas;
use bevy::sprite::TextureAtlasSprite;
use bevy::sprite::SpriteSheetBundle;
use bevy::transform::components::Transform;

// Where we add the code to use sprites
const FILENAME: &str = "glyph12.png";
pub struct NewGuy(Handle<TextureAtlas>);

pub fn load_sprite_sheet(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut materials: ResMut<Assets<TextureAtlas>>){
		let image = asset_server.load(FILENAME);
		let texture_atlas = TextureAtlas::from_grid_with_padding(
			image,
			Vec2::splat(9.5),
			4,
			13,
			Vec2::splat(1.9),
			Vec2::splat(2.5),
		);
	let atlas_handle =
		materials.add(texture_atlas);
	commands.insert_resource(NewGuy(atlas_handle));

	}

pub fn spawn_sprite(mut commands: Commands, spreets: Res<NewGuy>){
	let mut sprite = TextureAtlasSprite::new(6);
	sprite.color = Color::rgb(0.73, 0.114, 0.224);
	sprite.custom_size = Some(Vec2::splat(1.0));
	commands.spawn_bundle(SpriteSheetBundle{
		texture_atlas: spreets.0.clone(),
		sprite,
		transform: Transform{
			translation: Vec3::new(0.0,0.0,900.0,),
				..Default::default()
		},
		..Default::default()
	}).insert(Name::new("Spreet"));
}
