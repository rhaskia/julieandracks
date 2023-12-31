use bevy::prelude::*;
mod camera;
mod enemies;
mod plant_management;
use bevy::input::mouse::MouseWheel;
use bevy::sprite::collide_aabb::{collide, Collision};
use bevy::window::PrimaryWindow;
use enemies::*;
use plant_management::*;
//
// let KUMARA: Plant = Plant { name: "Kumara", sow: Season::Autumn, harvest: Season::Spring,
// ability: Ability::Tank(10), last_used: Stopwatch::new(), delay: 2.0, };
//
// const MANUKA: Plant = Plant { name: "Manuka", sow: Season::Autumn, harvest: Season::Spring}
// ability: Ability::Shooter(10.), last_used: Stopwatch::new(), delay: 2.0, };
//
// let PUHA: Plant = Plant { name: "Puha", sow: Season::Autumn, harvest: Season::Spring,
// ability: Ability::AOE(10.), last_used: Stopwatch::new(), delay: 2.0, };

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(ClearColor(Color::rgb(0.54, 0.69, 0.37)))
        .add_system(camera::camera_zoom)
        .add_system(bed_interact)
        .add_startup_system(plant_management::spawn_beds)
        .add_startup_system(enemies::spawn_enemies)
        .add_startup_system(setup)
        .add_system(move_enemies)
        .add_system(scroll_events)
        .add_system(manage_sticky)
        //.add_event::<CollisionEvent>()
        .run();
}

fn scroll_events(mut scroll_evr: EventReader<MouseWheel>, mut seed_bag_q: Query<&mut SeedBag>) {
    use bevy::input::mouse::MouseScrollUnit;

    let mut seed_bag: &mut SeedBag = &mut seed_bag_q.single_mut();
    for ev in scroll_evr.iter() {
        match ev.unit {
            MouseScrollUnit::Line => {
                println!(
                    "Scroll (line units): vertical: {}, horizontal: {}",
                    ev.y, ev.x
                );
                seed_bag.selected += (ev.y + 2.) as usize;
                seed_bag.selected %= 2;
            }
            MouseScrollUnit::Pixel => {
                println!(
                    "Scroll (pixel units): vertical: {}, horizontal: {}",
                    ev.y, ev.x
                );
                seed_bag.selected += (ev.y + 2.) as usize;
                seed_bag.selected %= 2;
            }
        }
    }
}
fn setup(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    let music = asset_server.load("assets/game thing.ogg");
    audio.play(music);
}
fn move_enemies(
    mut query: Query<(&Enemy, &mut Transform)>,
    //mut query_plant: Query<(&PlantBed, &mut Transform)>,
    time: Res<Time>,
) {
    'outer: for mut i in query.iter_mut() {
        if i.0.speed == 0.0 {
            continue;
        }
        i.1.translation.y += i.0.speed * time.delta_seconds();
    }
}

// will hover over beds and interact with them
pub(crate) fn bed_interact(
    mut bed_query: Query<(&mut PlantBed, &mut Transform, &mut Handle<Image>)>,
    mouse_pos: Query<&Window, With<PrimaryWindow>>,
    buttons: Res<Input<MouseButton>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut seed_bag_q: Query<&mut SeedBag>,
    asset_server: Res<AssetServer>,
) {
    // https://bevy-cheatbook.github.io/cookbook/cursor2world.html
    let (camera, camera_transform) = camera_q.single();
    let mut seed_bag: &mut SeedBag = &mut seed_bag_q.single_mut();

    let mut m_pos = match mouse_pos
        .single()
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        Some(a) => a,
        None => return,
    };

    // code to kill flys when you click on them
   /* pub(crate) fn enemy_kill(
        mut Enemy_query: Query<(&mut Enemy, &mut Transform, &mut Handle<Image>)>,
    ) {
        for mut Enemy in Enemy_query.iter_mut() {
            let mut Enemy_trans: &mut Transform = &mut Enemy.1;
            let mut fly_pos = Vec2::new(Enemy_trans.translation.x, Enemy_trans.translation.y);
            let mut rect2 = Rect::from_center_size(fly_pos, Vec2::ONE * 32.);
            if !rect2.contains(m_pos) {
                continue;
            }
        }
    }
            let mut Enemy: &mut Enemy = &mut Enemy.0;
            if buttons.just_pressed(MouseButton::Left) {
                Enemy.health -= 1;
                if Enemy.health == 0 {
                    Enemy.speed = 0.0;
                }
            }
        }
   */ 

    //code for planting
    for mut bed in bed_query.iter_mut() {
        let mut bed_trans: &mut Transform = &mut bed.1;
        let mut bed_pos = Vec2::new(bed_trans.translation.x, bed_trans.translation.y);
        bed_pos.y -= 32.;
        let rect = Rect::from_center_size(bed_pos, Vec2::ONE * 32.);

        bed_trans.scale = Vec3::new(1., 1., 1.);

        if !rect.contains(m_pos) {
            continue;
        }

        bed_trans.scale = Vec3::new(1.1, 1.1, 1.1);

        let mut plant_bed: &mut PlantBed = &mut bed.0;
        if buttons.just_pressed(MouseButton::Right) {
            plant_bed.plant = None;
            *bed.2 = asset_server.load("bed.png");
            println!("right clicked");
        }
        if buttons.just_pressed(MouseButton::Left) {
            // planting example -> put in func?

            if plant_bed.plant.is_none() {
                plant_bed.plant = Some(seed_bag.seeds[seed_bag.selected]);
                if seed_bag.selected == 0 {
                    *bed.2 = asset_server.load("manuka.png");
                } else {
                    *bed.2 = asset_server.load("puha.png");
                }
                println!("planted a plant");
            }
        }
    }

