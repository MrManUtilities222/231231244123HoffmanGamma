use crate::random::Random;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WeighedRandomItem {
    pub random_weight: i32,
}

impl WeighedRandomItem {
    pub fn new(random_weight: i32) -> Self {
        Self { random_weight }
    }

    pub fn is_valid(&self) -> bool {
        self.random_weight >= 0
    }
}

pub fn get_total_weight(items: &[WeighedRandomItem]) -> i32 {
    items.iter().map(|x| x.random_weight).sum()
}

pub fn get_random_item_index(random: &mut Random, items: &[WeighedRandomItem], total_weight: i32) -> i32 {
    if total_weight <= 0 || items.is_empty() {
        return -1;
    }

    let mut selection = random.next_int_n(total_weight);
    for (i, item) in items.iter().enumerate() {
        selection -= item.random_weight;
        if selection < 0 {
            return i as i32;
        }
    }
    -1
}

pub fn get_random_item_index_auto(random: &mut Random, items: &[WeighedRandomItem]) -> i32 {
    get_random_item_index(random, items, get_total_weight(items))
}

