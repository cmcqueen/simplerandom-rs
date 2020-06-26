#[derive(Debug)]
pub struct Shr3 {
    shr3: u32,
}

pub struct Cong {
    cong: u32,
}

impl Shr3 {
    pub fn new() -> Shr3 {
        Shr3 {
            shr3: 1,
        }
    }
    pub fn sanitise(&mut self) {
        if self.shr3 == 0 {
            self.shr3 = 1;
        }
    }
    pub fn next(&mut self) -> u32 {
        self.sanitise();
        let mut shr3 = self.shr3;
    
        shr3 ^= (shr3 << 13);
        shr3 ^= (shr3 >> 17);
        shr3 ^= (shr3 << 5);
        self.shr3 = shr3;
        
        shr3
    }
}