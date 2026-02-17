mod areas_volumes;
pub use crate::areas_volumes::*;

pub fn area_fit(
    (x, y): (usize, usize),
    kind: areas_volumes::GeometricalShapes,
    times: usize,
    (a, b): (usize, usize)
) -> bool {
    let s2 = (x * y) as f64;

    let s1 = match kind {
        areas_volumes::GeometricalShapes::Square => { areas_volumes::square_area(a) as f64 }
        areas_volumes::GeometricalShapes::Rectangle => {areas_volumes::rectangle_area(a, b) as f64 }
        areas_volumes::GeometricalShapes::Triangle => { areas_volumes::triangle_area(a, b) }
        areas_volumes::GeometricalShapes::Circle => { areas_volumes::circle_area(a) }
    };

    s1 * (times as f64) <= s2
}

pub fn volume_fit(
    (x, y, z): (usize, usize, usize),
    kind: areas_volumes::GeometricalVolumes,
    times: usize,
    (a, b, c): (usize, usize, usize)
) -> bool {
    let s2 = (x * y * z) as f64;

    let s1 = match kind {
        areas_volumes::GeometricalVolumes::Cube => { areas_volumes::cube_volume(a) as f64 }
        areas_volumes::GeometricalVolumes::Sphere => { areas_volumes::sphere_volume(a) }
        areas_volumes::GeometricalVolumes::Cone => { areas_volumes::cone_volume(a, b) }
        areas_volumes::GeometricalVolumes::TriangularPyramid => {
            areas_volumes::triangular_pyramid_volume(a as f64, b)
        }
        areas_volumes::GeometricalVolumes::Parallelepiped => {
            areas_volumes::parallelepiped_volume(a, b, c) as f64
        }
    };

    s1 * (times as f64) <= s2
}
