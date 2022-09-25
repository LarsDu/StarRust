use super::*;
use super::super::components::*;

pub fn standard_bullet(
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    bullet_data: &BulletFiredEvent,
) -> BulletBundle{
    return BulletBundle{
        pbr_bundle: PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::TEAL.into()),
            transform: Transform::from_xyz(
                bullet_data.translation.x,
                bullet_data.translation.y,
                0.0,
            )
            .with_scale(Vec3 {
                x: 0.2,
                y: 0.2,
                z: 0.8,
            })
            .with_rotation(bullet_data.rotation),
            ..default() //FIXME
        },
        collider: Collider {
            hitmask: bullet_data.hitmask,
            rect: Vec2::new(0.8, 0.2), //FIXME
            damage: 1,
        },
        bullet: Bullet{}

    };
}