static NROUNDS: usize = 24;

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
    assert!(offset + 8 <= x.len());
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


fn KeccakF1600_StatePermute(mut state: [u64; 24])
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

fn keccak_absorb(mut state: [u64; 24], rate: usize, m: Vec<u8>, p: u8)
{
    assert!((rate / 8) < state.len());
    for i in 0..state.len()
    {
        state[i] = 0;
    }

    while m.len() >= rate
    {
        let mut i = 0;
        while i < rate / 8
        {
            //state[i] ^= load64(m, 8 * i);
        }

        KeccakF1600_StatePermute(state);
    }
}

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
}
