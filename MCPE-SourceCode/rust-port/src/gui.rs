use crate::tesselator::Tesselator;
use crate::inventory::Inventory;

pub struct Gui {
    // Tesselators for different texture bindings
    pub gui_mesh: Vec<f32>,
    pub item_mesh: Vec<f32>,
}

impl Gui {
    pub fn new() -> Self {
        Self {
            gui_mesh: Vec::new(),
            item_mesh: Vec::new(),
        }
    }

    fn blit(&self, t: &mut Tesselator, x: f32, y: f32, w: f32, h: f32, tx: f32, ty: f32, tw: f32, th: f32, tex_w: f32, tex_h: f32) {
        let u0 = tx / tex_w;
        let v0 = ty / tex_h;
        let u1 = (tx + tw) / tex_w;
        let v1 = (ty + th) / tex_h;
        
        t.color(1.0, 1.0, 1.0);
        // Triangle 1
        t.vertex_uv(x, y + h, 0.0, u0, v1);
        t.vertex_uv(x + w, y + h, 0.0, u1, v1);
        t.vertex_uv(x + w, y, 0.0, u1, v0);

        // Triangle 2
        t.vertex_uv(x + w, y, 0.0, u1, v0);
        t.vertex_uv(x, y, 0.0, u0, v0);
        t.vertex_uv(x, y + h, 0.0, u0, v1);
    }

    pub fn render(&mut self, width: f32, height: f32, inventory: &Inventory) {
        let mut t = Tesselator::new();
        t.begin();

        // 1. Crosshair (Middle of screen)
        // MCPE gui.png has crosshair at x=240, y=0, width=15, height=15. Total 256x256.
        let cx = (width - 15.0) / 2.0;
        let cy = (height - 15.0) / 2.0;
        self.blit(&mut t, cx, cy, 15.0, 15.0, 240.0, 0.0, 15.0, 15.0, 256.0, 256.0);

        // 2. Hotbar Frames (Bottom Center)
        // MCPE hotbar frame: x=0, y=0, w=182, h=22
        let scale = 2.0;
        let hw = 182.0 * scale;
        let hh = 22.0 * scale;
        let hx = (width - hw) / 2.0;
        let hy = height - hh; // Bottom

        self.blit(&mut t, hx, hy, hw, hh, 0.0, 0.0, 182.0, 22.0, 256.0, 256.0);

        // Selected Slot Overlay
        // MCPE selection box: x=0, y=22, w=24, h=24
        let sel_slot = inventory.selected;
        let sel_w = 24.0 * scale;
        let sel_h = 24.0 * scale;
        let sel_x = hx + (sel_slot as f32 * 20.0 * scale) - (1.0 * scale);
        let sel_y = hy - (1.0 * scale);

        self.blit(&mut t, sel_x, sel_y, sel_w, sel_h, 0.0, 22.0, 24.0, 24.0, 256.0, 256.0);
        self.gui_mesh = t.end();

        // 3. Hotbar Item Icons (Using terrain.png)
        let mut item_t = Tesselator::new();
        item_t.begin();

        let item_scale = 16.0 * scale; // 32x32 pixels on screen
        for i in 0..Inventory::MAX_SELECTION_SIZE {
            if let Some(item) = inventory.container.get_linked(i) {
                if item.id > 0 && !item.is_null() {
                    let mut tex_id = 1;
                    if item.id == crate::tile::GRASS.id { tex_id = 3; } // Side texture for icon
                    if item.id == crate::tile::DIRT.id { tex_id = 2; }
                    if item.id == crate::tile::LOG.id { tex_id = 20; }
                    if item.id == crate::tile::LEAVES.id { tex_id = 52; }
                    if item.id == crate::tile::SAND.id { tex_id = 18; }
                    if item.id == crate::tile::CACTUS.id { tex_id = 70; }
                    
                    let tx = (tex_id % 16) as f32 * 16.0;
                    let ty = (tex_id / 16) as f32 * 16.0;

                    let slot_x = hx + (i as f32 * 20.0 * scale) + (3.0 * scale);
                    let slot_y = hy + (3.0 * scale);

                    self.blit(&mut item_t, slot_x, slot_y, item_scale, item_scale, tx, ty, 16.0, 16.0, 256.0, 256.0);
                }
            }
        }
        self.item_mesh = item_t.end();
    }
}
