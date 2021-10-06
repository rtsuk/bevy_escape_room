use bevy::{prelude::*, scene::InstanceId};
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_flycam::{FlyCam, NoCameraPlayerPlugin};
use bevy_mod_raycast::{DefaultRaycastingPlugin, RayCastMesh, RayCastSource, RaycastSystem};
use std::collections::HashSet;
use wasm_bindgen::prelude::*;

const INVENTORY_TEXTURE_ID: u64 = 0;
const BLUE_STATUS_TEXTURE_ID: u64 = 1;
const GREEN_STATUS_TEXTURE_ID: u64 = 2;
const RED_STATUS_TEXTURE_ID: u64 = 3;
const BLACKLIGHT_FLASHLIGHT_TEXTURE_ID: u64 = 4;

#[derive(Default)]
pub struct EquippedInstance(Option<InstanceId>);

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut scene_spawner: ResMut<SceneSpawner>,
    mut scene_instance: ResMut<EquippedInstance>,
) {
    commands.spawn_scene(asset_server.load("er.gltf#Scene0"));
    commands
        .spawn_bundle(PerspectiveCameraBundle::default())
        .with_children(|parent| {
            let instance_id = scene_spawner
                .spawn_as_child(asset_server.load("pl.gltf#Scene0"), parent.parent_entity());
            scene_instance.0 = Some(instance_id);
        })
        .with_children(|parent| {
            parent.spawn_bundle(LightBundle {
                light: Light {
                    fov: f32::to_radians(10.0),
                    intensity: 200.0,
                    range: 1.0,
                    depth: 0.1..2.0,
                    ..Light::default()
                },
                ..Default::default()
            });
        })
        .insert(RayCastSource::<MyRaycastSet>::new_transform_empty())
        .insert(FlyCam)
        .insert(Name::new("cam".to_string()));
}

/// this component indicates what entities should rotate
pub struct Rotates;

pub fn rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<Rotates>>) {
    for mut transform in query.iter_mut() {
        *transform = Transform::from_rotation(Quat::from_rotation_y(
            (4.0 * std::f32::consts::PI / 20.0) * time.delta_seconds(),
        )) * *transform;
    }
}

fn load_assets(mut egui_context: ResMut<EguiContext>, assets: Res<AssetServer>) {
    let texture_handle = assets.load("inventory_slot.png");
    egui_context.set_egui_texture(INVENTORY_TEXTURE_ID, texture_handle);
    let texture_handle = assets.load("blue_ball_statue.png");
    egui_context.set_egui_texture(BLUE_STATUS_TEXTURE_ID, texture_handle);
    let texture_handle = assets.load("green_ball_statue.png");
    egui_context.set_egui_texture(GREEN_STATUS_TEXTURE_ID, texture_handle);
    let texture_handle = assets.load("red_ball_statue.png");
    egui_context.set_egui_texture(RED_STATUS_TEXTURE_ID, texture_handle);
    let texture_handle = assets.load("blacklight_flashlight.png");
    egui_context.set_egui_texture(BLACKLIGHT_FLASHLIGHT_TEXTURE_ID, texture_handle);
}

const ITEMS: &[&'static str] = &[
    "BallStatueRed",
    "BallStatueGreen",
    "BallStatueBlue",
    "BlacklightFlashlight",
    "",
];

fn ui_example(egui_context: Res<EguiContext>, player: Res<Player>) {
    let textures: Vec<_> = ITEMS
        .iter()
        .map(|name| {
            if player.inventory.contains(*name) {
                match *name {
                    "BallStatueRed" => RED_STATUS_TEXTURE_ID,
                    "BallStatueGreen" => GREEN_STATUS_TEXTURE_ID,
                    "BallStatueBlue" => BLUE_STATUS_TEXTURE_ID,
                    "BlacklightFlashlight" => BLACKLIGHT_FLASHLIGHT_TEXTURE_ID,
                    _ => INVENTORY_TEXTURE_ID,
                }
            } else {
                INVENTORY_TEXTURE_ID
            }
        })
        .collect();
    egui::Window::new("Inventory")
        .default_width(100.0)
        .show(egui_context.ctx(), |ui| {
            ui.vertical_centered(|ui| {
                for texture_id in textures {
                    ui.add(egui::widgets::Image::new(
                        egui::TextureId::User(texture_id),
                        [80.0, 80.0],
                    ));
                }
            });
        });
}

fn update_raycast_with_cursor(
    picking_camera_query: Query<&RayCastSource<MyRaycastSet>>,
    entities: Query<(Entity, &Pickable)>,
    mut target: ResMut<Target>,
) {
    if let Some(picking_camera) = picking_camera_query.iter().last() {
        if let Some((picked_entity, _intersection)) = picking_camera.intersect_top() {
            if let Ok(pickable) = entities.get_component::<Pickable>(picked_entity) {
                *target = Target(Some(NamedEntity {
                    name: pickable.0.to_string(),
                    entity: picked_entity,
                }));
            }
        }
    }
}

fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    target: ResMut<Target>,
    mut equipped: ResMut<Equipped>,
    mut player: ResMut<Player>,
    mut commands: Commands,
) {
    if keyboard_input.just_pressed(KeyCode::F1) {
        if let Some(target) = target.0.as_ref() {
            let target_name = target.name.to_string();
            *equipped = Equipped(Some(target_name.to_string()));
            player.inventory.insert(target_name);
            commands.entity(target.entity).despawn();
        }
    }
}

#[derive(Default)]
struct Done(bool);

struct BlacklightFlashlight;

enum StatueColor {
    Red,
    Green,
    Blue,
}

struct BallStatue(StatueColor);
struct MyRaycastSet;

#[derive(Debug)]
pub struct NamedEntity {
    name: String,
    entity: Entity,
}

#[derive(Default, Debug)]
struct Target(pub Option<NamedEntity>);

#[derive(Default, Debug)]
struct Equipped(pub Option<String>);

#[derive(Debug)]
struct Parent(pub Entity);

#[derive(Debug)]
struct Inventory(pub String);

#[derive(Debug)]
struct Pickable(pub String);

#[derive(Debug, Default)]
struct Player {
    equipped: String,
    inventory: HashSet<String>,
}

fn make_children_pickable(
    commands: &mut Commands,
    parent: &Entity,
    children: &Children,
    name: &str,
) {
    for c in children.iter() {
        commands
            .entity(*c)
            .insert(RayCastMesh::<MyRaycastSet>::default());
        commands.entity(*c).insert(Parent(*parent));
        commands.entity(*c).insert(Pickable(name.to_string()));
    }
}

fn tag_stuff(
    mut commands: Commands,
    mut done: ResMut<Done>,
    equipped_instance: Res<EquippedInstance>,
    entities: Query<(Entity, &Name, &Children, &Transform)>,
    scene_spawner: Res<SceneSpawner>,
) {
    if !done.0 {
        if let Some(instance_id) = equipped_instance.0 {
            if let Some(_entity_iter) = scene_spawner.iter_instance_entities(instance_id) {
                done.0 = true;
            }

            for (e, n, children, t) in entities.iter() {
                let name = n.as_str();
                match name {
                    "BlacklightFlashlight" => {
                        commands.entity(e).insert(BlacklightFlashlight);
                        make_children_pickable(&mut commands, &e, children, name);
                        ()
                    }
                    "BallStatueGreen" => {
                        commands.entity(e).insert(BallStatue(StatueColor::Green));
                        make_children_pickable(&mut commands, &e, children, name);
                    }
                    "BallStatueBlue" => {
                        commands.entity(e).insert(BallStatue(StatueColor::Blue));
                        make_children_pickable(&mut commands, &e, children, name);
                    }
                    "BallStatueRed" => {
                        commands.entity(e).insert(BallStatue(StatueColor::Red));
                        make_children_pickable(&mut commands, &e, children, name);
                    }
                    _ => {
                        if name.starts_with("Inv") {
                            commands.entity(e).insert(Inventory(n.to_string()));
                            commands.entity(e).insert(Visible {
                                is_visible: false,
                                is_transparent: false,
                            });
                            for c in children.iter() {
                                commands.entity(*c).insert(Inventory(n.to_string()));
                                commands.entity(*c).insert(Visible {
                                    is_visible: false,
                                    is_transparent: false,
                                });
                            }
                        } else if name == "BlacklightSpot" {
                            commands.spawn_bundle(LightBundle {
                                transform: *t,
                                ..Default::default()
                            });
                            dbg!(n);
                            dbg!(t);
                        }
                    }
                }
            }
        }
    }
}

fn show_equipped(mut entities: Query<(&Inventory, &mut Visible)>, player: Res<Player>) {
    for (i, mut v) in entities.iter_mut() {
        if &player.equipped == &i.0 {
            v.is_visible = true;
        }
    }
}

#[wasm_bindgen]
pub fn run() {
    let mut app = App::build();

    app.add_plugins(DefaultPlugins);
    app.add_plugin(NoCameraPlayerPlugin);
    app.add_plugin(EguiPlugin);
    app.init_resource::<Done>();
    app.init_resource::<Target>();
    app.init_resource::<Equipped>();
    app.init_resource::<EquippedInstance>();
    app.init_resource::<Player>();
    app.add_plugin(DefaultRaycastingPlugin::<MyRaycastSet>::default());
    app.add_startup_system(load_assets.system());
    app.add_startup_system(crate::setup.system());
    app.add_system(rotator_system.system());
    app.add_system(ui_example.system());
    app.add_system(keyboard_input_system.system());
    app.add_system(tag_stuff.system());
    app.add_system(show_equipped.system());
    app.add_system_to_stage(
        CoreStage::PostUpdate,
        update_raycast_with_cursor
            .system()
            .before(RaycastSystem::BuildRays),
    );

    // when building for Web, use WebGL2 rendering
    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);

    app.run();
}
