use crate::common::terrain::base::TerrainNode;

pub struct TerrainAdder {
    nodes: Vec<Box<dyn TerrainNode>>,
}

impl TerrainNode for TerrainAdder {
    fn get_height(&self, x: i64, y: i64) -> u16 {
        self.nodes
            .iter()
            .map(|node| node.get_height(x, y))
            .fold(0, |acc, next| acc + next)
    }
}

pub struct TerrainMultiplier {
    nodes: Vec<Box<dyn TerrainNode>>,
}

impl TerrainNode for TerrainMultiplier {
    fn get_height(&self, x: i64, y: i64) -> u16 {
        self.nodes
            .iter()
            .map(|node| node.get_height(x, y))
            .fold(1u16, |acc, next| ((acc as i32 * next as i32) >> 4) as u16)
    }
}
