use contour::ContourBuilder;
use notan::{draw::{Draw, DrawShapes}, prelude::Color, math::{Mat3, Vec2}};
use rapier2d::{prelude::{RigidBodyHandle, RigidBodyBuilder, ColliderBuilder, RigidBodySet, ColliderSet, nalgebra}, na::vector, parry::transformation::vhacd::VHACDParameters};
use simplify_polyline::*;

use super::{element::*, rapier_world_handler::{PHYS_SCALE, SelectBody}, chunk::UPSCALE_FACTOR, element_texture_handler::{EL_TEX_WIDTH, EL_TEX_HEIGHT, ElementTexHandler}};

pub type RSBodyEdge = Vec<rapier2d::na::OPoint<f32, rapier2d::na::Const<2>>>;

pub struct ElInWorldInfo {
	pub chunk: (i32, i32),
	pub index_chunk: (usize, usize),
	pub index_body: (usize, usize)
}

pub struct RigidSandBody {
	pub body_elements: Vec<Vec<Option<Cell>>>,
	pub body_elements_in_chunks: Vec<ElInWorldInfo>,
	pub rigid_body_handle: RigidBodyHandle,
	pub body_edge: RSBodyEdge
}

impl RigidSandBody {
	pub fn new(x: f32, y: f32, rigid_body_set: &mut RigidBodySet, collider_set: &mut ColliderSet, element_texs: &ElementTexHandler, body_shape: SelectBody) -> Self {
		let mut body_elements = vec![];

		match body_shape {
		    SelectBody::Ball => (),
		    SelectBody::SandBodyBall => {
				for i in 0..100 {
					let mut row = vec![];
					for j in 0..100 {
						let mut element = wood_element();
						element.collider_type = ElColliderType::Body;
				        if let Some(tex_data) = element_texs.get_texture(element.element) {
				            element.color = tex_data[i as usize % (EL_TEX_WIDTH)][j as usize % (EL_TEX_HEIGHT)];
				        }
						row.push(Some(element));
					}
					body_elements.push(row);
				}
		
				for i in 0..100 {
					for j in 0..100 {
						if Vec2::new(i as f32, j as f32).distance(Vec2::new(50., 50.)) > 50. {
							body_elements[i][j] = None;
						}
					}
				}
			},
		    SelectBody::SandBodySquare =>  {
				for i in 0..10 {
					let mut row = vec![];
					for j in 0..10 {
						let mut element = wood_element();
						element.collider_type = ElColliderType::Body;
				        if let Some(tex_data) = element_texs.get_texture(element.element) {
				            element.color = tex_data[i as usize % (EL_TEX_WIDTH)][j as usize % (EL_TEX_HEIGHT)];
				        }
						row.push(Some(element));
					}
					body_elements.push(row);
				}
			},
		    SelectBody::SandBodyRectangle => {
				for i in 0..100 {
					let mut row = vec![];
					for j in 0..50 {
						let mut element = wood_element();
						element.collider_type = ElColliderType::Body;
				        if let Some(tex_data) = element_texs.get_texture(element.element) {
				            element.color = tex_data[i as usize % (EL_TEX_WIDTH)][j as usize % (EL_TEX_HEIGHT)];
				        }
						row.push(Some(element));
					}
					body_elements.push(row);
				}
			},
		}

		let (rigid_body_handle, final_edge) = create_rigid_body_handle(x, y, &body_elements, rigid_body_set, collider_set);
		

		Self {
			body_elements,
			body_elements_in_chunks: vec![],
			rigid_body_handle,
			body_edge: final_edge,
		}
	}

	pub fn debug_render(&self, draw: &mut Draw, rigid_body_set: &RigidBodySet) {
		let body = &rigid_body_set[self.rigid_body_handle];

		let translation = Mat3::from_translation(Vec2::new(body.translation().x * PHYS_SCALE, body.translation().y * PHYS_SCALE));
		let rotation = Mat3::from_angle(body.rotation().angle());
		let matrix = translation * rotation;

		draw.transform().push(matrix);
		
	    let mut prev_point = &self.body_edge[0];
	    for point in &self.body_edge {
	        draw.line(
	            ((prev_point.x) * UPSCALE_FACTOR * PHYS_SCALE / UPSCALE_FACTOR,
	             (prev_point.y) * UPSCALE_FACTOR * PHYS_SCALE / UPSCALE_FACTOR),
	            ((point.x) * UPSCALE_FACTOR * PHYS_SCALE / UPSCALE_FACTOR,
	             (point.y) * UPSCALE_FACTOR * PHYS_SCALE / UPSCALE_FACTOR)
	        ).color(Color::MAGENTA);
	        prev_point = point;
	    }
		draw.transform().pop();
	}

	pub fn remove_from_rapier(&mut self, ) {
		
	}
}

// INFO: Create a body map for the tracing algorithm to use
pub fn gen_body_map(body_elements: &Vec<Vec<Option<Cell>>>) -> Vec<f64> {
	let mut map = vec![];
	for i in 0..body_elements[0].len() {
		for j in 0..body_elements.len() {
			if body_elements[j][i].is_some() {
				map.push(1.);
			} else {
				map.push(0.);
			}
		}
	}

	map
}

/*
	We use the "contour" crate for getting all edges from a body map, that includes exteriors and interiors
	Then we simplify the edges with the "simplify-polyline" crate
*/
pub fn get_edge_from_body_map(body_map: Vec<f64>, body_elements: &Vec<Vec<Option<Cell>>>) -> Vec<Vec<nalgebra::OPoint<f32, nalgebra::Const<2>>>> {
	// INFO: Get all unsimplified edges from the body map
	let c = ContourBuilder::new(body_elements.len(), body_elements[0].len(), false);
	let edges = c.contours(&body_map, &[0.5]).unwrap();

	// INFO: For now we only get the exterior points of the shape and ignore holes
	let mut gotten_edges = vec![];
	for edge in edges {
		for polygon in &edge.geometry().0 {
			let mut edge_points = vec![];
			for point in polygon.exterior().points() {
				edge_points.push(point!(point.0.x as f32, point.0.y as f32));
			}
			gotten_edges.push(edge_points);
		}
	}

	// INFO: Simplify the gotten edges for better rapier2d performance
	let mut simplified = vec![];
	for edge in gotten_edges {
		let points = simplify(&edge, 1., true);
		simplified.push(points);
	}

	// INFO: Format the simplified edges to use with rapier2d
	let mut simplified_formated = vec![];
	for edge in simplified {
		let mut points = vec![];
		for point in edge {
			points.push(rapier2d::math::Point::new(point.vec[0] / (PHYS_SCALE / UPSCALE_FACTOR), point.vec[1] / (PHYS_SCALE / UPSCALE_FACTOR)));
		}
		simplified_formated.push(points);
	}

	simplified_formated
}

// INFO: Here we place the rigid sand body into the rapier world
fn create_rigid_body_handle(
		x: f32, y: f32,
		body_elements: &Vec<Vec<Option<Cell>>>, 
		rigid_body_set: &mut RigidBodySet,
		collider_set: &mut ColliderSet) -> (RigidBodyHandle, RSBodyEdge)
{
	// INFO: Create the body map
	let body_map = gen_body_map(&body_elements);

	// INFO: Get edges from the body map
	let final_edge = get_edge_from_body_map(body_map, &body_elements)[0].to_owned();
	
	let rigid_body = RigidBodyBuilder::dynamic().translation(vector![x, y]).build();

	let indices: Vec<[u32; 2]> = (0..final_edge.len() - 1).map(|i| [i as u32, i as u32 + 1]).collect();

	// INFO: Here we set the accuracy of the generated shape
	let mut params = VHACDParameters::default();
	params.concavity = 0.01;
	
	let collider = ColliderBuilder::convex_decomposition_with_params(&final_edge, &indices, &params).build();

	let rigid_body_handle = rigid_body_set.insert(rigid_body);
	collider_set.insert_with_parent(collider, rigid_body_handle, rigid_body_set);

	(rigid_body_handle, final_edge)
}
