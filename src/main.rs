use rand::random;
use ultraviolet::Vec2;

fn main() {
    println!("Hello, world!");
}

type V = Vec2;

struct Graph {
    verts: Vec<V>,
    edges: Vec<(usize, usize)>,
}

impl Graph {
    /// Generate a new random Model with n vertecice
    fn new_random(n: usize) -> Self {
        const SPREAD: f32 = 10.0;
        // generate n verts
        let verts = (0..n)
            .map(|_| V::new(random(), random()) * SPREAD)
            .collect();

        // every vert should have at least 1 neighboor

        let edges = (0..(n - 1))
            .map(|i| {
                // remaining vert ids
                let remaining = n - (i + 1);

                let j = ((remaining as f32) * random::<f32>()) as usize + i;

                assert!(j < n, "expect j < n");

                (i, j)
            })
            .collect();

        Graph { verts, edges }
    }
}

fn error(model: &Graph) -> f32 {
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
