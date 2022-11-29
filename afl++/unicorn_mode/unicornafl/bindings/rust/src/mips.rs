#![allow(non_camel_case_types)]

// MIPS registers
#[repr(C)]
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum RegisterMIPS {
    INVALID = 0,

    // General purpose registers
    PC = 1,
    ZERO = 2,
    AT = 3,
    V0 = 4,
    V1 = 5,
    A0 = 6,
    A1 = 7,
    A2 = 8,
    A3 = 9,
    T0 = 10,
    T1 = 11,
    T2 = 12,
    T3 = 13,
    T4 = 14,
    T5 = 15,
    T6 = 16,
    T7 = 17,
    S0 = 18,
    S1 = 19,
    S2 = 20,
    S3 = 21,
    S4 = 22,
    S5 = 23,
    S6 = 24,
    S7 = 25,
    T8 = 26,
    T9 = 27,
    K0 = 28,
    K1 = 29,
    GP = 30,
    SP = 31,
    FP = 32,
    RA = 33,

    // DSP registers
    DSPCCOND = 34,
    DSPCARRY = 35,
    DSPEFI = 36,
    DSPOUTFLAG = 37,
    DSPOUTFLAG16_19 = 38,
    DSPOUTFLAG20 = 39,
    DSPOUTFLAG21 = 40,
    DSPOUTFLAG22 = 41,
    DSPOUTFLAG23 = 42,
    DSPPOS = 43,
    DSPSCOUNT = 44,

    // ACC registers
    AC0 = 45,
    AC1 = 46,
    AC2 = 47,
    AC3 = 48,

    // COP registers
    CC0 = 49,
    CC1 = 50,
    CC2 = 51,
    CC3 = 52,
    CC4 = 53,
    CC5 = 54,
    CC6 = 55,
    CC7 = 56,

    // FPU registers
    F0 = 57,
    F1 = 58,
    F2 = 59,
    F3 = 60,
    F4 = 61,
    F5 = 62,
    F6 = 63,
    F7 = 64,
    F8 = 65,
    F9 = 66,
    F10 = 67,
    F11 = 68,
    F12 = 69,
    F13 = 70,
    F14 = 71,
    F15 = 72,
    F16 = 73,
    F17 = 74,
    F18 = 75,
    F19 = 76,
    F20 = 77,
    F21 = 78,
    F22 = 79,
    F23 = 80,
    F24 = 81,
    F25 = 82,
    F26 = 83,
    F27 = 84,
    F28 = 85,
    F29 = 86,
    F30 = 87,
    F31 = 88,
    FCC0 = 89,
    FCC1 = 90,
    FCC2 = 91,
    FCC3 = 92,
    FCC4 = 93,
    FCC5 = 94,
    FCC6 = 95,
    FCC7 = 96,

    // AFPR128
    W0 = 97,
    W1 = 98,
    W2 = 99,
    W3 = 100,
    W4 = 101,
    W5 = 102,
    W6 = 103,
    W7 = 104,
    W8 = 105,
    W9 = 106,
    W10 = 107,
    W11 = 108,
    W12 = 109,
    W13 = 110,
    W14 = 111,
    W15 = 112,
    W16 = 113,
    W17 = 114,
    W18 = 115,
    W19 = 116,
    W20 = 117,
    W21 = 118,
    W22 = 119,
    W23 = 120,
    W24 = 121,
    W25 = 122,
    W26 = 123,
    W27 = 124,
    W28 = 125,
    W29 = 126,
    W30 = 127,
    W31 = 128,
    HI = 129,
    LO = 130,
    P0 = 131,
    P1 = 132,
    P2 = 133,
    MPL0 = 134,
    MPL1 = 135,
    MPL2 = 136,
}