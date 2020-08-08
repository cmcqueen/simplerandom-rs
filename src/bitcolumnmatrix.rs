
pub struct BitColumnMatrix32 {
    columns: [u32; 32],
}

impl BitColumnMatrix32 {
    pub fn new(init_data: &[u32; 32]) -> BitColumnMatrix32 {
        BitColumnMatrix32 {
            columns: *init_data,
        }
    }
}
