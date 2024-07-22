use std::collections::HashMap;

use super::element::Element;

pub const EL_TEX_WIDTH: usize = 16;
pub const EL_TEX_HEIGHT: usize = 16;
type TextureData = [[[u8; 4]; EL_TEX_WIDTH]; EL_TEX_HEIGHT];

pub struct ElementTexHandler {
	textures: HashMap<Element, TextureData>
}

impl ElementTexHandler {
	pub fn new() -> Self {
		let mut textures = HashMap::new();

		textures.insert(Element::Solid, get_tex_data(include_bytes!("../assets/element_textures/wall.png")));
		textures.insert(Element::Gravel, get_tex_data(include_bytes!("../assets/element_textures/rock.png")));
		textures.insert(Element::Dirt, get_tex_data(include_bytes!("../assets/element_textures/dirt.png")));
		textures.insert(Element::SolidDirt, get_tex_data(include_bytes!("../assets/element_textures/dirt.png")));
		textures.insert(Element::Wood, get_tex_data(include_bytes!("../assets/element_textures/wood.png")));
		textures.insert(Element::Sand, get_tex_data(include_bytes!("../assets/element_textures/sand.png")));
		textures.insert(Element::Brick, get_tex_data(include_bytes!("../assets/element_textures/brick.png")));
		textures.insert(Element::Snow, get_tex_data(include_bytes!("../assets/element_textures/snow.png")));
		textures.insert(Element::Ice, get_tex_data(include_bytes!("../assets/element_textures/ice.png")));
		
		
		Self {
			textures
		}
	}

	pub fn get_texture(&self, element: Element) -> Option<&TextureData> {
		self.textures.get(&element)
	}
}

pub fn get_tex_data(data: &[u8]) -> TextureData {
	let img = image::load_from_memory(data).unwrap();
		
	let mut tex_data = [[[0; 4]; EL_TEX_WIDTH]; EL_TEX_HEIGHT];

	let img_data = img.into_rgba8();
	
    for (i, j, value) in img_data.enumerate_pixels() {
		tex_data[i as usize][j as usize][0] = value[0];
		tex_data[i as usize][j as usize][1] = value[1];
		tex_data[i as usize][j as usize][2] = value[2];
		tex_data[i as usize][j as usize][3] = value[3];
    }
	
	tex_data
}
