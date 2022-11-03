use bevy::math::vec2;
use bevy::math::Affine3A;
use bevy::math::Mat3A;
use bevy::math::Vec3A;
use bevy::prelude::*;

#[derive(Component, Clone, Copy, Debug, Eq, PartialEq, Reflect)]
#[reflect(Component, Default, PartialEq)]
pub struct Propagate(pub u8);

impl Default for Propagate {
    fn default() -> Self {
        Self::ALL
    }
}

impl Propagate {
    pub const NOTHING: Self = Self(0);
    pub const TRANSLATION: Self = Self(1);
    pub const DEPTH: Self = Self(2);
    pub const ROTATION: Self = Self(4);
    pub const SCALE: Self = Self(8);
    pub const ALL: Self = Self(15);

    #[must_use]
    #[inline]
    pub const fn inherits(self, rule: Self) -> bool {
        self.0 & rule.0 != 0
    }
}

impl std::ops::BitAnd for Propagate {
    type Output = Propagate;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl std::ops::BitAndAssign for Propagate {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}
impl std::ops::BitOr for Propagate {
    type Output = Propagate;

    #[inline]
    #[must_use]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitOrAssign for Propagate {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Reflect, Component)]
#[reflect(Component, Default, PartialEq)]
pub struct Transform2 {
    pub translation: Vec2,
    pub depth: f32,
    pub rotation: f32,
    pub scale: f32,
}

impl Transform2 {
    pub const IDENTITY: Self = Self {
        translation: Vec2::ZERO,
        depth: 0.0,
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
        Self::from_xy(translation.x, translation.y).with_depth(translation.z)
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
        Self::from_xy(x, y).with_depth(z)
    }

    #[inline]
    #[must_use]
    pub const fn from_translation_z(translation: Vec2, z: f32) -> Self {
        Self {
            translation,
            depth: z,
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
    pub const fn from_depth(depth: f32) -> Self {
        Self {
            depth,
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
    pub const fn with_depth(mut self, depth: f32) -> Self {
        self.depth = depth;
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
        value = self.rotation_scale_matrix() * value;
        value += self.translation;
        value
    }

    #[inline]
    #[must_use]
    pub fn mul_transform(&self, other: Self) -> Self {
        let translation = self.mul_vec2(other.translation);
        let depth = self.depth + other.depth;
        let rotation = self.rotation + other.rotation;
        let scale = self.scale * other.scale;
        Self {
            translation,
            depth,
            rotation,
            scale,
        }
    }

    #[inline]
    pub fn rotate(&mut self, radians: f32) {
        self.rotation += radians;
    }

    #[inline]
    #[must_use]
    pub fn rotation_matrix(&self) -> Mat2 {
        Mat2::from_angle(self.rotation)
    }

    #[inline]
    pub fn rotation_scale_matrix(&self) -> Mat2 {
        let (sin, cos) = self.rotation.sin_cos();
        let sin = self.scale * sin;
        let cos = self.scale * cos;
        Mat2::from_cols(vec2(cos, sin), vec2(-sin, cos))
    }

    #[inline]
    #[must_use]
    pub fn to_affine(&self) -> Affine3A {
        let (sin, cos) = self.rotation.sin_cos();
        let sin = self.scale * sin;
        let cos = self.scale * cos;
        Affine3A {
            matrix3: Mat3A {
                x_axis: Vec3A::new(cos, sin, 0.),
                y_axis: Vec3A::new(-sin, cos, 0.),
                z_axis: Vec3A::new(0., 0., 1.0),
            },
            translation: Vec3A::new(self.translation.x, self.translation.y, self.depth),
        }
    }

    /// Get the unit vector in the local up direction.
    #[inline]
    #[must_use]
    pub fn up(&self) -> Vec2 {
        self.rotation_matrix() * Vec2::Y
    }

    /// Get the unit vector in the local down direction.
    #[inline]
    #[must_use]
    pub fn down(&self) -> Vec2 {
        -self.up()
    }

    /// Get the unit vector in the local right direction.
    #[inline]
    #[must_use]
    pub fn right(&self) -> Vec2 {
        self.rotation_matrix() * Vec2::X
    }

    /// Get the unit vector in the local left direction.
    #[inline]
    #[must_use]
    pub fn left(&self) -> Vec2 {
        -self.right()
    }

    /// Translates this [`Transform2`] around a `point` in space.
    ///
    /// If this [`Transform2`] has a parent, the `point` is relative to the [`Transform2`] of the parent.
    #[inline]
    pub fn translate_around(&mut self, point: Vec2, angle: f32) {
        self.translation = point + Mat2::from_angle(angle) * (self.translation - point);
    }

    /// Rotates this [`Transform2`] around a `point` in space.
    ///
    /// If this [`Transform2`] has a parent, the `point` is relative to the [`Transform2`] of the parent.
    #[inline]
    pub fn rotate_around(&mut self, point: Vec2, rotation: f32) {
        self.translate_around(point, rotation);
        self.rotate(rotation);
    }
}

impl Default for Transform2 {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl std::fmt::Display for Transform2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ T[{}, {}], Z[{}], R[{}], S[{}] }}",
            self.translation.x, self.translation.y, self.depth, self.rotation, self.scale,
        )
    }
}

impl From<Transform2> for Transform {
    fn from(transform_2: Transform2) -> Self {
        Self {
            translation: transform_2.translation.extend(transform_2.depth),
            rotation: Quat::from_rotation_z(transform_2.rotation),
            scale: Vec2::splat(transform_2.scale).extend(1.0),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Reflect, Component)]
#[reflect(Component, Default, PartialEq)]
pub struct GlobalTransform2(Transform2);

impl GlobalTransform2 {
    pub const IDENTITY: Self = Self(Transform2::IDENTITY);

    #[must_use]
    #[inline]
    pub fn transform(&self) -> &Transform2 {
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

    #[must_use]
    #[inline]
    pub fn depth(&self) -> f32 {
        self.transform().depth
    }

    #[must_use]
    #[inline]
    pub fn mul_transform(&self, transform: Transform2) -> Transform2 {
        self.0.mul_transform(transform)
    }

    #[must_use]
    #[inline]
    pub fn mul_vec2(&self, value: Vec2) -> Vec2 {
        self.0.mul_vec2(value)
    }

    #[inline]
    #[must_use]
    pub(crate) fn propagate_transform(&self, other: Transform2, propagation: Propagate) -> Self {
        if propagation == Propagate::ALL {
            Self(self.mul_transform(other))
        } else {
            Self(Transform2 {
                translation: if propagation.inherits(Propagate::TRANSLATION) {
                    self.mul_vec2(other.translation)
                } else {
                    other.translation
                },
                depth: if propagation.inherits(Propagate::DEPTH) {
                    self.depth() + other.depth
                } else {
                    other.depth
                },
                rotation: if propagation.inherits(Propagate::ROTATION) {
                    self.rotation() + other.rotation
                } else {
                    other.rotation
                },
                scale: if propagation.inherits(Propagate::SCALE) {
                    self.scale() * other.scale
                } else {
                    other.scale
                },
            })
        }
    }
}

impl std::fmt::Display for GlobalTransform2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<Transform2> for GlobalTransform2 {
    #[inline]
    fn from(transform2d: Transform2) -> Self {
        GlobalTransform2(transform2d)
    }
}

impl From<GlobalTransform2> for GlobalTransform {
    #[inline]
    fn from(global_transform_2: GlobalTransform2) -> Self {
        global_transform_2.0.into()
    }
}

impl From<Transform2> for GlobalTransform {
    #[inline]
    fn from(transform2: Transform2) -> Self {
        transform2.to_affine().into()
    }
}

#[cfg(test)]
mod test {
    use bevy::math::vec2;

    use super::*;

    #[test]
    fn transform2_vs_transform() {
        let transform2 = Transform2 {
            translation: (1., 2.).into(),
            depth: 5.,
            rotation: 2.,
            scale: 4.,
        };
        let transform = Transform::from(transform2);

        let vs = [
            vec2(100., -100.),
            vec2(10., -100.),
            vec2(-100., -10.),
            vec2(0., 100.),
            vec2(3., 0.),
            vec2(0.1, 0.5),
        ];

        let e = 0.001;

        for &v in &vs {
            let a = transform2.mul_vec2(v);
            let b = transform.mul_vec3(v.extend(0.0));
            assert!((a.x - b.x).abs() < e);
            assert!((a.y - b.y).abs() < e);
        }

        let f = transform2.mul_transform(transform2);
        let g = transform.mul_transform(transform);

        for &v in &vs {
            let a = f.mul_vec2(v);
            let b = g.mul_vec3(v.extend(0.0));
            assert!((a.x - b.x).abs() < e);
            assert!((a.y - b.y).abs() < e);
        }

        let i = GlobalTransform::from(f);
        let j = GlobalTransform::from(g);

        println!("mat2: {:#?}", f.rotation_scale_matrix());
        println!("aff3: {:#?}", i.affine());
        println!("aff3: {:#?}", j.affine());

        for &v in &vs {
            let a = i.mul_vec3(v.extend(0.0));
            let b = j.mul_vec3(v.extend(0.0));
            assert!((a.x - b.x).abs() < e);
            assert!((a.y - b.y).abs() < e);
        }
    }

    #[test]
    fn propagate() {
        for i in 1..=15 {
            assert!(!Propagate::NOTHING.inherits(Propagate(i)));
        }

        for i in 1..=15 {
            assert!(Propagate::ALL.inherits(Propagate(i)));
        }

        for i in 1..=15 {
            let p = Propagate(i);
            assert!(p.inherits(p));
            for j in 1..=15 {
                let q = Propagate(j & (!i));
                assert!(!p.inherits(q));
            }
        }
    }
}
