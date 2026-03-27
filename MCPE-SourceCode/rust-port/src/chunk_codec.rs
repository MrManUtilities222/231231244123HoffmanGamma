use crate::data_layer::DataLayer;

pub const CHUNK_WIDTH: usize = 16;
pub const CHUNK_DEPTH: usize = 16;
pub const CHUNK_HEIGHT: usize = 128;
pub const CHUNK_COLUMNS: usize = CHUNK_WIDTH * CHUNK_DEPTH;
pub const CHUNK_BLOCK_COUNT: usize = CHUNK_WIDTH * CHUNK_DEPTH * CHUNK_HEIGHT;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkBuffers {
    pub blocks: Vec<u8>,
    pub data: DataLayer,
    pub block_light: DataLayer,
    pub sky_light: DataLayer,
}

impl Default for ChunkBuffers {
    fn default() -> Self {
        Self {
            blocks: vec![0; CHUNK_BLOCK_COUNT],
            data: DataLayer::new(CHUNK_BLOCK_COUNT),
            block_light: DataLayer::new(CHUNK_BLOCK_COUNT),
            sky_light: DataLayer::new(CHUNK_BLOCK_COUNT),
        }
    }
}

impl ChunkBuffers {
    // Mirrors LevelChunk::getBlocksAndData memory layout.
    pub fn encode_range(
        &self,
        x0: usize,
        y0: usize,
        z0: usize,
        x1: usize,
        y1: usize,
        z1: usize,
    ) -> Vec<u8> {
        let mut out = Vec::new();
        let len = y1 - y0;
        for x in x0..x1 {
            for z in z0..z1 {
                let slot = (x << 11) | (z << 7) | y0;
                out.extend_from_slice(&self.blocks[slot..(slot + len)]);
            }
        }

        let nlen = (y1 - y0) / 2;
        for x in x0..x1 {
            for z in z0..z1 {
                let slot = ((x << 11) | (z << 7) | y0) >> 1;
                out.extend_from_slice(&self.data.data[slot..(slot + nlen)]);
            }
        }
        for x in x0..x1 {
            for z in z0..z1 {
                let slot = ((x << 11) | (z << 7) | y0) >> 1;
                out.extend_from_slice(&self.block_light.data[slot..(slot + nlen)]);
            }
        }
        for x in x0..x1 {
            for z in z0..z1 {
                let slot = ((x << 11) | (z << 7) | y0) >> 1;
                out.extend_from_slice(&self.sky_light.data[slot..(slot + nlen)]);
            }
        }
        out
    }

    pub fn decode_range(
        &mut self,
        input: &[u8],
        x0: usize,
        y0: usize,
        z0: usize,
        x1: usize,
        y1: usize,
        z1: usize,
    ) -> usize {
        let mut p = 0usize;
        let len = y1 - y0;
        for x in x0..x1 {
            for z in z0..z1 {
                let slot = (x << 11) | (z << 7) | y0;
                self.blocks[slot..(slot + len)].copy_from_slice(&input[p..(p + len)]);
                p += len;
            }
        }

        let nlen = (y1 - y0) / 2;
        for x in x0..x1 {
            for z in z0..z1 {
                let slot = ((x << 11) | (z << 7) | y0) >> 1;
                self.data.data[slot..(slot + nlen)].copy_from_slice(&input[p..(p + nlen)]);
                p += nlen;
            }
        }
        for x in x0..x1 {
            for z in z0..z1 {
                let slot = ((x << 11) | (z << 7) | y0) >> 1;
                self.block_light.data[slot..(slot + nlen)].copy_from_slice(&input[p..(p + nlen)]);
                p += nlen;
            }
        }
        for x in x0..x1 {
            for z in z0..z1 {
                let slot = ((x << 11) | (z << 7) | y0) >> 1;
                self.sky_light.data[slot..(slot + nlen)].copy_from_slice(&input[p..(p + nlen)]);
                p += nlen;
            }
        }
        p
    }
}

