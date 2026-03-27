#[derive(Clone, Copy, Debug)]
pub struct Material {
    pub flammable: bool,
    pub never_buildable: bool,
    pub is_always_destroyable: bool,
    pub replaceable: bool,
}

impl Material {
    pub fn new() -> Self {
        Self {
            flammable: false,
            never_buildable: false,
            is_always_destroyable: true,
            replaceable: false,
        }
    }

    pub fn flammable(mut self) -> Self {
        self.flammable = true;
        self
    }

    pub fn never_buildable(mut self) -> Self {
        self.never_buildable = true;
        self
    }

    pub fn not_always_destroyable(mut self) -> Self {
        self.is_always_destroyable = false;
        self
    }

    pub fn replaceable(mut self) -> Self {
        self.replaceable = true;
        self
    }

    pub fn is_liquid(&self) -> bool {
        false
    }

    pub fn lets_water_through(&self) -> bool {
        !self.is_liquid() && !self.is_solid()
    }

    pub fn is_solid(&self) -> bool {
        true
    }

    pub fn blocks_light(&self) -> bool {
        true
    }

    pub fn is_solid_blocking(&self) -> bool {
        if self.never_buildable {
            return false;
        }
        self.blocks_motion()
    }

    pub fn is_always_destroyable(&self) -> bool {
        self.is_always_destroyable
    }

    pub fn blocks_motion(&self) -> bool {
        true
    }

    pub fn is_flammable(&self) -> bool {
        self.flammable
    }

    pub fn is_replaceable(&self) -> bool {
        self.replaceable
    }
}

// Static materials
lazy_static::lazy_static! {
    pub static ref AIR: Material = Material::new().never_buildable();
    pub static ref DIRT: Material = Material::new();
    pub static ref WOOD: Material = Material::new().flammable();
    pub static ref STONE: Material = Material::new();
    pub static ref METAL: Material = Material::new();
    pub static ref WATER: Material = Material::new().never_buildable();
    pub static ref LAVA: Material = Material::new().never_buildable();
    pub static ref LEAVES: Material = Material::new().flammable();
    pub static ref PLANT: Material = Material::new().never_buildable().replaceable();
    pub static ref REPLACEABLE_PLANT: Material = Material::new().never_buildable().replaceable();
    pub static ref SPONGE: Material = Material::new();
    pub static ref CLOTH: Material = Material::new().flammable();
    pub static ref FIRE: Material = Material::new().never_buildable().not_always_destroyable();
    pub static ref SAND: Material = Material::new();
    pub static ref DECORATION: Material = Material::new().never_buildable();
    pub static ref GLASS: Material = Material::new();
    pub static ref EXPLOSIVE: Material = Material::new().flammable();
    pub static ref CORAL: Material = Material::new();
    pub static ref ICE: Material = Material::new();
    pub static ref TOP_SNOW: Material = Material::new().never_buildable().replaceable();
    pub static ref SNOW: Material = Material::new().never_buildable().replaceable();
    pub static ref CACTUS: Material = Material::new();
    pub static ref CLAY: Material = Material::new();
    pub static ref VEGETABLE: Material = Material::new().never_buildable().replaceable();
    pub static ref PORTAL: Material = Material::new().never_buildable();
    pub static ref CAKE: Material = Material::new();
    pub static ref WEB: Material = Material::new();
}

pub fn init_materials() {
    // Initialize statics if needed
}