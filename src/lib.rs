pub fn straightforward_sum(buffer_a: &[u8], buffer_b: &[u8]) -> i32 {
    buffer_a
        .iter()
        .zip(buffer_b)
        .map(|(&a, &b)| (a as i32 - b as i32).pow(2))
        .sum()
}

pub fn regular_loop(buffer_a: &[u8], buffer_b: &[u8]) -> i32 {
    let chunks_iter_a = buffer_a.chunks_exact(3);
    let chunks_iter_b = buffer_b.chunks_exact(3);

    let sum_remainder: i32 = chunks_iter_a
        .remainder()
        .iter()
        .zip(chunks_iter_b.remainder())
        .map(|(&a, &b)| (a as i32 - b as i32).pow(2))
        .sum();
    let sum: i32 = chunks_iter_a
        .zip(chunks_iter_b)
        .map(|(chunks_a, chunks_b)| {
            (chunks_a[0] as i32 - chunks_b[0] as i32).pow(2)
                + (chunks_a[1] as i32 - chunks_b[1] as i32).pow(2)
                + (chunks_a[2] as i32 - chunks_b[2] as i32).pow(2)
        })
        .sum();

    sum + sum_remainder
}

pub fn four_at_a_time(buffer_a: &[u8], buffer_b: &[u8]) -> i32 {
    let chunks_iter_a = buffer_a.chunks_exact(4);
    let chunks_iter_b = buffer_b.chunks_exact(4);

    let sum_remainder: i32 = chunks_iter_a
        .remainder()
        .iter()
        .zip(chunks_iter_b.remainder())
        .map(|(&a, &b)| (a as i32 - b as i32).pow(2))
        .sum();
    let sum: i32 = chunks_iter_a
        .zip(chunks_iter_b)
        .map(|(chunks_a, chunks_b)| {
            (chunks_a[0] as i32 - chunks_b[0] as i32).pow(2)
                + (chunks_a[1] as i32 - chunks_b[1] as i32).pow(2)
                + (chunks_a[2] as i32 - chunks_b[2] as i32).pow(2)
                + (chunks_a[3] as i32 - chunks_b[3] as i32).pow(2)
        })
        .sum();

    sum + sum_remainder
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx")]
pub unsafe fn avx256(buffer_a: &[u8], buffer_b: &[u8]) -> i32 {
    use core::arch::x86_64::*;

    let chunks_iter_a = buffer_a.chunks_exact(16);
    let chunks_iter_b = buffer_b.chunks_exact(16);

    let sum_remainder: i32 = chunks_iter_a
        .remainder()
        .iter()
        .zip(chunks_iter_b.remainder())
        .map(|(&a, &b)| (a as i32 - b as i32).pow(2))
        .sum();
    let sum: i32 = chunks_iter_a
        .zip(chunks_iter_b)
        .map(|(chunks_a, chunks_b)| {
            let va = _mm256_set_epi16(
                chunks_a[0] as i16,
                chunks_a[1] as i16,
                chunks_a[2] as i16,
                chunks_a[3] as i16,
                chunks_a[4] as i16,
                chunks_a[5] as i16,
                chunks_a[6] as i16,
                chunks_a[7] as i16,
                chunks_a[8] as i16,
                chunks_a[9] as i16,
                chunks_a[10] as i16,
                chunks_a[11] as i16,
                chunks_a[12] as i16,
                chunks_a[13] as i16,
                chunks_a[14] as i16,
                chunks_a[15] as i16,
            );
            let vb = _mm256_set_epi16(
                chunks_b[0] as i16,
                chunks_b[1] as i16,
                chunks_b[2] as i16,
                chunks_b[3] as i16,
                chunks_b[4] as i16,
                chunks_b[5] as i16,
                chunks_b[6] as i16,
                chunks_b[7] as i16,
                chunks_b[8] as i16,
                chunks_b[9] as i16,
                chunks_b[10] as i16,
                chunks_b[11] as i16,
                chunks_b[12] as i16,
                chunks_b[13] as i16,
                chunks_b[14] as i16,
                chunks_b[15] as i16,
            );
            let diff = _mm256_sub_epi16(va, vb);
            let sqr: [i32; 8] = std::mem::transmute(_mm256_madd_epi16(diff, diff));
            sqr.iter().sum::<i32>()
        })
        .sum();

    sum + sum_remainder
}

pub fn sum_using_iterator(buffer_a: &[u8], buffer_b: &[u8]) -> i32 {
    buffer_a
        .chunks(3)
        .zip(buffer_b.chunks(3))
        .map(|(chunks_a, chunks_b)| {
            chunks_a
                .iter()
                .zip(chunks_b)
                .map(|(&a, &b)| (a as i32 - b as i32).pow(2))
                .sum::<i32>()
        })
        .sum()
}

pub fn dummy_input() -> Vec<u8> {
    const N: usize = 10_000;
    let mut vec = vec![0; N];
    for (i, val) in vec.iter_mut().enumerate() {
        *val = (i % (u8::MAX as usize)) as u8;
    }
    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity_check() {
        let mut buffer_a = dummy_input();
        buffer_a.reverse();
        let buffer_b = dummy_input();

        let correct_sum = straightforward_sum(&buffer_a, &buffer_b);
        assert_eq!(regular_loop(&buffer_a, &buffer_b), correct_sum);
        assert_eq!(four_at_a_time(&buffer_a, &buffer_b), correct_sum);
        #[cfg(target_arch = "x86_64")]
        unsafe {
            assert_eq!(avx256(&buffer_a, &buffer_b), correct_sum);
        }
        assert_eq!(sum_using_iterator(&buffer_a, &buffer_b), correct_sum);
    }
}
