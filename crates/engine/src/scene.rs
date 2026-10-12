/// A box in world space. Scale gives its full width, height, and depth.
pub struct Object {
    pub position: [f32; 3],
    pub scale: [f32; 3],
    pub color: [f32; 3],
}

pub const PROTOTYPE_SCENE: &[Object] = &[
    // Marker first so depth testing, rather than draw order, keeps it visible.
    Object {
        position: [0.0, 1.0, 0.0],
        scale: [1.0, 2.0, 1.0],
        color: [0.9, 0.3, 0.15],
    },
    Object {
        position: [0.0, -0.5, 0.0],
        scale: [12.0, 1.0, 12.0],
        color: [0.25, 0.45, 0.3],
    },
    Object {
        position: [0.0, 1.5, -6.5],
        scale: [14.0, 3.0, 1.0],
        color: [0.55, 0.65, 0.75],
    },
    Object {
        position: [0.0, 1.5, 6.5],
        scale: [14.0, 3.0, 1.0],
        color: [0.55, 0.65, 0.75],
    },
    Object {
        position: [-6.5, 1.5, 0.0],
        scale: [1.0, 3.0, 12.0],
        color: [0.45, 0.55, 0.65],
    },
    Object {
        position: [6.5, 1.5, 0.0],
        scale: [1.0, 3.0, 12.0],
        color: [0.45, 0.55, 0.65],
    },
];
