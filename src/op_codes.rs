struct OpCode {
    op: u8,
    len: u8,
    cycles: u8,
    mode: String,
    flags: Vec<bool>,
    syntax: String,
}

impl OpCode {
    pub fn new(op: u8, len: u8, cycles: u8, mode: String, flags: Vec<bool>, syntax: String) -> OpCode {
        OpCode {
            op,
            len,
            cycles,
            mode,
            flags,
            syntax,
        }
    }
}

lazy_static! {
    static ref OP_CODES: HashMap<u8, OpCode> = {
        let op_codes = HashMap::new();
        op_codes.insert(0x61, OpCode::new(0x61, ));
        
    }
}

enum OpCode {
    // ADC SBC
    61,
    63,
    65,
    67,
    69,
    6D,
    6F,
    71,
    72,
    73,
    75,
    77,
    79,
    7D,
    7F,
    E1,
    E3,
    E5,
    E7,
    E9,
    ED,
    EF,
    F1,
    F2,
    F3,
    F5,
    F7,
    F9,
    FD,
    FF,

    // CMP CPX CPY
    C1,
    C3,
    C5,
    C7,
    C9,
    CD,
    CF,
    D1,
    D2,
    D3,
    D5,
    D7,
    D9,
    DD,
    DF,
    E0,
    E4,
    EC,
    C0,
    C4,
    CC,

    // DEC DEX DEY INC INX INY
    3A,
    C6,
    CE,
    D6,
    DE,
    CA,
    88,
    1A,
    E6,
    EE,
    F6,
    FE,
    E8,
    C8,

    // AND EOR ORA
    21,
    23,
    25,
    27,
    29,
    2D,
    2F,
    31,
    32,
    33,
    35,
    37,
    39,
    3D,
    3F,
    41,
    43,
    45,
    47,
    49,
    4D,
    4F,
    51,
    52,
    53,
    55,
    57,
    59,
    5D,
    5F,
    01,
    03,
    05,
    07,
    09,
    0D,
    0F,
    11,
    12,
    13,
    15,
    17,
    19,
    1D,
    1F,

    // BIT
    24,
    2C,
    34,
    3C,
    89,

    // TRB TSB
    14,
    1C,
    04,
    0C,

    // ASL LSR ROL ROR
    06,
    0A,
    0E,
    16,
    1E,
    46,
    4A,
    4E,
    56,
    5E,
    26,
    2A,
    2E,
    36,
    3E,
    66,
    6A,
    6E,
    76,
    7E,

    // BCC BCS BEQ BMI BNE BPL BRA BVC BVS 
    90,
    B0,
    F0,
    30,
    D0,
    10,
    80,
    50,
    70,

    // BRL
    82,

    // JMP JSL JSR
    4C,
    5C,
    6C,
    7C,
    DC,
    22,
    20,
    FC,

    // RTL RTS
    6B,
    60,

    // BRK COP
    00,
    02,

    // RTI
    40,

    // CLC, CLD, CLI, CLV, SEC, SED, SEI
    18,
    D8,
    58,
    B8,
    38,
    F8,
    78,

    // REP SEP
    C2,
    E2,

    // LDA LDX LDY STA STX STY STZ
    A1,
    A3,
    A5,
    A7,
    A9,
    AD,
    AF,
    B1,
    B2,
    B3,
    B5,
    B7,
    B9,
    BD,
    BF,
    A2,
    A6,
    AE,
    B6,
    BE,
    A0,
    A4,
    AC,
    B4,
    BC,
    81,
    83,
    85,
    87,
    8D,
    8F,
    91,
    92,
    93,
    95,
    97,
    99,
    9D,
    9F,
    86,
    8E,
    96,
    84,
    8C,
    94,
    64,
    74,
    9C,
    93,

    // MVN MVP
    54,
    44,

    // NOP WDM
    EA,
    42,
    
    // PEA PEI PER
    F4,
    D4,
    62,

    // PHA PHX PHY PLA PLX PLY
    48,
    DA,
    5A,
    68,
    FA,
    7A,

    // PHB PHD PHK PHP PLB PLD PLP
    8B,
    0B,
    4B,
    08,
    AB,
    2B,
    28,

    // STP WAI
    DB,
    CB,

    // TAX TAY TSX TXA TXS TXY TYA TYX
    AA,
    A8,
    BA,
    8A,
    9A,
    9B,
    98,
    BB,

    // TCD TCS TDC TSC
    5B,
    1B,
    7B,
    3B

    // XBA
    EB,


    // XCE
    FB,
}
