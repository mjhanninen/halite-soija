use hlt::types;
use ua::space::{Space, Pos};
use ua::map::{Map, Site};

impl Pos {
    #[inline]
    pub fn to_hlt_location(&self) -> types::Location {
        types::Location {
            x: self.x as u16,
            y: self.y as u16,
        }
    }
}

impl Site {
    #[inline]
    pub fn from_hlt_site(&mut self, hlt_site: &types::Site) {
        self.owner = hlt_site.owner;
        self.strength = hlt_site.strength;
        self.production = hlt_site.production;
    }
}

impl Map {
    pub fn from_hlt_game_map(hlt_game_map: &types::GameMap) -> Map {
        let w = hlt_game_map.width;
        let h = hlt_game_map.height;
        let space = Space::with_dims(w as i16, h as i16);
        let mut map = Map::from_space(space);
        for p in map.space.sweep() {
            map.sites[map.space.ix(&p)]
                .from_hlt_site(hlt_game_map.get_site(p.to_hlt_location(),
                                                     types::STILL));
        }
        map
    }
}
