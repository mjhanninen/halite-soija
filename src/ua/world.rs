pub type Map<T> = Vec<T>;
pub type Production = i16;

#[derive(Clone, Debug)]
pub struct Space {
    pub width: i16,
    pub height: i16,
}

impl Space {
    pub fn with_dims(width: i16, height: i16) -> Self {
        Space {
            width: width,
            height: height,
        }
    }
    pub fn size(&self) -> usize {
        (self.width as usize) * (self.height as usize)
    }
}

#[derive(Debug)]
pub struct Environment {
    pub my_tag: Tag,
    pub space: Space,
    pub production_map: Map<Production>,
}

impl Environment {
    pub fn create(my_tag: Tag, width: i16, height: i16) -> Self {
        let space = Space::with_dims(width, height);
        let production_map = Vec::with_capacity(space.size());
        Environment {
            my_tag: my_tag,
            space: space,
            production_map: production_map,
        }
    }
}

pub type Tag = u8;
pub type Strength = i16;

#[derive(Clone, Debug)]
pub struct Occupation {
    pub tag: Tag,
    pub strength: Strength,
}

#[derive(Debug)]
pub struct State {
    pub occupation_map: Map<Occupation>,
}

impl State {
    pub fn for_environment(environment: &Environment) -> Self {
        State {
            occupation_map: vec![Occupation {tag: 0, strength: 0};
                 environment.space.size()],
        }
    }
}

pub type Pos = (i16, i16);

#[derive(Debug)]
pub enum Dir {
    Still,
    North,
    East,
    South,
    West,
}

pub type Move = (Pos, Dir);
