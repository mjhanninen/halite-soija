use ua::space::{Space, Pos, Dir};
use ua::map::{Map};
use ua::world;

impl Dir {
    #[inline]
    pub fn to_world(&self) -> world::Dir {
        match *self {
            Dir::North => world::Dir::North,
            Dir::East => world::Dir::East,
            Dir::South => world::Dir::South,
            Dir::West => world::Dir::West,
        }
    }
}

impl Pos {
    #[inline]
    pub fn to_world(&self) -> world::Pos {
        (self.x, self.y)
    }
}

impl Map {
    pub fn from_world(environment: &world::Environment,
                      state: &world::State) -> Map {
        let w = environment.space.width;
        let h = environment.space.height;
        let space = Space::with_dims(w as i16, h as i16);
        let mut map = Map::from_space(space);
        for i in 0..map.sites.len() {
            let site = &mut map.sites[i];
            site.owner = state.occupation_map[i].tag;
            site.strength = state.occupation_map[i].strength as u8;
            site.production = environment.production_map[i] as u8;
        }
        map
    }
}
