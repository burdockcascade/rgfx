use crate::renderer::vertex::Vertex2D;

#[derive(Clone, Debug)]
pub struct Mesh<V> {
    pub vertices: Vec<V>,
    pub indices: Vec<u16>,
}

impl<V> Mesh<V> {
    pub fn new(vertices: Vec<V>, indices: Vec<u16>) -> Self {
        Self { vertices, indices }
    }

}

pub struct MeshBuilder2D;

impl MeshBuilder2D {

    // Creates a mesh from a triangle with the given position, size, and color.
    pub fn from_triangle(color: [f32; 4]) -> Mesh<Vertex2D> {

        let x = 0.0;
        let y = 0.0;
        let size = 1.0; // Default size for the triangle

        let vertices = vec![
            Vertex2D {
                position: [x, y + size],
                uv: [0.5, 1.0],
                color,
            },
            Vertex2D {
                position: [x + size, y - size],
                uv: [1.0, 0.0],
                color,
            },
            Vertex2D {
                position: [x - size, y - size],
                uv: [0.0, 0.0],
                color,
            },
        ];
        let indices = vec![0, 1, 2]; // Single triangle

        Mesh::new(vertices, indices)
    }

    pub fn from_rectangle(width: f32, height: f32, color: [f32; 4]) -> Mesh<Vertex2D> {
        let x = 0.0;
        let y = 0.0;
        let half_width = width / 2.0;
        let half_height = height / 2.0;
        let vertices = vec![
            // Top-left
            Vertex2D {
                position: [x - half_width, y + half_height],
                uv: [0.0, 0.0],
                color,
            },
            // Top-right
            Vertex2D {
                position: [x + half_width, y + half_height],
                uv: [1.0, 0.0],
                color,
            },
            // Bottom-left
            Vertex2D {
                position: [x - half_width, y - half_height],
                uv: [0.0, 1.0],
                color,
            },
            // Bottom-right
            Vertex2D {
                position: [x + half_width, y - half_height],
                uv: [1.0, 1.0],
                color,
            },
        ];
        let indices = vec![0, 1, 2, 1, 3, 2]; // Two triangles forming a rectangle
        Mesh::new(vertices, indices)
    }

    pub fn from_circle(radius: f32, segments: usize, color: [f32; 4]) -> Mesh<Vertex2D> {
        let mut vertices = Vec::with_capacity(segments + 1);
        let mut indices = Vec::with_capacity(segments * 3);

        // Center vertex
        vertices.push(Vertex2D {
            position: [0.0, 0.0],
            uv: [0.5, 0.5],
            color,
        });

        // Circle vertices
        for i in 0..=segments {
            let angle = (i as f32 / segments as f32) * std::f32::consts::PI * 2.0;
            let x = radius * angle.cos();
            let y = radius * angle.sin();
            vertices.push(Vertex2D {
                position: [x, y],
                uv: [(x / radius + 1.0) / 2.0, (y / radius + 1.0) / 2.0],
                color,
            });
        }

        // Indices for the circle
        for i in 1..=segments {
            indices.push(0);
            indices.push(if i == segments { 1 } else { i as u16 + 1 });
            indices.push(i as u16);
        }

        Mesh::new(vertices, indices)
    }

}