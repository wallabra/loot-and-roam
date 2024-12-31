//! Basic terrain definitions.

/// A terrain definition.
pub struct Terrain {
    /// The definition from which the Terrain gets its heightmap values.
    def: Box<dyn TerrainNode>,
}

impl Terrain {
    pub fn compute_height_at_raw(&self, x: i64, y: i64) -> u16 {
        self.def.get_height(x, y)
    }

    pub fn compute_height_at_f64(&self, x: i64, y: i64) -> f32 {
        self.def.get_height(x, y) as f32 / (1 << 4) as f32
    }
}

impl Default for Terrain {
    fn default() -> Self {
        Terrain {
            def: Box::from(ConstantNode { value: 64 }),
        }
    }
}

/// A terrain node.
pub trait TerrainNode {
    fn get_height(&self, x: i64, y: i64) -> u16;
}

pub struct ConstantNode {
    value: u16,
}

impl TerrainNode for ConstantNode {
    fn get_height(&self, _x: i64, _y: i64) -> u16 {
        self.value
    }
}
