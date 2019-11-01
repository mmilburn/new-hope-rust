static NROUNDS: usize = 24;
static SHAKE128_RATE: usize = 168;

/* Keccak round constants */
static KeccakF_RoundConstants: [u64; 24] = 
[
    0x0000000000000001,
    0x0000000000008082,
    0x800000000000808a,
    0x8000000080008000,
    0x000000000000808b,
    0x0000000080000001,
    0x8000000080008081,
    0x8000000000008009,
    0x000000000000008a,
    0x0000000000000088,
    0x0000000080008009,
    0x000000008000000a,
    0x000000008000808b,
    0x800000000000008b,
    0x8000000000008089,
    0x8000000000008003,
    0x8000000000008002,
    0x8000000000000080,
    0x000000000000800a,
    0x800000008000000a,
    0x8000000080008081,
    0x8000000000008080,
    0x0000000080000001,
    0x8000000080008008
];

macro_rules! ROL
{
    ($a: ident, $offset: literal) =>
    {
        (($a << $offset)) ^ ($a >> (64 - $offset))
    };
}

fn load64(x: &Vec<u8>, offset: usize) -> u64
{
    let mut r: u64 = 0;
    for i in offset..offset + 8
    {
        let value: u64 = x[i] as u64;
        let shift: u32 = 8 * i as u32;
        r |= value.wrapping_shl(shift);
    }

    return r;
}

fn store64(u: u64) -> Vec<u8>
{
    let mut value = u;
    let mut y: Vec<u8> = Vec::new();
    while y.len() < 8
    {
        let byte: u8 = (0x0000_0000_0000_00FF & value) as u8;
        y.push(byte);
        value = value >> 8;
    }
    return y;
}


fn KeccakF1600_StatePermute(state: &mut Vec<u64>)
{
    
    let (mut Aba, mut Abe, mut Abi, mut Abo, mut Abu): 
        (u64, u64, u64, u64, u64);
    let (mut Aga, mut Age, mut Agi, mut Ago, mut Agu): 
        (u64, u64, u64, u64, u64);
    let (mut Aka, mut Ake, mut Aki, mut Ako, mut Aku): 
        (u64, u64, u64, u64, u64);  
    let (mut Ama, mut Ame, mut Ami, mut Amo, mut Amu): 
        (u64, u64, u64, u64, u64);
    let (mut Asa, mut Ase, mut Asi, mut Aso, mut Asu): 
        (u64, u64, u64, u64, u64);
    let (mut BCa, mut BCe, mut BCi, mut BCo, mut BCu): 
        (u64, u64, u64, u64, u64);
    let (mut Da, mut De, mut Di, mut Do, mut Du):      
        (u64, u64, u64, u64, u64);
    let (mut Eba, mut Ebe, mut Ebi, mut Ebo, mut Ebu): 
        (u64, u64, u64, u64, u64);
    let (mut Ega, mut Ege, mut Egi, mut Ego, mut Egu): 
        (u64, u64, u64, u64, u64);
    let (mut Eka, mut Eke, mut Eki, mut Eko, mut Eku): 
        (u64, u64, u64, u64, u64);
    let (mut Ema, mut Eme, mut Emi, mut Emo, mut Emu): 
        (u64, u64, u64, u64, u64);
    let (mut Esa, mut Ese, mut Esi, mut Eso, mut Esu): 
        (u64, u64, u64, u64, u64);


        //copyFromState(A, state)
        Aba = state[ 0];
        Abe = state[ 1];
        Abi = state[ 2];
        Abo = state[ 3];
        Abu = state[ 4];
        Aga = state[ 5];
        Age = state[ 6];
        Agi = state[ 7];
        Ago = state[ 8];
        Agu = state[ 9];
        Aka = state[10];
        Ake = state[11];
        Aki = state[12];
        Ako = state[13];
        Aku = state[14];
        Ama = state[15];
        Ame = state[16];
        Ami = state[17];
        Amo = state[18];
        Amu = state[19];
        Asa = state[20];
        Ase = state[21];
        Asi = state[22];
        Aso = state[23];
        Asu = state[24];

        for round in (0..NROUNDS).step_by(2)
        {
            //    prepareTheta
            BCa = Aba^Aga^Aka^Ama^Asa;
            BCe = Abe^Age^Ake^Ame^Ase;
            BCi = Abi^Agi^Aki^Ami^Asi;
            BCo = Abo^Ago^Ako^Amo^Aso;
            BCu = Abu^Agu^Aku^Amu^Asu;

            //thetaRhoPiChiIotaPrepareTheta(round  , A, E)
            Da = BCu^ROL!(BCe, 1);
            De = BCa^ROL!(BCi, 1);
            Di = BCe^ROL!(BCo, 1);
            Do = BCi^ROL!(BCu, 1);
            Du = BCo^ROL!(BCa, 1);

            Aba ^= Da;
            BCa = Aba;
            Age ^= De;
            BCe = ROL!(Age, 44);
            Aki ^= Di;
            BCi = ROL!(Aki, 43);
            Amo ^= Do;
            BCo = ROL!(Amo, 21);
            Asu ^= Du;
            BCu = ROL!(Asu, 14);
            Eba =   BCa ^((!BCe)&  BCi );
            Eba ^= KeccakF_RoundConstants[round];
            Ebe =   BCe ^((!BCi)&  BCo );
            Ebi =   BCi ^((!BCo)&  BCu );
            Ebo =   BCo ^((!BCu)&  BCa );
            Ebu =   BCu ^((!BCa)&  BCe );

            Abo ^= Do;
            BCa = ROL!(Abo, 28);
            Agu ^= Du;
            BCe = ROL!(Agu, 20);
            Aka ^= Da;
            BCi = ROL!(Aka,  3);
            Ame ^= De;
            BCo = ROL!(Ame, 45);
            Asi ^= Di;
            BCu = ROL!(Asi, 61);
            Ega =   BCa ^((!BCe)&  BCi );
            Ege =   BCe ^((!BCi)&  BCo );
            Egi =   BCi ^((!BCo)&  BCu );
            Ego =   BCo ^((!BCu)&  BCa );
            Egu =   BCu ^((!BCa)&  BCe );

            Abe ^= De;
            BCa = ROL!(Abe,  1);
            Agi ^= Di;
            BCe = ROL!(Agi,  6);
            Ako ^= Do;
            BCi = ROL!(Ako, 25);
            Amu ^= Du;
            BCo = ROL!(Amu,  8);
            Asa ^= Da;
            BCu = ROL!(Asa, 18);
            Eka =   BCa ^((!BCe)&  BCi );
            Eke =   BCe ^((!BCi)&  BCo );
            Eki =   BCi ^((!BCo)&  BCu );
            Eko =   BCo ^((!BCu)&  BCa );
            Eku =   BCu ^((!BCa)&  BCe );

            Abu ^= Du;
            BCa = ROL!(Abu, 27);
            Aga ^= Da;
            BCe = ROL!(Aga, 36);
            Ake ^= De;
            BCi = ROL!(Ake, 10);
            Ami ^= Di;
            BCo = ROL!(Ami, 15);
            Aso ^= Do;
            BCu = ROL!(Aso, 56);
            Ema =   BCa ^((!BCe)&  BCi );
            Eme =   BCe ^((!BCi)&  BCo );
            Emi =   BCi ^((!BCo)&  BCu );
            Emo =   BCo ^((!BCu)&  BCa );
            Emu =   BCu ^((!BCa)&  BCe );

            Abi ^= Di;
            BCa = ROL!(Abi, 62);
            Ago ^= Do;
            BCe = ROL!(Ago, 55);
            Aku ^= Du;
            BCi = ROL!(Aku, 39);
            Ama ^= Da;
            BCo = ROL!(Ama, 41);
            Ase ^= De;
            BCu = ROL!(Ase,  2);
            Esa =   BCa ^((!BCe)&  BCi );
            Ese =   BCe ^((!BCi)&  BCo );
            Esi =   BCi ^((!BCo)&  BCu );
            Eso =   BCo ^((!BCu)&  BCa );
            Esu =   BCu ^((!BCa)&  BCe );

            //    prepareTheta
            BCa = Eba^Ega^Eka^Ema^Esa;
            BCe = Ebe^Ege^Eke^Eme^Ese;
            BCi = Ebi^Egi^Eki^Emi^Esi;
            BCo = Ebo^Ego^Eko^Emo^Eso;
            BCu = Ebu^Egu^Eku^Emu^Esu;

            //thetaRhoPiChiIotaPrepareTheta(round+1, E, A)
            Da = BCu^ROL!(BCe, 1);
            De = BCa^ROL!(BCi, 1);
            Di = BCe^ROL!(BCo, 1);
            Do = BCi^ROL!(BCu, 1);
            Du = BCo^ROL!(BCa, 1);

            Eba ^= Da;
            BCa = Eba;
            Ege ^= De;
            BCe = ROL!(Ege, 44);
            Eki ^= Di;
            BCi = ROL!(Eki, 43);
            Emo ^= Do;
            BCo = ROL!(Emo, 21);
            Esu ^= Du;
            BCu = ROL!(Esu, 14);
            Aba =   BCa ^((!BCe)&  BCi );
            Aba ^= KeccakF_RoundConstants[round+1];
            Abe =   BCe ^((!BCi)&  BCo );
            Abi =   BCi ^((!BCo)&  BCu );
            Abo =   BCo ^((!BCu)&  BCa );
            Abu =   BCu ^((!BCa)&  BCe );

            Ebo ^= Do;
            BCa = ROL!(Ebo, 28);
            Egu ^= Du;
            BCe = ROL!(Egu, 20);
            Eka ^= Da;
            BCi = ROL!(Eka, 3);
            Eme ^= De;
            BCo = ROL!(Eme, 45);
            Esi ^= Di;
            BCu = ROL!(Esi, 61);
            Aga =   BCa ^((!BCe)&  BCi );
            Age =   BCe ^((!BCi)&  BCo );
            Agi =   BCi ^((!BCo)&  BCu );
            Ago =   BCo ^((!BCu)&  BCa );
            Agu =   BCu ^((!BCa)&  BCe );

            Ebe ^= De;
            BCa = ROL!(Ebe, 1);
            Egi ^= Di;
            BCe = ROL!(Egi, 6);
            Eko ^= Do;
            BCi = ROL!(Eko, 25);
            Emu ^= Du;
            BCo = ROL!(Emu, 8);
            Esa ^= Da;
            BCu = ROL!(Esa, 18);
            Aka =   BCa ^((!BCe)&  BCi );
            Ake =   BCe ^((!BCi)&  BCo );
            Aki =   BCi ^((!BCo)&  BCu );
            Ako =   BCo ^((!BCu)&  BCa );
            Aku =   BCu ^((!BCa)&  BCe );

            Ebu ^= Du;
            BCa = ROL!(Ebu, 27);
            Ega ^= Da;
            BCe = ROL!(Ega, 36);
            Eke ^= De;
            BCi = ROL!(Eke, 10);
            Emi ^= Di;
            BCo = ROL!(Emi, 15);
            Eso ^= Do;
            BCu = ROL!(Eso, 56);
            Ama =   BCa ^((!BCe)&  BCi );
            Ame =   BCe ^((!BCi)&  BCo );
            Ami =   BCi ^((!BCo)&  BCu );
            Amo =   BCo ^((!BCu)&  BCa );
            Amu =   BCu ^((!BCa)&  BCe );

            Ebi ^= Di;
            BCa = ROL!(Ebi, 62);
            Ego ^= Do;
            BCe = ROL!(Ego, 55);
            Eku ^= Du;
            BCi = ROL!(Eku, 39);
            Ema ^= Da;
            BCo = ROL!(Ema, 41);
            Ese ^= De;
            BCu = ROL!(Ese, 2);
            Asa =   BCa ^((!BCe)&  BCi );
            Ase =   BCe ^((!BCi)&  BCo );
            Asi =   BCi ^((!BCo)&  BCu );
            Aso =   BCo ^((!BCu)&  BCa );
            Asu =   BCu ^((!BCa)&  BCe );
        }

        //copyToState(state, A)
        state[ 0] = Aba;
        state[ 1] = Abe;
        state[ 2] = Abi;
        state[ 3] = Abo;
        state[ 4] = Abu;
        state[ 5] = Aga;
        state[ 6] = Age;
        state[ 7] = Agi;
        state[ 8] = Ago;
        state[ 9] = Agu;
        state[10] = Aka;
        state[11] = Ake;
        state[12] = Aki;
        state[13] = Ako;
        state[14] = Aku;
        state[15] = Ama;
        state[16] = Ame;
        state[17] = Ami;
        state[18] = Amo;
        state[19] = Amu;
        state[20] = Asa;
        state[21] = Ase;
        state[22] = Asi;
        state[23] = Aso;
        state[24] = Asu;

}

//This CANNOT handle messages that aren't byte divisble.
fn keccak_absorb(rate: usize, m: Vec<u8>, p: u8) -> Vec<u64>
{
    let rate_qwords = rate / 8;
    let mut state: Vec<u64> = vec![0; 25];

    let mut t: Vec<u8> = vec![0u8; rate];
    let mut rate_offset = 0;

    while (m.len() - rate_offset) >= rate
    {
        for i in 0..rate_qwords
        {
            state[i] ^= load64(&m, rate_offset + 8 * i);
        }

        KeccakF1600_StatePermute(&mut state);
        rate_offset += rate;
    }

    //this is so side-effecty and weird!
    //remember rate_offset has been updated.
    let mpart = m.len() - rate_offset;

    
    for i in 0..(mpart)
    {
        t[i] = m[rate_offset + i];
    }

    t[mpart] = p;
    t[rate - 1] |= 128;
    for i in 0..rate_qwords
    {
        state[i] ^= load64(&t, 8 * i);
    }

    return state;
}

fn keccak_squeezeblocks()
{}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_load_store64()
    {
        let x: Vec<u8> = vec![ 0x1B, 0x1A, 0x0F, 0x0E, 0x0D, 0x0C, 0x0B, 0x0A ];
        let loaded = load64(&x, 0);
        let expected = 0x0A0B0C0D0E0F1A1B;
        assert_eq!(loaded == expected, true, "expected {} found {}", expected,
                   loaded);

        let y = store64(expected);
        for i in 0..y.len()
        {
            assert_eq!(x[i] == y[i], true, "x[{}] = {} y[{}] = {}", i, x[i], i, y[i]);
        }
    }

    #[test]
    //https://github.com/XKCP/XKCP/blob/master/tests/TestVectors/KeccakF-1600-IntermediateValues.txt
    fn test_keccakf1600_on_zeros()
    {
        let mut s: Vec<u64> = vec![0; 25];
        //I pulled these values in as big endian...
        let final_state: Vec<u64> = vec![
            0xE7DDE140798F25F1,
            0x8A47C033F9CCD584,
            0xEEA95AA61E2698D5,
            0x4D49806F304715BD,
            0x57D05362054E288B,
            0xD46F8E7F2DA497FF,
            0xC44746A4A0E5FE90,
            0x762E19D60CDA5B8C,
            0x9C05191BF7A630AD,
            0x64FC8FD0B75A9330,
            0x35D617233FA95AEB,
            0x0321710D26E6A6A9,
            0x5F55CFDB167CA581,
            0x26C84703CD31B843,
            0x9F56A5111A2FF201,
            0x61AED9215A63E505,
            0xF270C98CF2FEBE64,
            0x1166C47B95703661,
            0xCB0ED04F555A7CB8,
            0xC832CF1C8AE83E8C,
            0x14263AAE22790C94,
            0xE409C5A224F94118,
            0xC26504E72635F516,
            0x3BA1307FE944F675,
            0x49A2EC5C7BFFF1EA,
        ];

        KeccakF1600_StatePermute(&mut s);

        for i in 0..s.len()
        {
            let f = s[i];
            let e = u64::from_be(final_state[i]);
            assert_eq!(f == e, true, 
                       "s[{}] = 0x{:X?} final_state[{}] = 0x{:X?}", i, f, i, e);
        }
    }

    #[test]
    fn test_keccak_absorb()
    {
        let msg: Vec<u8> = vec![
            0x83,0xAF,0x34,0x27,0x9C,0xCB,0x54,0x30,0xFE,0xBE,0xC0,0x7A,0x81,
            0x95,0x0D,0x30,0xF4,0xB6,0x6F,0x48,0x48,0x26,0xAF,0xEE,0x74,0x56,
            0xF0,0x07,0x1A,0x51,0xE1,0xBB,0xC5,0x55,0x70,0xB5,0xCC,0x7E,0xC6,
            0xF9,0x30,0x9C,0x17,0xBF,0x5B,0xEF,0xDD,0x7C,0x6B,0xA6,0xE9,0x68,
            0xCF,0x21,0x8A,0x2B,0x34,0xBD,0x5C,0xF9,0x27,0xAB,0x84,0x6E,0x38,
            0xA4,0x0B,0xBD,0x81,0x75,0x9E,0x9E,0x33,0x38,0x10,0x16,0xA7,0x55,
            0xF6,0x99,0xDF,0x35,0xD6,0x60,0x00,0x7B,0x5E,0xAD,0xF2,0x92,0xFE,
            0xEF,0xB7,0x35,0x20,0x7E,0xBF,0x70,0xB5,0xBD,0x17,0x83,0x4F,0x7B,
            0xFA,0x0E,0x16,0xCB,0x21,0x9A,0xD4,0xAF,0x52,0x4A,0xB1,0xEA,0x37,
            0x33,0x4A,0xA6,0x64,0x35,0xE5,0xD3,0x97,0xFC,0x0A,0x06,0x5C,0x41,
            0x1E,0xBB,0xCE,0x32,0xC2,0x40,0xB9,0x04,0x76,0xD3,0x07,0xCE,0x80,
            0x2E,0xC8,0x2C,0x1C,0x49,0xBC,0x1B,0xEC,0x48,0xC0,0x67,0x5E,0xC2,
            0xA6,0xC6,0xF3,0xED,0x3E,0x5B,0x74,0x1D,0x13,0x43,0x70,0x95,0x70,
            0x7C,0x56,0x5E,0x10,0xD8,0xA2,0x0B,0x8C,0x20,0x46,0x8F,0xF9,0x51,
            0x4F,0xCF,0x31,0xB4,0x24,0x9C,0xD8,0x2D,0xCE,0xE5,0x8C,0x0A,0x2A,
            0xF5,0x38,0xB2,0x91,0xA8,0x7E,0x33,0x90,0xD7,0x37,0x19,0x1A,0x07,
            0x48,0x4A,0x5D,0x3F,0x3F,0xB8,0xC8,0xF1,0x5C,0xE0,0x56,0xE5,0xE5,
            0xF8,0xFE,0xBE,0x5E,0x1F,0xB5,0x9D,0x67,0x40,0x98,0x0A,0xA0,0x6C,
            0xA8,0xA0,0xC2,0x0F,0x57,0x12,0xB4,0xCD,0xE5,0xD0,0x32,0xE9,0x2A,
            0xB8,0x9F,0x0A,0xE1,
        ];

        let final_state: Vec<u64> = vec![
            0xB887BE43E92717F6,
            0x37585B6696523384,
            0x0317D5FAC9797379,
            0xFBD1644BE44E34DB,
            0xB8FC2E68D17D9FA2,
            0x05EAE4A18871B352,
            0x5A153D434EC9F9B4,
            0x39B45DB77B63741C,
            0xC92E42F05505F8C2,
            0x6BA14164621DF21B,
            0xC45CDDCCA58F06A1,
            0x61C7C14C09238913,
            0xA5D180AF726DE793,
            0x77EA5492BD2CDC10,
            0xF2C8694D6E7F0907,
            0x0A14F87252BD8EF5,
            0xCF0DA70A1DAD0DC1,
            0x06A6DCD327FC56CF,
            0x6B05EDAE2224A298,
            0x31DC7848C88D402A,
            0x6C632DE10DEA95A1,
            0x2CDC6EE2EFD71E8B,
            0xB4EAFA78A17D3F92,
            0x599916226ED706D3,
            0xDE924A3B2AC45E56,
        ];

        let state = keccak_absorb(SHAKE128_RATE, msg, 0x1F);
        
        for i in 0..state.len()
        {
            let f = state[i];
            let e = u64::from_be(final_state[i]);
            assert_eq!(f == e, true, 
                       "s[{}] = 0x{:X?} final_state[{}] = 0x{:X?}", i, f, i, e);
        }
    }

        
}
