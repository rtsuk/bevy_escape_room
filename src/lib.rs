use bevy::{prelude::*, scene::InstanceId};
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_flycam::{FlyCam, NoCameraPlayerPlugin};
use bevy_mod_raycast::{DefaultRaycastingPlugin, RayCastMesh, RayCastSource, RaycastSystem};
use std::collections::HashSet;
use wasm_bindgen::prelude::*;

enum TextureIds {
    BlacklightFlashlightSelTextureId,
    BlacklightFlashlightTextureId,
    BluePosterTextureId,
    BluePosterUvTextureId,
    BlueStatueSelTextureId,
    BlueStatueTextureId,
    GreenPosterTextureId,
    GreenPosterUvTextureId,
    GreenStatueSelTextureId,
    GreenStatueTextureId,
    InventorySelTextureId,
    InventoryTextureId,
    RedPosterTextureId,
    RedPosterUvTextureId,
    RedStatueSelTextureId,
    RedStatueTextureId,
}

impl Into<u64> for TextureIds {
    fn into(self) -> u64 {
        self as u64
    }
}

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
        .insert(RayCastSource::<PickingRaycastSet>::new_transform_empty())
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
    egui_context.set_egui_texture(TextureIds::InventoryTextureId.into(), texture_handle);
    let texture_handle = assets.load("inventory_slot_sel.png");
    egui_context.set_egui_texture(TextureIds::InventorySelTextureId.into(), texture_handle);
    let texture_handle = assets.load("blue_ball_statue_sel.png");
    egui_context.set_egui_texture(TextureIds::BlueStatueSelTextureId.into(), texture_handle);
    let texture_handle = assets.load("blue_ball_statue.png");
    egui_context.set_egui_texture(TextureIds::BlueStatueTextureId.into(), texture_handle);
    let texture_handle = assets.load("green_ball_statue_sel.png");
    egui_context.set_egui_texture(TextureIds::GreenStatueSelTextureId.into(), texture_handle);
    let texture_handle = assets.load("green_ball_statue.png");
    egui_context.set_egui_texture(TextureIds::GreenStatueTextureId.into(), texture_handle);
    let texture_handle = assets.load("red_ball_statue.png");
    egui_context.set_egui_texture(TextureIds::RedStatueTextureId.into(), texture_handle);
    let texture_handle = assets.load("red_ball_statue_sel.png");
    egui_context.set_egui_texture(TextureIds::RedStatueSelTextureId.into(), texture_handle);
    let texture_handle = assets.load("blacklight_flashlight_sel.png");
    egui_context.set_egui_texture(
        TextureIds::BlacklightFlashlightSelTextureId.into(),
        texture_handle,
    );
    let texture_handle = assets.load("blacklight_flashlight.png");
    egui_context.set_egui_texture(
        TextureIds::BlacklightFlashlightTextureId.into(),
        texture_handle,
    );
    let texture_handle = assets.load("PosterBlueUV.png");
    egui_context.set_egui_texture(TextureIds::BluePosterTextureId.into(), texture_handle);
    let texture_handle = assets.load("PosterBlueUVBlackLight.png");
    egui_context.set_egui_texture(TextureIds::BluePosterUvTextureId.into(), texture_handle);
    let texture_handle = assets.load("PosterRedUV.png");
    egui_context.set_egui_texture(TextureIds::RedPosterTextureId.into(), texture_handle);
    let texture_handle = assets.load("PosterRedUVBlackLight.png");
    egui_context.set_egui_texture(TextureIds::RedPosterUvTextureId.into(), texture_handle);
    let texture_handle = assets.load("PosterGreenUV.png");
    egui_context.set_egui_texture(TextureIds::GreenPosterTextureId.into(), texture_handle);
    let texture_handle = assets.load("PosterGreenUVBlackLight.png");
    egui_context.set_egui_texture(TextureIds::GreenPosterUvTextureId.into(), texture_handle);
}

const ITEMS: &[&'static str] = &[
    "InvBallStatueRed",
    "InvBallStatueGreen",
    "InvBallStatueBlue",
    "InvBlacklightFlashlight",
    "",
];

fn maybe_equipped(
    equipped: &str,
    name: &str,
    equipped_id: TextureIds,
    not_equipped_id: TextureIds,
) -> u64 {
    if equipped == name {
        equipped_id.into()
    } else {
        not_equipped_id.into()
    }
}

fn ui_example(egui_context: Res<EguiContext>, player: Res<Player>) {
    let textures: Vec<_> = ITEMS
        .iter()
        .enumerate()
        .map(|(index, name)| {
            if player.inventory.contains(*name) {
                match *name {
                    "InvBallStatueRed" => maybe_equipped(
                        player.equipped_name(),
                        *name,
                        TextureIds::RedStatueSelTextureId,
                        TextureIds::RedStatueTextureId,
                    ),
                    "InvBallStatueGreen" => maybe_equipped(
                        player.equipped_name(),
                        *name,
                        TextureIds::GreenStatueSelTextureId,
                        TextureIds::GreenStatueTextureId,
                    ),
                    "InvBallStatueBlue" => maybe_equipped(
                        player.equipped_name(),
                        *name,
                        TextureIds::BlueStatueSelTextureId,
                        TextureIds::BlueStatueTextureId,
                    ),
                    "InvBlacklightFlashlight" => maybe_equipped(
                        player.equipped_name(),
                        *name,
                        TextureIds::BlacklightFlashlightSelTextureId,
                        TextureIds::BlacklightFlashlightTextureId,
                    ),
                    _ => TextureIds::InventoryTextureId.into(),
                }
            } else {
                if index == player.equipped {
                    TextureIds::InventorySelTextureId
                } else {
                    TextureIds::InventoryTextureId
                }
                .into()
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
    picking_camera_query: Query<&RayCastSource<PickingRaycastSet>>,
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
    mut player: ResMut<Player>,
    mut commands: Commands,
) {
    if keyboard_input.just_pressed(KeyCode::F1) {
        if let Some(target) = target.0.as_ref() {
            let target_name = target.name.to_string();
            let inv_name = format!("Inv{}", target_name);
            player.inventory.insert(inv_name.clone());
            player.equipped = Player::item_index(&inv_name);
            commands.entity(target.entity).despawn();
        }
    }

    if keyboard_input.just_pressed(KeyCode::Down) {
        player.equipped = match player.equipped {
            4 => 0,
            _ => player.equipped + 1,
        };
    } else if keyboard_input.just_pressed(KeyCode::Up) {
        player.equipped = match player.equipped {
            0 => 4,
            _ => player.equipped - 1,
        };
    }
}

#[derive(Default)]
struct Done(bool);

struct BlacklightFlashlight;

#[derive(Debug, Clone, Copy)]
enum StatueColor {
    Red,
    Green,
    Blue,
}

struct BallStatue(StatueColor);
struct PickingRaycastSet;
struct BlacklightRaycastSet;

#[derive(Debug)]
pub struct NamedEntity {
    name: String,
    entity: Entity,
}

#[derive(Default, Debug)]
struct Target(pub Option<NamedEntity>);

#[derive(Default, Debug)]
struct BlacklightTarget(pub Option<NamedEntity>);

#[derive(Default, Debug)]
struct Equipped(pub Option<String>);

#[derive(Debug)]
struct Parent(pub Entity);

#[derive(Debug)]
struct Inventory(pub String);

#[derive(Debug)]
struct Pickable(pub String);

#[derive(Debug)]
struct Poster(pub String, pub StatueColor, pub bool);

#[derive(Debug)]
struct Player {
    equipped: usize,
    inventory: HashSet<String>,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            equipped: 4,
            inventory: HashSet::default(),
        }
    }
}

impl Player {
    pub fn equipped_name(&self) -> &'static str {
        ITEMS[self.equipped]
    }

    pub fn item_index(name: &str) -> usize {
        ITEMS
            .iter()
            .enumerate()
            .find_map(|(index, item_name)| {
                if *item_name == name {
                    Some(index)
                } else {
                    None
                }
            })
            .unwrap_or(4)
    }
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
            .insert(RayCastMesh::<PickingRaycastSet>::default());
        commands.entity(*c).insert(Parent(*parent));
        commands.entity(*c).insert(Pickable(name.to_string()));
    }
}

fn make_children_posters(
    commands: &mut Commands,
    children: &Children,
    name: &str,
    color: StatueColor,
    uv: bool,
) {
    for c in children.iter() {
        commands
            .entity(*c)
            .insert(Poster(name.to_string(), color, uv));
        if !uv {
            commands
                .entity(*c)
                .insert(RayCastMesh::<BlacklightRaycastSet>::default());
        }
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
                    "RedPoster" => {
                        make_children_posters(
                            &mut commands,
                            children,
                            name,
                            StatueColor::Red,
                            false,
                        );
                    }
                    "GreenPoster" => {
                        make_children_posters(
                            &mut commands,
                            children,
                            name,
                            StatueColor::Green,
                            false,
                        );
                    }
                    "BluePoster" => {
                        make_children_posters(
                            &mut commands,
                            children,
                            name,
                            StatueColor::Blue,
                            false,
                        );
                    }
                    "RedPosterUV" => {
                        make_children_posters(
                            &mut commands,
                            children,
                            "RedPoster",
                            StatueColor::Red,
                            true,
                        );
                    }
                    "GreenPosterUV" => {
                        make_children_posters(
                            &mut commands,
                            children,
                            "GreenPoster",
                            StatueColor::Green,
                            true,
                        );
                    }
                    "BluePosterUV" => {
                        make_children_posters(
                            &mut commands,
                            children,
                            "BluePoster",
                            StatueColor::Blue,
                            true,
                        );
                    }
                    _ => {
                        if name.starts_with("Inv") {
                            if name == "InvBlacklightFlashlight" {
                                commands.entity(e).insert(
                                    RayCastSource::<BlacklightRaycastSet>::new_transform(
                                        Mat4::from_rotation_y(2.35619),
                                    ),
                                );
                            }
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
                        }
                    }
                }
            }
        }
    }
}

fn show_equipped(mut entities: Query<(&Inventory, &mut Visible)>, player: Res<Player>) {
    for (i, mut v) in entities.iter_mut() {
        v.is_visible = player.inventory.contains(&i.0) && player.equipped_name() == &i.0;
    }
}

fn update_posters(
    mut _commands: Commands,
    mut entities: Query<(Entity, &Poster, &mut Visible)>,
    target: ResMut<BlacklightTarget>,
    player: Res<Player>,
) {
    let blacklight_equipped = player.equipped_name() == "InvBlacklightFlashlight";
    let target_poster_name = target
        .0
        .as_ref()
        .map(|target| target.name.clone())
        .unwrap_or(String::from(""));
    for (_i, p, mut v) in entities.iter_mut() {
        if blacklight_equipped {
            if p.0 == target_poster_name {
                v.is_visible = p.2;
            } else {
                v.is_visible = !p.2;
            }
        } else {
            v.is_visible = !p.2;
        }
    }
}

fn shine_on_poster(
    blacklight_camera_query: Query<&RayCastSource<BlacklightRaycastSet>>,
    entities: Query<(Entity, &Poster)>,
    mut target: ResMut<BlacklightTarget>,
) {
    if let Some(blacklight_camera) = blacklight_camera_query.iter().last() {
        if let Some((illuminated_entity, _intersection)) = blacklight_camera.intersect_top() {
            if let Ok(poster) = entities.get_component::<Poster>(illuminated_entity) {
                *target = BlacklightTarget(Some(NamedEntity {
                    name: poster.0.to_string(),
                    entity: illuminated_entity,
                }));
            }
        }
    }
}

#[wasm_bindgen]
pub fn run() {
    let mut app = App::build();
    #[cfg(feature = "bundle")]
    {
        use bevy::asset::AssetServerSettings;
        app.insert_resource(AssetServerSettings {
            asset_folder: "../Resources/assets".to_string(),
        });
    }
    app.add_plugins(DefaultPlugins);
    app.add_plugin(NoCameraPlayerPlugin);
    app.add_plugin(EguiPlugin);
    app.init_resource::<Done>();
    app.init_resource::<Target>();
    app.init_resource::<BlacklightTarget>();
    app.init_resource::<Equipped>();
    app.init_resource::<EquippedInstance>();
    app.init_resource::<Player>();
    app.add_plugin(DefaultRaycastingPlugin::<PickingRaycastSet>::default());
    app.add_plugin(DefaultRaycastingPlugin::<BlacklightRaycastSet>::default());
    app.add_startup_system(load_assets.system());
    app.add_startup_system(crate::setup.system());
    app.add_system(rotator_system.system());
    app.add_system(ui_example.system());
    app.add_system(keyboard_input_system.system());
    app.add_system(tag_stuff.system());
    app.add_system(show_equipped.system());
    app.add_system(update_posters.system());
    app.add_system_to_stage(
        CoreStage::PostUpdate,
        update_raycast_with_cursor
            .system()
            .before(RaycastSystem::BuildRays),
    );
    app.add_system_to_stage(
        CoreStage::PostUpdate,
        shine_on_poster.system().before(RaycastSystem::BuildRays),
    );

    // when building for Web, use WebGL2 rendering
    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);

    app.run();
}
