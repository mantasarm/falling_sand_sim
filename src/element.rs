use notan::math::Vec2;

#[derive(Clone, Copy, Debug)]
pub struct Cell {
	pub element: Element,
	pub density: f32,
	pub state: State,
	pub color: [u8; 4],
	pub velocity: Vec2,
	pub drag: f32,
	pub sleeping: bool
}

pub fn air_element() -> Cell {
	Cell {
		element: Element::Air,
		density: 0.,
		state: State::Gas,
		color: [0, 0, 0, 0],
		velocity: Vec2::ZERO,
		drag: 1.,
		sleeping: true
	}
}

pub fn solid_element() -> Cell {
	Cell {
		element: Element::Solid,
		density: 100.,
		state: State::Solid,
		color: [69, 62, 66, 255],
		velocity: Vec2::ZERO,
		drag: 0.,
		sleeping: false
	}
}

pub fn sand_element() -> Cell {
	Cell {
		element: Element::Sand,
		density: 60.,
		state: State::Powder,
		color: [243, 239, 118, 255],
		velocity: Vec2::ZERO,
		drag: 0.9,
		sleeping: false
	}
}

pub fn sawdust_element() -> Cell {
	Cell {
		element: Element::SawDust,
		density: 40.,
		state: State::Powder,
		color: [181, 137, 100, 255],
		velocity: Vec2::ZERO,
		drag: 0.9,
		sleeping: false
	}
}

pub fn water_element() -> Cell {
	Cell {
		element: Element::Water,
		density: 50.,
		state:State::Liquid,
		color: [55, 46, 229, 200],
		velocity: Vec2::ZERO,
		drag: 0.6,
		sleeping: false
	}
}

pub fn smoke_element() -> Cell {
	Cell {
		element: Element::Smoke,
		density: -20.,
		state: State::Gas,
		color: [42, 42, 42, 220],
		velocity: Vec2::ZERO,
		drag: 1.,
		sleeping: false
	}
}

pub fn steam_element() -> Cell {
	Cell {
		element: Element::Steam,
		density: -10.,
		state: State::Gas,
		color: [143, 159, 234, 175],
		velocity: Vec2::ZERO,
		drag: 1.,
		sleeping: false
	}
}


#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Element {
	Air, Solid, Sand, SawDust, Water, Steam, Smoke
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum State {
	Solid, Powder, Liquid, Gas, Plasma
}
