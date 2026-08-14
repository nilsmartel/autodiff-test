use ultraviolet::Vec2;
fn main() {
    println!("Hello, world!");
}
type V = Vec2;

struct Model {
    verts: Vec<V>,
    edges: Vec<(usize, usize)>,
}

