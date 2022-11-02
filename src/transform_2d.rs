use bevy::prelude::*;

use crate::alt_systems::Transform2dPropagationDescriptor;

#[derive(Clone, Copy, Debug, PartialEq, Reflect, Component)]
#[reflect(Component, Default, PartialEq)]
pub struct Transform2d {
    pub translation: Vec2,
    pub z: f32,
    pub rotation: f32,
    pub scale: f32,
}

impl Transform2d {
    pub const IDENTITY: Self = Self {
        translation: Vec2::ZERO,
        z: 0.0,
        rotation: 0.0,
        scale: 1.0,
    };

    #[inline]
    #[must_use]
    pub const fn from_translation(translation: Vec2) -> Self {
        Self {
            translation,
            ..Self::IDENTITY
        }
    }

    #[inline]
    #[must_use]
    pub const fn from_translation3(translation: Vec3) -> Self {
        Self::from_xy(translation.x, translation.y).with_z(translation.z)
    }

    #[inline]
    #[must_use]
    pub const fn from_xy(x: f32, y: f32) -> Self {
        Self {
            translation: Vec2 { x, y },
            ..Self::IDENTITY
        }
    }

    #[inline]
    #[must_use]
    pub const fn from_xyz(x: f32, y: f32, z: f32) -> Self {
        Self::from_xy(x, y).with_z(z)
    }

    #[inline]
    #[must_use]
    pub const fn from_translation_z(translation: Vec2, z: f32) -> Self {
        Self {
            translation,
            z,
            ..Self::IDENTITY
        }
    }

    #[inline]
    #[must_use]
    pub const fn from_rotation(rotation: f32) -> Self {
        Self {
            rotation,
            ..Self::IDENTITY
        }
    }

    #[inline]
    #[must_use]
    pub const fn from_scale(scale: f32) -> Self {
        Self {
            scale,
            ..Self::IDENTITY
        }
    }

    #[inline]
    #[must_use]
    pub const fn from_z(z: f32) -> Self {
        Self {
            z: z,
            ..Self::IDENTITY
        }
    }

    #[inline]
    #[must_use]
    pub const fn with_translation(mut self, translation: Vec2) -> Self {
        self.translation = translation;
        self
    }

    #[inline]
    #[must_use]
    pub const fn with_rotation(mut self, rotation: f32) -> Self {
        self.rotation = rotation;
        self
    }

    #[inline]
    #[must_use]
    pub const fn with_z(mut self, z: f32) -> Self {
        self.z = z;
        self
    }

    #[inline]
    #[must_use]
    pub const fn with_scale(mut self, scale: f32) -> Self {
        self.scale = scale;
        self
    }

    #[inline]
    pub fn mul_vec2(&self, mut value: Vec2) -> Vec2 {
        value = self.scale * value;
        value = Mat2::from_angle(self.rotation) * value;
        value += self.translation;
        value
    }

    #[inline]
    #[must_use]
    pub fn mul_transform(&self, other: Self) -> Self {
        let translation = self.mul_vec2(other.translation);
        let z = self.z + other.z;
        let rotation = self.rotation + other.rotation;
        let scale = self.scale * other.scale;
        Self {
            translation,
            z,
            rotation,
            scale
        }
    }

    #[inline]
    pub fn rotate(&mut self, radians: f32) {
        self.rotation += radians;
    }
    
    #[inline]
    pub fn apply_scale(&mut self, scale_factor: f32) {
        self.scale *= scale_factor;
    }
}

impl Default for Transform2d {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl std::fmt::Display for Transform2d {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ T[{}, {}], Z[{}], R[{}], S[{}] }}",
            self.translation.x,
            self.translation.y,
            self.z,
            self.rotation,
            self.scale,
        )
    }
}

impl From<Transform2d> for Transform {
    fn from(transform_2: Transform2d) -> Self {
        Self {
            translation: transform_2.translation.extend(transform_2.z),
            rotation: Quat::from_rotation_z(transform_2.rotation),
            scale: Vec2::splat(transform_2.scale).extend(1.0),
        }
    }
}


#[derive(Clone, Copy, Debug, Default, PartialEq, Reflect, Component, Deref, DerefMut)]
#[reflect(Component, Default, PartialEq)]
pub struct GlobalTransform2d(pub Transform2d);

impl GlobalTransform2d {
    pub const IDENTITY: Self = Self(Transform2d::IDENTITY);

    #[must_use]
    #[inline]
    pub fn transform(&self) -> &Transform2d {
        &self.0
    }

    #[must_use]
    #[inline]
    pub fn translation(&self) -> Vec2 {
        self.transform().translation
    }

    #[must_use]
    #[inline]
    pub fn rotation(&self) -> f32 {
        self.transform().rotation
    }

    #[must_use]
    #[inline]
    pub fn scale(&self) -> f32 {
        self.transform().scale
    }    

    #[inline]
    #[must_use]
    pub fn propagate_transform(&self, other: Transform2d, desc: Option<&Transform2dPropagationDescriptor>) -> Self {
        if let Some(desc) = desc {   
            Self(Transform2d {
                translation: if desc.inherit_translation {
                    self.mul_vec2(other.translation)
                } else {
                    other.translation
                },
                z: if desc.inherit_z {
                    self.z + other.z
                } else {
                    other.z
                },
                rotation: if desc.inherit_rotation {
                    self.rotation + other.rotation
                } else {
                    other.rotation
                },
                scale: if desc.inherit_scale {
                    self.scale * other.scale
                } else {
                    other.scale
                }
            })
        } else {
            Self(self.mul_transform(other))
        }
    }
}

impl std::fmt::Display for GlobalTransform2d {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<Transform2d> for GlobalTransform2d {
    fn from(transform2d: Transform2d) -> Self {
        GlobalTransform2d(transform2d)
    }
}

impl From<GlobalTransform2d> for GlobalTransform {
    fn from(global_transform_2: GlobalTransform2d) -> Self {
        Self::from(Transform::from(global_transform_2.0))
    }
}