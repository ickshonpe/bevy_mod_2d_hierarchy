use crate::prelude::*;
use crate::transform2::Propagate;
use bevy::prelude::*;
use bevy::render::camera::CameraRenderGraph;
use bevy::render::primitives::Frustum;
use bevy::render::texture::DEFAULT_IMAGE_HANDLE;
use bevy::render::view::VisibleEntities;
use bevy::text::Text2dBounds;
use bevy::text::Text2dSize;

/// 2d transform components
#[derive(Bundle, Clone, Debug, Default)]
pub struct TransformBundle2 {
    pub transform_2d: Transform2,
    pub global_transform_2d: GlobalTransform2,
    /// not required but probably going to want to extract for rendering
    /// the derive_global_transform system is inexpensive
    pub global_transform: GlobalTransform,
    pub propagate: Propagate,
}

impl TransformBundle2 {
    pub const IDENTITY: Self = TransformBundle2 {
        transform_2d: Transform2::IDENTITY,
        global_transform_2d: GlobalTransform2::IDENTITY,
        global_transform: GlobalTransform::identity(),
        propagate: Propagate::ALL,
    };

    #[inline]
    pub const fn from_transform(transform: Transform2) -> Self {
        TransformBundle2 {
            transform_2d: transform,
            ..Self::IDENTITY
        }
    }
}

impl From<Transform2> for TransformBundle2 {
    #[inline]
    fn from(transform: Transform2) -> Self {
        Self::from_transform(transform)
    }
}

/// SpatialBundle with a 2d transform
#[derive(Bundle, Clone, Debug, Default)]
pub struct SpatialBundle2 {
    pub transform2: Transform2,
    pub global_transform2: GlobalTransform2,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
    pub propagate: Propagate,
}

impl SpatialBundle2 {
    #[inline]
    pub const fn from_transform(transform2: Transform2) -> Self {
        Self {
            transform2,
            global_transform2: GlobalTransform2::IDENTITY,
            global_transform: GlobalTransform::identity(),
            visibility: Visibility { is_visible: true },
            computed_visibility: ComputedVisibility::not_visible(),
            propagate: Propagate::ALL,
        }
    }
}

impl From<Transform2> for SpatialBundle2 {
    #[inline]
    fn from(transform2: Transform2) -> Self {
        Self::from_transform(transform2)
    }
}

/// SpriteBundle with a 2d transform
#[derive(Bundle, Clone)]
pub struct SpriteBundle2 {
    pub sprite: Sprite,
    pub transform2: Transform2,
    pub global_transform2: GlobalTransform2,
    pub global_transform: GlobalTransform,
    pub texture: Handle<Image>,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
    pub propagate: Propagate,
}

impl Default for SpriteBundle2 {
    fn default() -> Self {
        Self {
            sprite: Default::default(),
            transform2: Default::default(),
            global_transform2: Default::default(),
            global_transform: Default::default(),
            texture: DEFAULT_IMAGE_HANDLE.typed(),
            visibility: Default::default(),
            computed_visibility: Default::default(),
            propagate: Propagate::ALL,
        }
    }
}

/// SpriteSheetBundle with a 2d transform
#[derive(Bundle, Clone, Default)]
pub struct SpriteSheetBundle2 {
    pub sprite: TextureAtlasSprite,
    pub texture_atlas: Handle<TextureAtlas>,
    pub transform2: Transform2,
    pub global_transform2: GlobalTransform2,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
    pub propagate: Propagate,
}

/// Text2dBundle with 2d transform
#[derive(Bundle, Clone, Debug, Default)]
pub struct Text2dBundle2 {
    pub text: Text,
    pub trasform2: Transform2,
    pub global_transform2: GlobalTransform2,
    pub global_transform: GlobalTransform,
    pub text_2d_size: Text2dSize,
    pub text_2d_bounds: Text2dBounds,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
    pub propagate: Propagate,
}

/// Camera2dBundle with 2d transform
#[derive(Bundle)]
pub struct Camera2dBundle2 {
    pub camera: Camera,
    pub camera_render_graph: CameraRenderGraph,
    pub projection: OrthographicProjection,
    pub visible_entities: VisibleEntities,
    pub frustum: Frustum,
    pub transform2: Transform2,
    pub global_transform2: GlobalTransform2,
    pub propagate: Propagate,
    pub global_transform: GlobalTransform,
    pub camera_2d: Camera2d,
}

impl Default for Camera2dBundle2 {
    fn default() -> Self {
        Self::new_with_far(1000.0)
    }
}

impl Camera2dBundle2 {
    /// Create an orthographic projection camera with a custom `Z` position.
    ///
    /// The camera is placed at `Z=far-0.1`, looking toward the world origin `(0,0,0)`.
    /// Its orthographic projection extends from `0.0` to `-far` in camera view space,
    /// corresponding to `Z=far-0.1` (closest to camera) to `Z=-0.1` (furthest away from
    /// camera) in world space.
    pub fn new_with_far(far: f32) -> Self {
        // create a regular bevy Camera2dBundle and copy the components except for `Transform`
        let cb1 = Camera2dBundle::new_with_far(far);
        Self {
            camera_render_graph: cb1.camera_render_graph,
            projection: cb1.projection,
            visible_entities: cb1.visible_entities,
            frustum: cb1.frustum,
            camera: cb1.camera,
            camera_2d: cb1.camera_2d,
            transform2: Default::default(),
            global_transform: Default::default(),
            global_transform2: Default::default(),
            propagate: Default::default(),
        }
    }
}
