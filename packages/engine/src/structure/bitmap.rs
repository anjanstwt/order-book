use crate::types::{DEFAULT_CAPACITY_TICKS, Tick};

pub struct BitMap {
    l0: Vec<u64>,
    l1: Vec<u64>,
    l2: Vec<u64>,
}

impl BitMap {
    pub fn new(capacity_ticks: Option<u64>) -> Self {
        let ticks = capacity_ticks.unwrap_or(DEFAULT_CAPACITY_TICKS);

        /*
         * the logic behind adding 63
         * e.g.
         * (remember 1 word = 64 bits here)
         * if the capacity is 100 then dividing it by 64 will give 1 i.e. 1 word is required
         * and it can only fill from 0-63, but we needed till 100
         * so adding 63 will make the 100 -> 163
         * now dividing will give 2 which will contain all values from 0-100
         *
         * */

        let l0_words = ((ticks + 63) / 64) as usize;
        let l1_words = ((l0_words + 63) / 64) as usize;
        let l2_words = ((l1_words + 63) / 64) as usize;

        Self {
            l0: vec![0u64; l0_words],
            l1: vec![0u64; l1_words],
            l2: vec![0u64; l2_words],
        }
    }

    pub fn set(&mut self, tick: Tick) {
        debug_assert!(
            (tick / 64) < self.l0.len() as u64,
            "tick {tick} out of range",
        );

        // setting the l0 layer
        let l0_word_idx = (tick / 64) as usize;
        let l0_bit_idx = tick % 64;
        /*
         * this is the most critical part
         * the generalised form of this is,
         * w |= 1u64 << b;
         * w = word, b = bit, 1u64 = 1 bit is non zero else zero
         * so what this does
         * 1u64 has Left most bit 1, << (b) pushes it left side b times
         * now |= takes bitwise OR between prev changed word w and new pushing bit b, i.e. it is
         * equivalent to w = w | b
         * that is if the word was 000100 and new bit is 000010
         * then their or will be 000110
         *
         * */
        self.l0[l0_word_idx] |= 1u64 << l0_bit_idx;

        // setting the l1 layer
        let l1_word_idx = (l0_word_idx / 64) as usize;
        let l1_bit_idx = l0_word_idx % 64;
        self.l1[l1_word_idx] |= 1u64 << l1_bit_idx;

        // setting the l2 layer
        let l2_word_idx = (l1_word_idx / 64) as usize;
        let l2_bit_idx = l1_word_idx % 64;
        self.l2[l2_word_idx] |= 1u64 << l2_bit_idx;
    }

    pub fn clear(&mut self, tick: Tick) {
        // clearing the l0 layer
        let l0_word_idx = (tick / 64) as usize;
        let l0_bit_idx = tick % 64;

        debug_assert!(
            (self.l0[l0_word_idx] >> l0_bit_idx) & 1 == 1,
            "tick {tick} was not set",
        );

        /*
         * this will work as,
         * let's say w was 000110 and b is 000010
         * we will revert the b first -> 111101
         * their bitwise AND will be 000100
         *
         * */
        self.l0[l0_word_idx] &= !(1u64 << l0_bit_idx);

        // check if the every bits are zero in this word or not
        // if any single bit is not 0, then no need to change the upper layers
        if self.l0[l0_word_idx] != 0 {
            return;
        }

        // clearing the l1 layer
        let l1_word_idx = (l0_word_idx / 64) as usize;
        let l1_bit_idx = l0_word_idx % 64;
        self.l1[l1_word_idx] &= !(1u64 << l1_bit_idx);

        if self.l1[l1_word_idx] != 0 {
            return;
        }

        // clearing the l2 layer
        let l2_word_idx = (l1_word_idx / 64) as usize;
        let l2_bit_idx = l1_word_idx % 64;
        self.l2[l2_word_idx] &= !(1u64 << l2_bit_idx);
    }

    // this will be used to find the next lowest price
    // i.e. for ASK side
    pub fn first_lowest_tick(&self) -> Option<Tick> {
        for (l2_word_idx, &l2_word) in self.l2.iter().enumerate() {
            // if this is true, means the word is empty and we can skip that word
            if l2_word == 0 {
                continue;
            }

            let l1_bit = l2_word.trailing_zeros() as usize;
            let l1_word_idx = (l2_word_idx * 64) + l1_bit;
            let l1_word = self.l1[l1_word_idx];

            let l0_bit = l1_word.trailing_zeros() as usize;
            let l0_word_idx = (l1_word_idx * 64) + l0_bit;
            let l0_word = self.l0[l0_word_idx];

            let tick_bit = l0_word.trailing_zeros() as u64;
            let tick = ((l0_word_idx as u64) * 64) + tick_bit;
            return Some(tick);
        }

        None
    }

    // this will be used to find the next highest price
    // i.e. for BID side
    pub fn first_highest_tick(&self) -> Option<Tick> {
        for (l2_word_idx, &l2_word) in self.l2.iter().enumerate().rev() {
            if l2_word == 0 {
                continue;
            }

            let l1_bit = 63 - l2_word.leading_zeros() as usize;
            let l1_word_idx = (l2_word_idx * 64) + l1_bit;
            let l1_word = self.l1[l1_word_idx];

            let l0_bit = 63 - l1_word.leading_zeros() as usize;
            let l0_word_idx = (l1_word_idx * 64) + l0_bit;
            let l0_word = self.l0[l0_word_idx];

            let tick_bit = 63 - l0_word.leading_zeros() as u64;
            let tick = ((l0_word_idx as u64) * 64) + tick_bit;
            return Some(tick);
        }
        None
    }
}
