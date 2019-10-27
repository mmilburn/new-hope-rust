const QINV: u32 = 12287; // -inverse_mod(p,2^18)
const RLOG: u32 = 18;
//const RINV: u32 = 576;
const NEWHOPE_Q: u32 = 12289;
/*
 *"for example, for the input 2^32−q(R−1) = 1073491969 the addition
 a = a + u causes an overflow and the function returns 0 instead
 of the correct result 4095."
 "the input to the Montgomeryreduction, is at most69634·(q−1)  =  855662592and thus safely below the maximum input that theMontgomery reduction can handle."

  Helpful links:
  http://www.people.vcu.edu/~jwang3/CMSC691/j34monex.pdf
  https://web.archive.org/web/20171215100101/http://www.people.vcu.edu/~jwang3/CMSC691/j34monex.pdf
 * */
pub fn montgomery_reduce(mut a: u32) -> u16
{
  if !cfg!(test)
  {
    assert!(a < 855662592);
  }
  let mut u: u32 = a.wrapping_mul(QINV);
  u &= (1 << RLOG) - 1;
  u *= NEWHOPE_Q;
  a = a.wrapping_add(u);
  //This was represented as a >> 18 in the reference implemetation. Is it
  //merely coincidental that RLOG is 18 as well? Probably not the original
  //listing in the paper had 18 in place of the RLOG in the reference
  //implementation.
  return (a >> RLOG) as u16;
}


#[cfg(test)]
mod tests
{
 use super::*;

 #[test]
 fn test_overflow_to_zero()
 {
   assert_eq!(montgomery_reduce(1073491969), 0);
 }

 /**
  * Wrote some junk octave to generate
  * a few expected values.
  * x = randi(100000, 1, 10);
  * q = 12289;
  * qinv = 12287;
  * rlog = 18;
  * r = 2**rlog;
  * rinv = 576;
  * disp(x);
  *
  * for t = x
  *   m = mod((t * qinv), r);
  *   u = (t + m * q)/r;
  *   printf ("t = %d u = %d\n", t, u);
  * endfor
  *
  * */
 #[test]
 fn check_a_few_inputs()
 {
    let mut expected = Vec::new();
    expected.push((83360, 2237));
    expected.push((72406, 9279));
    expected.push((21359, 1495));
    expected.push((13632, 11650));
    expected.push((8681, 10922));
    expected.push((76624, 5625));
    expected.push((92278, 2203));
    expected.push((43432, 8717));
    expected.push((65816, 10740));
    expected.push((77096, 7139));
    
    for tuple in expected
    {
        assert_eq!(montgomery_reduce(tuple.0), tuple.1);
    }

 }
}
