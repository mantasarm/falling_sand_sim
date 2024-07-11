use notan::math::Vec2;

#[derive(Clone, Copy, Debug)]
pub struct Cell {
    pub element: Element,
    pub action: Option<Action>,
    pub density: f32,
    pub state: State,
    pub color: [u8; 4],
    pub velocity: Vec2,
    pub drag: f32,
    pub lifetime: i32,
}

pub fn air_element() -> Cell {
    Cell {
        element: Element::Air,
        action: None,
        density: 0.,
        state: State::Gas,
        color: [0, 0, 0, 0],
        velocity: Vec2::ZERO,
        drag: 0.95,
        lifetime: -1,
    }
}

pub fn solid_element() -> Cell {
    Cell {
        element: Element::Solid,
        action: None,
        density: 100.,
        state: State::Solid,
        color: [69, 62, 66, 255],
        velocity: Vec2::ZERO,
        drag: 0.,
        lifetime: -1,
    }
}

pub fn wood_element() -> Cell {
    Cell {
        element: Element::Wood,
        action: None,
        density: 100.,
        state: State::Solid,
        color: [111, 83, 57, 255],
        velocity: Vec2::ZERO,
        drag: 0.,
        lifetime: -1,
    }
}

pub fn coal_element() -> Cell {
    Cell {
        element: Element::Coal,
        action: None,
        density: 100.,
        state: State::Solid,
        color: [42, 42, 42, 255],
        velocity: Vec2::ZERO,
        drag: 0.,
        lifetime: -1,
    }
}

pub fn sand_element() -> Cell {
    Cell {
        element: Element::Sand,
        action: None,
        density: 60.,
        state: State::Powder,
        color: [243, 239, 118, 255],
        velocity: Vec2::ZERO,
        drag: 1.0,
        lifetime: -1,
    }
}

pub fn dirt_element() -> Cell {
    Cell {
        element: Element::Dirt,
        action: None,
        density: 60.,
        state: State::Powder,
        color: [76, 57, 32, 255],
        velocity: Vec2::ZERO,
        drag: 0.9,
        lifetime: -1,
    }
}

pub fn sawdust_element() -> Cell {
    Cell {
        element: Element::SawDust,
        action: None,
        density: 40.,
        state: State::Powder,
        color: [181, 137, 100, 255],
        velocity: Vec2::ZERO,
        drag: 0.9,
        lifetime: -1,
    }
}

pub fn water_element() -> Cell {
    Cell {
        element: Element::Water,
        action: None,
        density: 50.,
        state: State::Liquid,
        color: [55, 46, 229, 175],
        velocity: Vec2::ZERO,
        drag: 0.4,
        lifetime: -1,
    }
}

pub fn petrol_element() -> Cell {
    Cell {
        element: Element::Petrol,
        action: None,
        density: 45.,
        state: State::Liquid,
        color: [0, 95, 106, 175],
        velocity: Vec2::ZERO,
        drag: 0.4,
        lifetime: -1,
    }
}

pub fn smoke_element() -> Cell {
    Cell {
        element: Element::Smoke,
        action: None,
        density: 4.,
        state: State::Gas,
        color: [42, 42, 42, 220],
        velocity: Vec2::ZERO,
        drag: 0.95,
        lifetime: -1,
    }
}

pub fn steam_element() -> Cell {
    Cell {
        element: Element::Steam,
        action: None,
        density: 2.,
        state: State::Gas,
        color: [143, 159, 234, 140],
        velocity: Vec2::ZERO,
        drag: 0.95,
        lifetime: -1,
    }
}

pub fn fire_element() -> Cell {
    Cell {
        element: Element::Fire,
        action: None,
        density: 4.,
        state: State::Plasma,
        color: [255, 170, 0, 220],
        velocity: Vec2::ZERO,
        drag: 1.,
        lifetime: 50,
    }
}

pub fn methane_element() -> Cell {
    Cell {
        element: Element::Methane,
        action: None,
        density: 3.,
        state: State::Gas,
        color: [130, 171, 41, 140],
        velocity: Vec2::ZERO,
        drag: 0.95,
        lifetime: -1,
    }
}

pub fn lava_element() -> Cell {
    Cell {
        element: Element::Lava,
        action: Some(Action::Burn),
        density: 120.,
        state: State::Liquid,
        color: [234, 46, 56, 255],
        velocity: Vec2::ZERO,
        drag: 0.1,
        lifetime: -1,
    }
}

pub fn source_element() -> Cell {
    Cell {
        element: Element::Source,
        action: Some(Action::EmitSource(Element::Air)),
        density: 100.,
        state: State::Solid,
        color: [252, 186, 3, 255],
        velocity: Vec2::ZERO,
        drag: 0.,
        lifetime: -1,
    }
}

pub fn gravel_element() -> Cell {
    Cell {
        element: Element::Gravel,
        action: None,
        density: 130.,
        state: State::Solid,
        color: [83, 84, 78, 255],
        velocity: Vec2::ZERO,
        drag: 0.9,
        lifetime: -1,
    }
}

pub fn soliddirt_element() -> Cell {
    Cell {
        element: Element::SolidDirt,
        action: None,
        density: 100.,
        state: State::Solid,
        color: [76, 57, 32, 255],
        velocity: Vec2::ZERO,
        drag: 0.,
        lifetime: -1,
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Element {
    Air,
    Solid,
    Sand,
    SawDust,
    Water,
    Steam,
    Smoke,
    Dirt,
    Fire,
    Wood,
    Coal,
    Methane,
    Petrol,
    Lava,
    Source,
    Gravel,
    SolidDirt
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum State {
    Solid,
    Powder,
    Liquid,
    Gas,
    Plasma,
}

#[derive(Clone, Copy, Debug)]
pub enum Action {
    Burn, EmitSource(Element)
}

pub fn el_from_enum(element: Element) -> Cell {
    match element {
        Element::Air => air_element(),
        Element::Solid => solid_element(),
        Element::Sand => sand_element(),
        Element::SawDust => sawdust_element(),
        Element::Water => water_element(),
        Element::Steam => steam_element(),
        Element::Smoke => smoke_element(),
        Element::Dirt => dirt_element(),
        Element::Fire => fire_element(),
        Element::Wood => wood_element(),
        Element::Coal => coal_element(),
        Element::Methane => methane_element(),
        Element::Petrol => petrol_element(),
        Element::Lava => lava_element(),
        Element::Source => source_element(),
        Element::Gravel => gravel_element(),
        Element::SolidDirt => soliddirt_element()
    }
}

