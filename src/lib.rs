use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_flycam::{FlyCam, NoCameraPlayerPlugin};
use bevy_mod_raycast::{DefaultRaycastingPlugin, RayCastMesh, RayCastSource, RaycastSystem};
use wasm_bindgen::prelude::*;

const INVENTORY_TEXTURE_ID: u64 = 0;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(PerspectiveCameraBundle::default())
        .insert(RayCastSource::<MyRaycastSet>::new_transform_empty())
        .insert(FlyCam)
        .insert(Name::new("cam".to_string()));
    commands.spawn_scene(asset_server.load("er.gltf#Scene0"));
    commands
        .spawn_bundle(LightBundle {
            transform: Transform::from_xyz(3.0, 5.0, 3.0),
            ..Default::default()
        })
        .insert(Rotates);
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(0.0, 0.0, 20.0),
        ..Default::default()
    });
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
}

fn ui_example(egui_context: Res<EguiContext>) {
    egui::Window::new("Inventory")
        .default_width(100.0)
        .show(egui_context.ctx(), |ui| {
            ui.vertical_centered(|ui| {
                for _ in 0..5 {
                    ui.add(egui::widgets::Image::new(
                        egui::TextureId::User(INVENTORY_TEXTURE_ID),
                        [80.0, 80.0],
                    ));
                }
            });
        });
}

fn update_raycast_with_cursor(
    picking_camera_query: Query<&RayCastSource<MyRaycastSet>>,
    entities: Query<(Entity, &Parent)>,
    mut target: ResMut<Target>,
) {
    if let Some(picking_camera) = picking_camera_query.iter().last() {
        if let Some((picked_entity, _intersection)) = picking_camera.intersect_top() {
            if let Ok(parent) = entities.get_component::<Parent>(picked_entity) {
                target.0 = Some(parent.0);
            } else {
                target.0 = None;
            }
        }
    }
}

fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    target: ResMut<Target>,
    mut equipped: ResMut<Equipped>,
) {
    if keyboard_input.just_pressed(KeyCode::F1) {
        if let Some(target) = target.0.as_ref() {
            *equipped = Equipped(Some(*target));
        }
    }
}

fn calculate_equipped_transform_system(
    query: Query<(&FlyCam, &Transform)>,
    mut equipped_transform: ResMut<EquippedTransform>,
) {
    let (_, camera_transform) = query.single().expect("camera");
    let offset = Vec3::new(1.0, 0.0, -8.0);

    let matrix =
        Mat4::from_rotation_translation(camera_transform.rotation, camera_transform.translation);
    let transformed_offset = matrix.transform_vector3(offset);

    let mut calculated_equipped_transform = *camera_transform;
    calculated_equipped_transform.translation += transformed_offset;
    dbg!(&camera_transform);
    dbg!(&calculated_equipped_transform);
    *equipped_transform = EquippedTransform(calculated_equipped_transform);
}

fn show_equipped_system(
    equipped: ResMut<Equipped>,
    equipped_transform: ResMut<EquippedTransform>,
    mut query: Query<&mut Transform>,
) {
    if let Some(equipped) = equipped.0.as_ref() {
        let mut transform = query.get_mut(*equipped).expect("equipped");
        *transform = equipped_transform.0.into();
    }
}

#[derive(Default)]
struct Done(bool);

struct BlacklightFlashlight;
struct BallStatueGreen;
struct MyRaycastSet;

#[derive(Default, Debug)]
struct Target(pub Option<Entity>);

#[derive(Default, Debug)]
struct Equipped(pub Option<Entity>);

#[derive(Default, Debug)]
struct EquippedTransform(pub Transform);

#[derive(Debug)]
struct Parent(pub Entity);

fn make_children_pickable(commands: &mut Commands, parent: &Entity, children: &Children) {
    for c in children.iter() {
        commands
            .entity(*c)
            .insert(RayCastMesh::<MyRaycastSet>::default());
        commands.entity(*c).insert(Parent(*parent));
    }
}

fn tag_stuff(
    mut commands: Commands,
    mut done: ResMut<Done>,
    entities: Query<(Entity, &Name, &Children)>,
) {
    if !done.0 {
        for (e, n, children) in entities.iter() {
            match n.as_str() {
                "BlacklightFlashlight" => {
                    println!("BlacklightFlashlight");
                    commands.entity(e).insert(BlacklightFlashlight);
                    make_children_pickable(&mut commands, &e, children);
                    ()
                }
                "BallStatueGreen" => {
                    println!("BallStatueGreen");
                    commands.entity(e).insert(BallStatueGreen);
                    make_children_pickable(&mut commands, &e, children);
                }
                _ => {}
            }

            done.0 = true;
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
    app.init_resource::<EquippedTransform>();
    app.add_plugin(DefaultRaycastingPlugin::<MyRaycastSet>::default());
    app.add_startup_system(load_assets.system());
    app.add_startup_system(crate::setup.system());
    app.add_system(rotator_system.system());
    app.add_system(ui_example.system());
    app.add_system(keyboard_input_system.system());
    app.add_system(tag_stuff.system());
    app.add_system(calculate_equipped_transform_system.system());
    app.add_system(show_equipped_system.system());
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
