use bevy::prelude::*;
mod camera;
mod plant_management;

fn main()
{
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(camera::camera_zoom)
        .add_system(plant_management::bed_interact)
        .add_startup_system(plant_management::spawn_beds)
        .run();
}