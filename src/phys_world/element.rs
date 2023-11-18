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
	pub lifetime: i32
}

pub fn air_element() -> Cell {
	Cell {
		element: Element::Air,
		action: None,
		density: 0.,
		state: State::Gas,
		color: [0, 0, 0, 0],
		velocity: Vec2::ZERO,
		drag: 1.,
		lifetime: -1
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
		lifetime: -1
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
		lifetime: -1
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
		lifetime: -1
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
		drag: 0.9,
		lifetime: -1
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
		lifetime: -1
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
		lifetime: -1
	}
}

pub fn water_element() -> Cell {
	Cell {
		element: Element::Water,
		action: None,
		density: 50.,
		state:State::Liquid,
		color: [55, 46, 229, 175],
		velocity: Vec2::ZERO,
		drag: 0.8,
		lifetime: -1
	}
}

pub fn petrol_element() -> Cell {
	Cell {
		element: Element::Petrol,
		action: None,
		density: 40.,
		state:State::Liquid,
		color: [0, 95, 106, 175],
		velocity: Vec2::ZERO,
		drag: 0.8,
		lifetime: -1
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
		drag: 1.,
		lifetime: -1
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
		drag: 1.,
		lifetime: -1
	}
}


pub fn fire_element() -> Cell {
	Cell {
		element: Element::Fire,
		action: None,
		density: 1.,
		state: State::Plasma,
		color: [255, 170, 0, 220],
		velocity: Vec2::ZERO,
		drag: 1.,
		lifetime: 150
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
		drag: 1.,
		lifetime: -1
	}
}


#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Element {
	Air, Solid, Sand, SawDust, Water, Steam, Smoke, Dirt, Fire, Wood, Coal, Methane, Petrol
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum State {
	Solid, Powder, Liquid, Gas, Plasma
}

#[derive(Clone, Copy, Debug)]
pub enum Action {
	Burn
}