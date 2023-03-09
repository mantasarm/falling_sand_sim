use notan::math::Vec2;

#[derive(Clone, Copy)]
pub struct Cell {
	pub element: Element,
	pub color: [u8; 4],
	pub velocity: Vec2,
}

pub fn air_element() -> Cell {
	Cell {
		element: Element::Air,
		color: [0, 0, 0, 255],
		velocity: Vec2::default(),
	}
}

pub fn solid_element() -> Cell {
	Cell {
		element: Element::Solid,
		color: [69, 62, 66, 255],
		velocity: Vec2::default(),
	}
}

pub fn sand_element() -> Cell {
	Cell {
		element: Element::Sand,
		color: [243, 239, 118, 255],
		velocity: Vec2::new(-0., -0.),
	}
}

pub fn sawdust_element() -> Cell {
	Cell {
		element: Element::SawDust,
		color: [181, 137, 100, 255],
		velocity: Vec2::default()
	}
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Element {
	Air, Solid, Sand, SawDust
}
