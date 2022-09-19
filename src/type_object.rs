#![allow(dead_code)]

struct Breed {
    health: i32,
    attack: String,
}

impl Breed {
    fn new(parent: Option<&Breed>, health: i32, attack: String) -> Self {
        // プロパティを継承するロジック
        if let Some(parent_breed) = parent {
            Self {
                health: if health == 0 {
                    parent_breed.health
                } else {
                    health
                },
                attack: if attack == "" {
                    parent_breed.attack.clone()
                } else {
                    attack
                },
            }
        } else {
            Self { health, attack }
        }
    }

    fn new_monster(&self) -> Monster {
        Monster::new(self)
    }
}

struct Monster<'a> {
    health: i32,
    breed: &'a Breed,
}

impl<'a> Monster<'a> {
    fn new(breed: &'a Breed) -> Self {
        Self {
            health: breed.health,
            breed: breed,
        }
    }

    fn attack(&self) -> &str {
        &self.breed.attack
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn type_object() {
        let simple_breed = Breed::new(None, 100, "bite".to_string());
        let simple_monster = simple_breed.new_monster();
        assert_eq!(simple_monster.attack(), "bite");

        let inherit_breed_1 = Breed::new(Some(&simple_breed), 10, "".to_string());
        let inherit_monster_1 = inherit_breed_1.new_monster();
        assert_eq!(inherit_monster_1.attack(), "bite");

        let inherit_breed_2 = Breed::new(Some(&simple_breed), 0, "bite".to_string());
        let inherit_monster_2 = inherit_breed_2.new_monster();
        assert_eq!(inherit_monster_2.health, 100);
    }
}
