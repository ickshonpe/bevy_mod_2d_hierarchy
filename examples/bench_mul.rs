use bevy::math::vec2;
use bevy_2d_transform_hierarchy::alt_systems::Transform2dPropagationDescriptor;
use bevy_2d_transform_hierarchy::transform_2d::GlobalTransform2d;
use bevy_2d_transform_hierarchy::transform_2d::Transform2d;
use stopwatch::*;

const N: u64 = 1_000;
const R: u64 = 1_000_000;
const OTHER: Transform2d  = Transform2d {
    translation: vec2(0.01, 0.0),
    z: 0.0,
    scale: 1.0,
    rotation: 0.0,
};

pub fn mul(reps: u64) {
    let mut transform = GlobalTransform2d::default();
   
    for _ in 0..reps {
        transform = transform.mul_transform(OTHER).into();
        transform.translation *= -1.0;
    }
}

pub fn prop_some(reps: u64) {
    let mut transform = GlobalTransform2d::default();
   
    let descriptor =
        Transform2dPropagationDescriptor {
            inherit_translation: true,
            inherit_z: true,
            inherit_rotation: true,
            inherit_scale: true,
        }

    ;
    for _ in 0..reps {
        transform = transform.propagate_transform(OTHER, Some(&descriptor)).into();
        transform.translation *= -1.0;
    }
}

pub fn prop_none(reps: u64) {
    let mut transform = GlobalTransform2d::default();
    for _ in 0..reps {
        transform = transform.propagate_transform(OTHER, None).into();
        transform.translation *= -1.0;
    }
}

pub fn main() {
    let mut sw = Stopwatch::new();
    let mut m_t = 0;
    let mut pd_t = 0;
    let mut pn_t = 0;
    for _ in 0..N {
       
        sw.start();
        prop_some(R);
        sw.stop();
        pd_t += sw.elapsed().as_millis();

        sw.start();
        mul(R);
        sw.stop();
        m_t += sw.elapsed().as_millis();


        sw.start();
        prop_none(R);
        sw.stop();
        pn_t += sw.elapsed().as_millis();
    }
    println!("mul nanos: {m_t}");
    println!("prop_none nanos: {pn_t}");
    println!("prop_some nanos: {pd_t}");
    let p_mn = m_t as f64 / pn_t as f64;
    let p_md = m_t as f64 / pd_t as f64;
    let p_nd = m_t as f64 / pn_t as f64;
    println!("proportion m / pn: {p_mn}");
    println!("proportion m / pd: {p_md}");
    println!("proportion pn / pd: {p_nd}");
}

