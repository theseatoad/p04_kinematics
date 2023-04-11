use bevy::{
    ecs::component,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
// pub struct IkPlugin;
// impl Plugin for IkPlugin {
//     fn build(&self, app: &mut App) {
//         app.add_system(update_ik_pos);
//     }
// }

#[derive(Component, Debug, Clone)]
struct Segment {
    a: Vec2,
    b: Vec2,
    len: f32,
    // must be stored as a radian
    angle: f32,
}

impl Segment {
    pub fn new(x: f32, y: f32, len: f32, angle: f32) -> Self {
        let mut seg = Segment {
            a: Vec2 { x, y },
            b: Vec2 { x: 0.0, y: 0.0 },
            len,
            angle,
        };
        seg.calculate_b();
        seg
    }

    pub fn calculate_b(&mut self) {
        let dx: f32 = self.len * self.angle.cos();
        let dy: f32 = self.len * self.angle.sin();
        self.b = Vec2 {
            x: self.a.x + dx,
            y: self.a.y + dy,
        };
    }

    // move root node
    pub fn move_root(&mut self, new_x: f32, new_y: f32) {}
}

#[derive(Bundle)]
pub struct SegmentBundle<M: bevy::sprite::Material2d> {
    segment: Segment,
    #[bundle]
    mesh_a: MaterialMesh2dBundle<M>,
}

impl<M: bevy::sprite::Material2d> SegmentBundle<M> {
    pub fn new(
        x: f32,
        y: f32,
        len: f32,
        angle: f32,
        material: Handle<M>,
        mesh: Mesh2dHandle,
    ) -> Self {
        let segment = Segment::new(x, y, len, angle);
        SegmentBundle {
            segment: segment.clone(),
            mesh_a: MaterialMesh2dBundle {
                mesh: mesh.0.clone().into(),
                material: material.clone().into(),
                transform: Transform {
                    translation: Vec3 {
                        x: segment.a.x.clone(),
                        y: segment.a.y.clone(),
                        z: 0.0,
                    },
                    rotation: Quat::from_rotation_z(angle),
                    scale: Vec3::splat(10.0),
                },
                ..default()
            },
        }
    }
}
