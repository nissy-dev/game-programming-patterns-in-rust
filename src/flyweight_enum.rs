#![allow(dead_code)]

enum Terrain {
    GRASS,
    HILL,
    RIVER,
}

impl Terrain {
    fn cost(&self) -> i32 {
        match self {
            Terrain::GRASS => 1,
            Terrain::HILL => 2,
            Terrain::RIVER => 3,
        }
    }

    fn is_water(&self) -> bool {
        match self {
            Terrain::GRASS => false,
            Terrain::HILL => false,
            Terrain::RIVER => true,
        }
    }
}

struct World<'a> {
    _terrains: Vec<Vec<Option<&'a Terrain>>>,
}

impl<'a> World<'a> {
    fn new() -> World<'a> {
        World {
            _terrains: vec![vec![None; 3]; 3],
        }
    }

    fn generate_terrains(&mut self) {
        for i in 0..3 {
            for j in 0..3 {
                if (i + j) % 2 == 0 {
                    self._terrains[i][j] = Some(&Terrain::HILL);
                } else {
                    self._terrains[i][j] = Some(&Terrain::GRASS);
                }
            }
        }

        // Lay a river
        for i in 0..3 {
            self._terrains[1][i] = Some(&Terrain::RIVER);
        }
    }

    fn get_tile(&self, x: usize, y: usize) -> &'a Terrain {
        if let Some(terrain) = self._terrains[x][y] {
            terrain
        } else {
            panic!("World's terrain has not been generated yet");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn world_test() {
        let mut world = World::new();
        world.generate_terrains();
        let tile = world.get_tile(1, 1);
        assert_eq!(tile.is_water(), true);
        assert_eq!(tile.cost(), 3);
    }
}
