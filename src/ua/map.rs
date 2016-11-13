use ua::space::{Pos, Space};

#[derive(Clone, Debug)]
pub struct Site {
    pub owner: u8,
    pub strength: u8,
    pub production: u8,
}

impl Site {
    pub fn blank() -> Self {
        Site {
            owner: 0,
            strength: 0,
            production: 0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Map {
    pub space: Space,
    pub sites: Vec<Site>,
}

impl Map {
    pub fn from_space(space: Space) -> Self {
        let sites = vec![Site::blank(); space.len()];
        Map {
            space: space,
            sites: sites,
        }
    }

    pub fn site(&self, pos: &Pos) -> &Site {
        &self.sites[self.space.ix(pos)]
    }
}
