use notan::math::Vec2;

#[derive(Clone, Copy, Debug)]
pub struct Cell {
	pub element: Element,
	pub density: f32,
	pub state: State,
	pub color: [u8; 4],
	pub velocity: Vec2,
	pub drag: f32
}

pub fn air_element() -> Cell {
	Cell {
		element: Element::Air,
		density: 0.,
		state: State::Gas,
		color: [0, 0, 0, 255],
		velocity: Vec2::ZERO,
		drag: 1.
	}
}

pub fn solid_element() -> Cell {
	Cell {
		element: Element::Solid,
		density: 100.,
		state: State::Solid,
		color: [69, 62, 66, 255],
		velocity: Vec2::ZERO,
		drag: 0.
	}
}

pub fn sand_element() -> Cell {
	Cell {
		element: Element::Sand,
		density: 60.,
		state: State::Solid,
		color: [243, 239, 118, 255],
		velocity: Vec2::ZERO,
		drag: 0.9
	}
}

pub fn sawdust_element() -> Cell {
	Cell {
		element: Element::SawDust,
		density: 40.,
		state: State::Solid,
		color: [181, 137, 100, 255],
		velocity: Vec2::ZERO,
		drag: 0.9
	}
}

pub fn water_element() -> Cell {
	Cell {
		element: Element::Water,
		density: 50.,
		state:State::Liquid,
		color: [55, 46, 229, 255],
		velocity: Vec2::ZERO,
		drag: 0.5
	}
}

pub fn smoke_element() -> Cell {
	Cell {
		element: Element::Smoke,
		density: -10.,
		state: State::Gas,
		color: [42, 42, 42, 255],
		velocity: Vec2::ZERO,
		drag: 1.
	}
}


#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Element {
	Air, Solid, Sand, SawDust, Water, Smoke
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum State {
	Solid, Liquid, Gas, Plasma
}
