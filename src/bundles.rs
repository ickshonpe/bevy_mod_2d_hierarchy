use bevy::prelude::*;
use bevy::render::texture::DEFAULT_IMAGE_HANDLE;
use bevy::text::Text2dBounds;
use bevy::text::Text2dSize;
use crate::prelude::*;


#[derive(Bundle, Clone, Debug, Default)]
pub struct Transform2dBundle {
    pub transform_2d: Transform2d,
    pub global_transform_2d: GlobalTransform2d,
    pub global_transform: GlobalTransform
}

impl Transform2dBundle {
    pub const IDENTITY: Self = Transform2dBundle {
        transform_2d: Transform2d::IDENTITY,
        global_transform_2d: GlobalTransform2d::IDENTITY,
        global_transform: GlobalTransform::identity()
    };

    #[inline]
    pub const fn from_transform(transform: Transform2d) -> Self {
        Transform2dBundle {
            transform_2d: transform,
            ..Self::IDENTITY
        }
    }
}

impl From<Transform2d> for Transform2dBundle {
    #[inline]
    fn from(transform: Transform2d) -> Self {
        Self::from_transform(transform)
    }
}

#[derive(Bundle, Clone, Debug, Default)]
pub struct Spatial2dBundle {
    pub transform_2d: Transform2d,
    pub global_transform_2d: GlobalTransform2d,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
}

impl Spatial2dBundle {
    #[inline]
    pub const fn from_transform(transform_2d: Transform2d) -> Self {
        Self {
            transform_2d,
            global_transform_2d: GlobalTransform2d::IDENTITY,
            global_transform: GlobalTransform::identity(),
            visibility: Visibility { is_visible: true },
            computed_visibility: ComputedVisibility::not_visible(),
        }
    }
}

impl From<Transform2d> for Spatial2dBundle {
    #[inline]
    fn from(transform_2d: Transform2d) -> Self {
        Self::from_transform(transform_2d)
    }
}


#[derive(Bundle, Clone,)]
pub struct SpriteBundle2 {
    pub sprite: Sprite,
    pub transform_2d: Transform2d,
    pub global_transform_2d: GlobalTransform2d,
    pub global_transform: GlobalTransform,
    pub texture: Handle<Image>,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
}

impl Default for SpriteBundle2 {
    fn default() -> Self {
        Self { 
            sprite: Default::default(), 
            transform_2d: Default::default(), 
            global_transform_2d: Default::default(), 
            global_transform: Default::default(), 
            texture: DEFAULT_IMAGE_HANDLE.typed(),
            visibility: 
            Default::default(), 
            computed_visibility: Default::default() 
        }
    }
}

#[derive(Bundle, Clone, Default)]
pub struct SpriteSheetBundle2 {
    pub sprite: TextureAtlasSprite,
    pub texture_atlas: Handle<TextureAtlas>,
    pub transform_2d: Transform2d,
    pub global_transform_2d: GlobalTransform2d,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
}

#[derive(Bundle, Clone, Debug, Default)]
pub struct Text2dBundle2 {
    pub text: Text,
    pub transform_2d: Transform2d,
    pub global_transform_2d: GlobalTransform2d,
    pub global_transform: GlobalTransform,
    pub text_2d_size: Text2dSize,
    pub text_2d_bounds: Text2dBounds,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
}