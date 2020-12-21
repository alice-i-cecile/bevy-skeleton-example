use bevy::prelude::*;
use std::collections::HashMap;

enum Bone {
    Hip,
    Femur,
    Skull,
}

// Some sort of tree structure
struct SkeletonBlueprint;

impl SkeletonBlueprint {
    fn human() -> Self {
        SkeletonBlueprint
    }
}

fn main() {
    App::build()
        // Blueprint of Bone category connections
        .add_resource(SkeletonBlueprint::human())
        // Map of actual bones
        .add_resource(HashMap::<Bone, Entity>::new())
        .add_startup_system(spawn_bones.system())
        .add_startup_system(connect_bones.system())
        .run();
}

fn spawn_bones(
    commands: &mut Commands,
    blueprint: Res<SkeletonBlueprint>,
    skeleton_map: ResMut<HashMap<Bone, Entity>>,
) {
    for bone in blueprint.iter() {
        let current_bone = Bone::Skull;

        commands
            .spawn(SpriteBundle {
                transform: Transform::from_scale(Vec3::splat(0.75)),
                ..Default::default()
            })
            .with(current_bone);

        skeleton_map.insert(current_bone, commands.current_entity());
    }
}

fn connect_bones(
    query: Query<(Entity, &Bone, &Transform)>,
    blueprint: Res<SkeletonBlueprint>,
    skeleton_map: ResMut<HashMap<Bone, Entity>>,
) {
    for (entity, bone, transform) in query.iter() {
        // Determine which bone the current bone should connect to using the blueprint

        // Look up the entity ID for that bone using the skeleton_map

        // Then use push_children to enforce the appropriate relationship
    }
}
