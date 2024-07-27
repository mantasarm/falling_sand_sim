use contour::ContourBuilder;
use simplify_polyline::*;

use super::{chunk::{Chunk, COLS, ROWS, UPSCALE_FACTOR}, element::State, rapier_world_handler::PHYS_SCALE};

pub fn edges_from_chunk(chunk: &mut Chunk) {
	let mut map = vec![];
	for i in 0..ROWS {
		for j in 0..COLS {
			if chunk.grid[j][i].state == State::Solid {
				map.push(1.);
			} else {
				map.push(0.);
			}
		}
	}
	
	let c = ContourBuilder::new(COLS, ROWS, false);
	let edges = c.contours(&map, &[0.5]).unwrap();

	let mut gotten_edges = vec![];
	for edge in edges {
		for polygon in &edge.geometry().0 {
			let mut edge_points = vec![];
			for point in polygon.exterior().points() {
				edge_points.push(point!(point.0.x as f32, point.0.y as f32));
			}
			gotten_edges.push(edge_points);

			for interior in polygon.interiors().to_owned() {
				let mut edge_interior_points = vec![];
				
				for point in interior.points() {
					edge_interior_points.push(point!(point.0.x as f32, point.0.y as f32));
				}
				if !edge_interior_points.is_empty() {
					gotten_edges.push(edge_interior_points);
				}
			}
		}
	}

	let mut simplified = vec![];
	for edge in gotten_edges {
		let points = simplify(&edge, 1., true);
		simplified.push(points);
	}

	let mut simplified_formated = vec![];
	for edge in simplified {
		let mut points = vec![];
		for point in edge {
			points.push(rapier2d::math::Point::new(point.vec[0] / (PHYS_SCALE / UPSCALE_FACTOR), point.vec[1] / (PHYS_SCALE / UPSCALE_FACTOR)));
		}
		simplified_formated.push(points);
	}

	chunk.edges = simplified_formated;
}
