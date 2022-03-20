/// This demo creates three strands of rope. The strand offsets to the left & right are supposed to
/// stay as is; while the center strand should get smaller every 2s via despawning of the lowest
/// entity in the strand.

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use nalgebra::Point2;

const BALLS_SIZE: f32 = 0.25;

const WORLD_BOUNDS_L: f32 = -10.;
const WORLD_BOUNDS_R: f32 = 10.;
const WORLD_BOUNDS_U: f32 = 10.;
const WORLD_BOUNDS_D: f32 = -10.;

#[derive(Default)]
struct Overview {
    balls: Vec<Entity>,
    next_removal_time: f64,
    head: Option<Entity>,
}

fn main() {
    App::new()
        .init_resource::<Overview>()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(Msaa::default())
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_world.system())
        .add_startup_system(setup_balls.system())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_system(pop_ball.system())
        .run();
}

fn setup_world(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.left = WORLD_BOUNDS_L;
    camera.orthographic_projection.right = WORLD_BOUNDS_R;
    camera.orthographic_projection.top = WORLD_BOUNDS_U;
    camera.orthographic_projection.bottom = WORLD_BOUNDS_D;
    camera.orthographic_projection.scaling_mode = bevy::render::camera::ScalingMode::None;
    camera.orthographic_projection.window_origin = bevy::render::camera::WindowOrigin::Center;

    commands.spawn().insert_bundle(camera);
}

fn setup_balls(mut commands: Commands, mut overview: ResMut<Overview>, time: Res<Time>) {
    {
        // setup left offset balls
        let mut tmp_overview = Overview::default();
        init_balls(&mut commands, &mut tmp_overview, 8, -5.);
    }

    {
        // setup right offset balls
        let mut tmp_overview = Overview::default();
        init_balls(&mut commands, &mut tmp_overview, 8, 5.);
    }

    init_balls(&mut commands, &mut overview, 16, 0.);
    overview.next_removal_time = time.seconds_since_startup() + 2.;
}

fn init_balls(commands: &mut Commands, overview: &mut Overview, total_balls: usize, pos_x: f32) {
    let size = Vec2::splat(BALLS_SIZE);
    let cuboid = ColliderShape::cuboid(BALLS_SIZE, BALLS_SIZE);

    let mut y = WORLD_BOUNDS_U - BALLS_SIZE;

    let entity = commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::BLUE,
                custom_size: Some(size.clone()),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static.into(),
            position: [pos_x, y].into(),
            ..RigidBodyBundle::default()
        })
        .insert_bundle(ColliderBundle {
            shape: cuboid.clone().into(),
            ..ColliderBundle::default()
        })
        .insert(RigidBodyPositionSync::Discrete)
        .id();

    overview.head = Some(entity);

    let mut parent_entity = entity;

    for _i in 0..total_balls {
        y -= BALLS_SIZE;

        let entity = commands
            .spawn()
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::RED,
                    custom_size: Some(size.clone()),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert_bundle(RigidBodyBundle {
                body_type: RigidBodyType::Dynamic.into(),
                position: [pos_x, y].into(),
                ..Default::default()
            })
            .insert_bundle(ColliderBundle {
                shape: cuboid.clone().into(),
                ..ColliderBundle::default()
            })
            .insert(RigidBodyPositionSync::Discrete)
            .id();

        // create joint between this and one above
        let joint = RevoluteJoint::new()
            .local_anchor1(Point2::origin())
            .local_anchor2(Point2::new(0., BALLS_SIZE))
            .motor_model(MotorModel::AccelerationBased);

        commands
            .spawn_bundle((JointBuilderComponent::new(joint, parent_entity, entity),))
            .id();

        overview.balls.push(entity);
        parent_entity = entity;
    }
}

fn pop_ball(mut commands: Commands, mut overview: ResMut<Overview>, time: Res<Time>) {
    if time.seconds_since_startup() > overview.next_removal_time {
        overview.next_removal_time = time.seconds_since_startup() + 2.;

        if let Some(entity) = overview.balls.pop() {
            println!("remove a ball");
            commands.entity(entity).despawn_recursive();
        }
    }
}
