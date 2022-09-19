#![allow(dead_code)]

struct Terrain {
    cost: i32,
    is_water: bool,
}

struct World<'a> {
    _terrains: Vec<Vec<Option<&'a Terrain>>>,
    _grass: &'a Terrain,
    _hill: &'a Terrain,
    _river: &'a Terrain,
}

impl<'a> World<'a> {
    fn new() -> World<'a> {
        World {
            _terrains: vec![vec![None; 3]; 3],
            _grass: &Terrain {
                cost: 1,
                is_water: false,
            },
            _hill: &Terrain {
                cost: 2,
                is_water: false,
            },
            _river: &Terrain {
                cost: 3,
                is_water: true,
            },
        }
    }

    fn generate_terrains(&mut self) {
        for i in 0..3 {
            for j in 0..3 {
                if (i + j) % 2 == 0 {
                    self._terrains[i][j] = Some(&self._hill);
                } else {
                    self._terrains[i][j] = Some(&self._grass);
                }
            }
        }

        // Lay a river
        for i in 0..3 {
            self._terrains[1][i] = Some(&self._river);
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
    fn flyweight() {
        let mut world = World::new();
        world.generate_terrains();
        let tile = world.get_tile(1, 1);
        assert_eq!(tile.is_water, true);
        assert_eq!(tile.cost, 3);
    }
}
