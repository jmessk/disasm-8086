pub enum Mnemonic {
    // DATA TRANSFER
    MOV,
    PUSH,
    POP,
    XCHG,
    IN,
    OUT,
    XLAT,
    LEA,
    LDS,
    LES,
    LAHF,
    SAHF,
    PUSHF,
    POPF,

    // ARITHMETIC
    ADD,
    ADC,
    INC,
    AAA,
    BAA,
    SUB,
    SSB,
    DEC,
    NEG,
    CMP,
    AAS,
    DAS,
    MUL,
    IMUL,
    AAM,
    DIV,
    IDIV,
    AAD,
    CBW,
    CWD,

    // LOGIC
    NOT,
    SHL, // SAL
    SHR,
    SAR,
    ROL,
    ROR,
    RCL,
    RCR,
    AND,
    TEST,
    OR,
    XOR,

    // STRING MANIPULATION
    REP,
    MOVS,
    CMPS,
    SCAS,
    LODS,
    STOS,

    // CONTROL TRANSFER
    CALL,
    JMP,
    RET,
    JE,  // JZ
    JL,  // JNGE
    JLE, // JNG
    JB,  // JNAE
    JBE, // JNA
    JP,  // JPE
    JO,
    JS,
    JNE,  //JNZ
    JNL,  // JGE
    JNLE, // JG
    JNB,  // JAE
    JNBE, // JA
    JNP,  //JPO
    JNO,
    NJS,
    LOOP,
    LOOPZ,  // LOOPE
    LOOPNZ, // LOOPNE
    JCXZ,
    INT,
    INTO,
    IRET,

    // PROCESSOR CONTROL
    CLC,
    CMC,
    STC,
    CLD,
    STD,
    CLI,
    STI,
    HLT,
    WAIT,
    ESC,
    LOCK,
}
