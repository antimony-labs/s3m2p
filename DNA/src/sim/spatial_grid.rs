use glam::Vec2;

use super::arena::BoidArena;

/// Spatial grid with fixed-size cells (no heap allocation per cell)
pub struct SpatialGrid<const CELL_CAPACITY: usize> {
    cell_size: f32,
    cols: usize,
    rows: usize,
    // Each cell stores up to CELL_CAPACITY indices
    cells: Vec<[u16; CELL_CAPACITY]>,
    cell_counts: Vec<u8>,
}

impl<const CELL_CAPACITY: usize> SpatialGrid<CELL_CAPACITY> {
    pub fn new(width: f32, height: f32, cell_size: f32) -> Self {
        let cols = ((width / cell_size).ceil() as usize).max(1);
        let rows = ((height / cell_size).ceil() as usize).max(1);
        let cell_count = cols * rows;

        Self {
            cell_size,
            cols,
            rows,
            cells: vec![[0; CELL_CAPACITY]; cell_count],
            cell_counts: vec![0; cell_count],
        }
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        let new_cols = ((width / self.cell_size).ceil() as usize).max(1);
        let new_rows = ((height / self.cell_size).ceil() as usize).max(1);

        if new_cols != self.cols || new_rows != self.rows {
            self.cols = new_cols;
            self.rows = new_rows;
            let cell_count = new_cols * new_rows;
            self.cells.resize(cell_count, [0; CELL_CAPACITY]);
            self.cell_counts.resize(cell_count, 0);
        }
    }

    #[inline]
    fn cell_index(&self, pos: Vec2) -> usize {
        let col = ((pos.x / self.cell_size) as usize).min(self.cols.saturating_sub(1));
        let row = ((pos.y / self.cell_size) as usize).min(self.rows.saturating_sub(1));
        row * self.cols + col
    }

    /// Clear all cells (O(cells) not O(boids))
    pub fn clear(&mut self) {
        for count in &mut self.cell_counts {
            *count = 0;
        }
    }

    /// Insert boid index into grid
    #[inline]
    pub fn insert(&mut self, idx: u16, pos: Vec2) {
        let cell_idx = self.cell_index(pos);
        let count = self.cell_counts[cell_idx] as usize;
        if count < CELL_CAPACITY {
            self.cells[cell_idx][count] = idx;
            self.cell_counts[cell_idx] += 1;
        }
    }

    /// Build grid from arena (only alive boids)
    pub fn build<const CAP: usize>(&mut self, arena: &BoidArena<CAP>) {
        self.clear();
        for idx in arena.iter_alive() {
            self.insert(idx as u16, arena.positions[idx]);
        }
    }

    /// Query neighbors, writes indices to output buffer, returns count
    pub fn query_neighbors<const CAP: usize>(
        &self,
        pos: Vec2,
        radius: f32,
        arena: &BoidArena<CAP>,
        exclude_idx: usize,
        output: &mut [u16],
    ) -> usize {
        let radius_sq = radius * radius;
        let mut count = 0;

        let min_col = ((pos.x - radius) / self.cell_size).floor().max(0.0) as usize;
        let max_col = (((pos.x + radius) / self.cell_size).ceil() as usize).min(self.cols);
        let min_row = ((pos.y - radius) / self.cell_size).floor().max(0.0) as usize;
        let max_row = (((pos.y + radius) / self.cell_size).ceil() as usize).min(self.rows);

        for row in min_row..max_row {
            for col in min_col..max_col {
                let cell_idx = row * self.cols + col;
                let cell_count = self.cell_counts[cell_idx] as usize;

                for i in 0..cell_count {
                    let other_idx = self.cells[cell_idx][i] as usize;
                    if other_idx == exclude_idx {
                        continue;
                    }

                    let dist_sq = (arena.positions[other_idx] - pos).length_squared();
                    if dist_sq < radius_sq && count < output.len() {
                        output[count] = other_idx as u16;
                        count += 1;
                    }
                }
            }
        }

        count
    }

    /// Count neighbors (no allocation)
    #[inline]
    pub fn count_neighbors<const CAP: usize>(
        &self,
        pos: Vec2,
        radius: f32,
        arena: &BoidArena<CAP>,
        exclude_idx: usize,
    ) -> usize {
        let mut neighbors = [0u16; 64];
        self.query_neighbors(pos, radius, arena, exclude_idx, &mut neighbors)
    }
}
