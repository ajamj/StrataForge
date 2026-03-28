pub struct SeismicVolume {
    pub data: Vec<f32>, // Flat array for now
    pub width: usize,   // Inlines
    pub height: usize,  // Xlines
    pub depth: usize,   // Samples
}

impl SeismicVolume {
    pub fn get_inline(&self, inline_idx: usize) -> Vec<f32> {
        // Extract a 2D slice from the 3D volume
        // Assuming data is stored as [inline][crossline][sample]
        let slice_size = self.height * self.depth;
        let start = inline_idx * slice_size;
        let end = start + slice_size;

        if end <= self.data.len() {
            self.data[start..end].to_vec()
        } else {
            Vec::new()
        }
    }

    pub fn get_crossline(&self, crossline_idx: usize) -> Vec<f32> {
        // Extract crossline across all inlines
        // Data is [inline][crossline][sample]
        let mut slice = Vec::with_capacity(self.width * self.depth);
        for i in 0..self.width {
            let start = i * self.height * self.depth + crossline_idx * self.depth;
            let end = start + self.depth;
            if end <= self.data.len() {
                slice.extend_from_slice(&self.data[start..end]);
            }
        }
        slice
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_inline() {
        let width = 2;
        let height = 2;
        let depth = 2;
        let data = (0..width * height * depth).map(|x| x as f32).collect();
        let volume = SeismicVolume { data, width, height, depth };

        // Inline 0 should have samples for all height/depth at inline 0
        let inline0 = volume.get_inline(0);
        assert_eq!(inline0, vec![0.0, 1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_get_crossline() {
        let width = 2;
        let height = 2;
        let depth = 2;
        let data = (0..width * height * depth).map(|x| x as f32).collect();
        let volume = SeismicVolume { data, width, height, depth };

        // Crossline 0 should have samples for all inlines at crossline 0
        // data[0*2*2 + 0*2 + 0] = 0
        // data[0*2*2 + 0*2 + 1] = 1
        // data[1*2*2 + 0*2 + 0] = 4
        // data[1*2*2 + 0*2 + 1] = 5
        let xline0 = volume.get_crossline(0);
        assert_eq!(xline0, vec![0.0, 1.0, 4.0, 5.0]);
    }
}
