//FIXME (mmilburn): I'm pretty sure I want usize for len.
pub fn verify(a: &Box<[u8]>, b: &Box<[u8]>, len: usize) -> bool
{
    assert!(a.len() == b.len());
    //FIXME (mmilburn): I don't know why the reference implementation uses a 
    //64-bit unsigned quantity here.
    let mut r: u64 = 0;

    for i in 0..len
    {
        r |= (a[i] ^ b[i]) as u64;
    }

    r = r.wrapping_neg() >> 63;
    //YES THIS LOGIC IS INVERTED! UGH!
    return r != 0;
}

//Ugh, impurity.
pub fn cmov(r: &mut Box<[u8]>, x: &[u8], len: usize, b: bool)
{
    assert!(len <= r.len());
    //Original code has b as an unsigned char, then did this:
    //b = -b;
    //So set the 8-bit b's two's complement (btc).
    let btc: u8 = if b { 255 } else { 0 };

    for i in 0..len
    {
        r[i] ^= btc & (x[i] ^ r[i]);
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_simple_verify_match()
    {
        let a: Box<[u8]> = Box::new([1, 2, 3]);
        let b: Box<[u8]> = Box::new([1, 2, 3]);
        let c: Box<[u8]> = Box::new([1, 2, 4]);
        assert_eq!(!verify(&a, &b, 3), true, "Arrays a and b match");
        assert_eq!(!verify(&a, &c, 3), false, "Arrays a and c don't match");
    }



    #[test]
    fn prove_simple_verify()
    {
        let zero: Box<[u8]> = Box::new([0]);

        for i in 1..=255
        {
            let index: Box<[u8]> = Box::new([i]);
            assert_eq!(!verify(&zero, &index, index.len()), false, 
                       "Single element array: [{}] does not match [{}]", 
                       zero[0], i);
        }
        //You've got bigger problems if this fails.
        assert_eq!(!verify(&zero, &zero, zero.len()), true, 
                   "Single element array: [{}] does match [{}]", zero[0], 
                   zero[0]);
    }

    #[test]
    fn test_small_cmov()
    {
        let a: Box<[u8]> = Box::new([1, 2, 3]);
        let mut b: Box<[u8]> = Box::new([1, 2, 3]);
        //n.b. the values will XOR to 0 with the values in b.
        let c: Box<[u8]> = Box::new([3, 5, 4]);
        
        cmov(&mut b, &c, 3, false);
        assert_eq!(!verify(&a, &b, a.len()), true, 
                   "b unchanged - a: {:?} b: {:?}", a, b);
        cmov(&mut b, &c, 3, true);
        assert_eq!(!verify(&a, &b, a.len()), false, 
                   "b changed - a: {:?} b: {:?}", a, b);
        assert_eq!(!verify(&b, &c, b.len()), true, 
                   "b values == c values - b: {:?} c: {:?}", b, c);
    }
}
