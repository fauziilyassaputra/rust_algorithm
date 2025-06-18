pub struct PCG32 {
    state: u64,
    increment: u64,
}

impl PCG32 {
    pub fn new_default(seed: u64) -> Self {
        PCG32 {
            state: seed,
            increment: 1442695040888963407,
        }
    }

    pub fn get_u64(&mut self) -> u64 {
        self.next() as u64
    }

    pub fn get_u32(&mut self) -> u32 {
        self.next()
    }

    fn next(&mut self) -> u32 {
        let oldstate = self.state;
        self.state = oldstate.wrapping_mul(6364136223846793005).wrapping_add(self.increment);
        let xorshifted = (((oldstate >> 18) ^ oldstate) >> 27) as u32;
        let rot = (oldstate >> 59) as u32;
        xorshifted.rotate_right(rot)
    }
}

fn main() {}
