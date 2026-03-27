use crate::item_instance::ItemInstance;

pub struct FillingContainer {
    pub items: Vec<Option<ItemInstance>>,
    pub linked_slots: Vec<i32>,
    pub num_total_slots: usize,
    pub num_linked_slots: usize,
    pub is_creative: bool,
}

impl FillingContainer {
    pub fn new(num_total_slots: usize, num_linked_slots: usize, is_creative: bool) -> Self {
        let mut items = Vec::with_capacity(num_total_slots);
        for _ in 0..num_total_slots {
            items.push(None);
        }
        
        let mut linked_slots = Vec::with_capacity(num_linked_slots);
        for i in 0..num_linked_slots {
            linked_slots.push(-1);
        }

        Self {
            items,
            linked_slots,
            num_total_slots,
            num_linked_slots,
            is_creative,
        }
    }

    pub fn clear_inventory(&mut self) {
        for i in 0..self.num_linked_slots {
            self.linked_slots[i] = i as i32;
        }

        for i in self.num_linked_slots..self.num_total_slots {
            self.items[i] = None;
        }
    }

    pub fn get_linked(&self, slot: usize) -> Option<&ItemInstance> {
        if slot < self.num_linked_slots {
            let inv_slot = self.linked_slots[slot];
            if inv_slot >= 0 && (inv_slot as usize) < self.num_total_slots {
                return self.items[inv_slot as usize].as_ref();
            }
        }
        None
    }

    pub fn link_empty_slot(&mut self, inventory_slot: usize) -> bool {
        // Check if already linked
        for i in 0..self.num_linked_slots {
            if self.linked_slots[i] == inventory_slot as i32 {
                return true;
            }
        }

        // Find empty link slot
        for i in 0..self.num_linked_slots {
            if self.get_linked(i).is_none() {
                self.linked_slots[i] = inventory_slot as i32;
                return true;
            }
        }
        false
    }
    
    pub fn get_free_slot(&self) -> Option<usize> {
        for i in self.num_linked_slots..self.num_total_slots {
            if let Some(item) = &self.items[i] {
                if item.is_null() {
                    return Some(i);
                }
            } else {
                return Some(i);
            }
        }
        None
    }

    pub fn add_item(&mut self, mut item: ItemInstance) -> bool {
        if self.is_creative {
            return true;
        }

        if let Some(slot) = self.get_free_slot() {
            self.items[slot] = Some(item);
            self.link_empty_slot(slot);
            return true;
        }
        false
    }
}

pub struct Inventory {
    pub container: FillingContainer,
    pub selected: usize,
}

impl Inventory {
    pub const MAX_SELECTION_SIZE: usize = 9;

    pub fn new(creative_mode: bool) -> Self {
        let mut inv = Self {
            container: FillingContainer::new(36 + Self::MAX_SELECTION_SIZE, Self::MAX_SELECTION_SIZE, creative_mode),
            selected: 0,
        };
        inv.setup_default();
        inv
    }

    pub fn setup_default(&mut self) {
        self.container.clear_inventory();

        // Initialize with default standard blocks
        let default_blocks = vec![
            crate::tile::STONE.id,
            crate::tile::DIRT.id,
            crate::tile::GRASS.id,
            crate::tile::LOG.id,
            crate::tile::LEAVES.id,
            crate::tile::SAND.id,
            crate::tile::CACTUS.id,
        ];

        for (i, &block_id) in default_blocks.iter().enumerate() {
            let item = ItemInstance::from_id(block_id.into(), 64, 0);
            let slot = self.container.num_linked_slots + i;
            if slot < self.container.num_total_slots {
                self.container.items[slot] = Some(item);
                self.container.linked_slots[i] = slot as i32;
            }
        }
    }

    pub fn select_slot(&mut self, slot: usize) {
        if slot < Self::MAX_SELECTION_SIZE {
            self.selected = slot;
        }
    }

    pub fn get_selected(&self) -> Option<&ItemInstance> {
        self.container.get_linked(self.selected)
    }
}
