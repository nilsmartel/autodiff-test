use ultraviolet::Vec2;
fn main() {
    println!("Hello, world!");
}
type V = Vec2;

struct Model {
    verts: Vec<V>,
    edges: Vec<(usize, usize)>,
}

fn error(model: &Model) -> f32 {
    const DESIRED_DISTANCE: f32 = 1.0;

    let mut sum = 0.0;
    for (i, j) in &model.edges {
        let v1: Vec2 = model.verts[*i];
        let v2: Vec2 = model.verts[*j];

        // compute eucl. distance
        let v = v1 - v2;
        let distance = v.dot(v).sqrt();

        // we add the power, so that no harsh outliers are tolerated by the computation algorithm
        // and the error is always >= 0
        let error = (distance - DESIRED_DISTANCE).powi(2);

        sum += error;
    }

    sum
}
