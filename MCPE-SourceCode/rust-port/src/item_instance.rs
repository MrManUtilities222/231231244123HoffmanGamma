use crate::item::Item;
use crate::tile::Tile;

#[derive(Clone, Debug, PartialEq)]
pub struct ItemInstance {
    pub id: i32,
    pub count: i32,
    pub aux_value: i32,
}

impl Default for ItemInstance {
    fn default() -> Self {
        Self {
            id: 0,
            count: 0,
            aux_value: 0,
        }
    }
}

impl ItemInstance {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_tile(tile: &Tile, count: i32, aux_value: i32) -> Self {
        Self {
            id: tile.id as i32,
            count,
            aux_value,
        }
    }

    pub fn from_item(item: &Item, count: i32, aux_value: i32) -> Self {
        Self {
            id: item.id,
            count,
            aux_value,
        }
    }

    pub fn from_id(id: i32, count: i32, aux_value: i32) -> Self {
        Self {
            id,
            count,
            aux_value,
        }
    }

    pub fn is_null(&self) -> bool {
        self.id == 0 || self.count <= 0
    }

    pub fn set_null(&mut self) {
        self.id = 0;
        self.count = 0;
        self.aux_value = 0;
    }

    pub fn remove(&mut self, count: i32) -> ItemInstance {
        let removed_count = std::cmp::min(self.count, count);
        self.count -= removed_count;
        ItemInstance::from_id(self.id, removed_count, self.aux_value)
    }

    // In a full implementation, this looks up the actual Item registry.
    // For now, hardcode standard item rules or use basic defaults.
    pub fn get_max_stack_size(&self) -> i32 {
        if self.id < 256 {
            64 // Block tiles stack to 64
        } else {
            // Look up from the item definitions
            crate::item::get_max_stack_size(self.id)
        }
    }

    pub fn is_stackable(&self) -> bool {
        self.get_max_stack_size() > 1 && (!self.is_damageable_item() || !self.is_damaged())
    }

    pub fn is_stackable_with(a: &ItemInstance, b: &ItemInstance) -> bool {
        a.id == b.id && b.is_stackable() && (!b.is_stacked_by_data() || a.aux_value == b.aux_value)
    }

    pub fn is_damageable_item(&self) -> bool {
        false // Dummy implementation
    }

    pub fn is_stacked_by_data(&self) -> bool {
        if self.id < 256 { true } else { false } // Dummy implementation
    }

    pub fn is_damaged(&self) -> bool {
        self.is_damageable_item() && self.aux_value > 0
    }

    pub fn get_damage_value(&self) -> i32 {
        self.aux_value
    }

    pub fn get_aux_value(&self) -> i32 {
        self.aux_value
    }

    pub fn set_aux_value(&mut self, value: i32) {
        self.aux_value = value;
    }

    pub fn get_max_damage(&self) -> i32 {
        0
    }

    pub fn hurt(&mut self, amount: i32) {
        if !self.is_damageable_item() {
            return;
        }

        self.aux_value += amount;
        if self.aux_value > self.get_max_damage() {
            self.count -= 1;
            if self.count < 0 {
                self.count = 0;
            }
            self.aux_value = 0;
        }
    }

    pub fn matches(a: &ItemInstance, b: &ItemInstance) -> bool {
        a.id == b.id && a.count == b.count && a.aux_value == b.aux_value
    }

    pub fn same_item(&self, b: &ItemInstance) -> bool {
        self.id == b.id && self.aux_value == b.aux_value
    }

    pub fn to_string(&self) -> String {
        format!("{} x {}({})@{}", self.count, "item", self.id, self.aux_value)
    }
}
