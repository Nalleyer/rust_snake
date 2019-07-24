use amethyst::{
  assets::{AssetStorage, Handle, Loader},
  core::Transform,
  prelude::*,
  renderer::{ImageFormat, Texture},
};

pub fn load_texture<N>(name: N, world: &mut World) -> Handle<Texture>
where
   N: Into<String>,
{
    let loader = world.read_resource::<Loader>();
    loader.load(
        name,
        ImageFormat::default(),
        (),
        &world.read_resource::<AssetStorage<Texture>>(),
    )
}