// i don't want a whole rand crate every time i need a random number :(
pub struct XORShiftRNG { pub state: u64 }
impl XORShiftRNG {
    pub fn new() -> Self {
        Self {
            state: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos() as u64
        }
    }

    pub fn next(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }
}

pub fn shuffle<T>(array: &mut [T], xsr: &mut XORShiftRNG) {
    for i in (0..array.len()).rev() {
        let j = (xsr.next() as usize) % (i + 1);
        array.swap(i, j);
    }
}

pub fn rand(max: &usize, xsr: &mut XORShiftRNG) -> usize {
    let cap = usize::MAX - (usize::MAX % *max as usize);

    loop { // it'll find a number... eventually...
        let num = xsr.next() as usize;
        if num <= cap { return num % *max }
    }
}
