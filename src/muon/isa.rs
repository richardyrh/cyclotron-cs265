use num_derive::FromPrimitive;
use crate::muon::decode::DecodedInst;

#[derive(FromPrimitive, Clone)]
pub enum Opcode {
    LOAD   = 0b0000011,
    FENCE  = 0b0001111,
    ADDI   = 0b0010011,
    AUIPC  = 0b0010111,
    ADDIW  = 0b0011011,
    STORE  = 0b0100011,
    ADD    = 0b0110011,
    LUI    = 0b0110111,
    ADDW   = 0b0111011,
    BRANCH = 0b1100011,
    JALR   = 0b1100111,
    JAL    = 0b1101111,
    CSR    = 0b1110011,
}

impl From<&Opcode> for u16 {
    fn from(value: &Opcode) -> Self {
        value.clone() as u16
    }
}

pub struct InstAction;

impl InstAction {
    pub const WRITE_REG: u32 = 1;
    pub const MEM_LOAD: u32 = 2;
    pub const MEM_STORE: u32 = 4;
    pub const SET_PC: u32 = 8;
    pub const LINK: u32 = 16;
    pub const FENCE: u32 = 32;
}

pub trait OpImp<const N: usize> {
    fn run(&self, operands: [u32; N]) -> u32;
}

impl OpImp<1> for fn(u32) -> u32 {
    fn run(&self, operands: [u32; 1]) -> u32 {
        self(operands[0])
    }
}

impl OpImp<2> for fn(u32, u32) -> u32 {
    fn run(&self, operands: [u32; 2]) -> u32 {
        self(operands[0], operands[1])
    }
}

impl OpImp<3> for fn(u32, u32, u32) -> u32 {
    fn run(&self, operands: [u32; 3]) -> u32 {
        self(operands[0], operands[1], operands[2])
    }
}

pub struct InstImp<const N: usize> {
    name: String,
    opcode: Opcode,
    f3: Option<u8>,
    f7: Option<u8>,
    actions: u32,
    op: Box<dyn OpImp<N>>,
}

impl InstImp<0> {
    pub fn una(name: &str, opcode: Opcode, actions: u32, op: fn(u32) -> u32) -> InstImp<1> {
        InstImp::<1> { name: name.to_string(), opcode, f3: None, f7: None, actions, op: Box::new(op) }
    }

    pub fn bin_f3_f7(name: &str, opcode: Opcode, f3: u8, f7: u8, actions: u32, op: fn(u32, u32) -> u32) -> InstImp<2> {
        InstImp::<2> { name: name.to_string(), opcode, f3: Some(f3), f7: Some(f7), actions, op: Box::new(op) }
    }

    pub fn bin_f3(name: &str, opcode: Opcode, f3: u8, actions: u32, op: fn(u32, u32) -> u32) -> InstImp<2> {
        InstImp::<2> { name: name.to_string(), opcode, f3: Some(f3), f7: None, actions, op: Box::new(op) }
    }

    pub fn bin(name: &str, opcode: Opcode, actions: u32, op: fn(u32, u32) -> u32) -> InstImp<2> {
        InstImp::<2> { name: name.to_string(), opcode, f3: None, f7: None, actions, op: Box::new(op) }
    }

    pub fn ter_f3(name: &str, opcode: Opcode, f3: u8, actions: u32, op: fn(u32, u32, u32) -> u32) -> InstImp<3> {
        InstImp::<3> { name: name.to_string(), opcode, f3: Some(f3), f7: None, actions, op: Box::new(op) }
    }
}
pub struct InstGroupVariant<const N: usize> {
    pub insts: Vec<InstImp<N>>,
    pub get_operands: fn(&DecodedInst) -> [u32; N],
}

impl<const N: usize> InstGroupVariant<N> {
    // returns Some((alu result, actions)) if opcode, f3 and f7 matches
    fn execute(&self, req: &DecodedInst) -> Option<(u32, u32)> {
        let operands = (self.get_operands)(&req);

        self.insts.iter().map(|inst| {
            match inst {
                InstImp { opcode, f3: Some(f3), f7: Some(f7), op, .. } => {
                    (req.opcode == opcode.into() && req.f3 == *f3 && req.f7 == *f7).then(|| op.run(operands))
                },
                InstImp { opcode, f3: Some(f3), f7: _, op, .. } => {
                    (req.opcode == opcode.into() && req.f3 == *f3).then(|| op.run(operands))
                },
                InstImp { opcode, f3: _, f7: _, op, .. } => {
                    (req.opcode == opcode.into()).then(|| op.run(operands))
                },
            }.map(|alu| (alu, inst.actions))
        }).fold(None, Option::or)
    }
}

pub enum InstGroup {
    Unary(InstGroupVariant<1>),
    Binary(InstGroupVariant<2>),
    Ternary(InstGroupVariant<3>),
}

impl InstGroup {
    pub fn execute(&self, req: &DecodedInst) -> Option<(u32, u32)> {
        match self {
            InstGroup::Unary(x) => { x.execute(req) }
            InstGroup::Binary(x) => { x.execute(req) }
            InstGroup::Ternary(x) => { x.execute(req) }
        }
    }
}

// TODO: make this all constant
pub struct ISA;
impl ISA {
    pub fn get_insts() -> Vec<Box<InstGroup>> {
        let r3_inst_imps: Vec<InstImp<2>> = vec![
            InstImp::bin_f3_f7("add",  Opcode::ADD, 0,  0, InstAction::WRITE_REG, |a, b| { a + b }),
            InstImp::bin_f3_f7("sub",  Opcode::ADD, 0, 32, InstAction::WRITE_REG, |a, b| { ((a as i32) - (b as i32)) as u32 }),
            InstImp::bin_f3_f7("sll",  Opcode::ADD, 1,  0, InstAction::WRITE_REG, |a, b| { a << (b & 31) }),
            InstImp::bin_f3_f7("slt",  Opcode::ADD, 2,  0, InstAction::WRITE_REG, |a, b| { if (a as i32) < (b as i32) { 1 } else { 0 } }),
            InstImp::bin_f3_f7("sltu", Opcode::ADD, 3,  0, InstAction::WRITE_REG, |a, b| { if a < b { 1 } else { 0 } }),
            InstImp::bin_f3_f7("xor",  Opcode::ADD, 4,  0, InstAction::WRITE_REG, |a, b| { a ^ b }),
            InstImp::bin_f3_f7("srl",  Opcode::ADD, 5,  0, InstAction::WRITE_REG, |a, b| { a >> (b & 31) }),
            InstImp::bin_f3_f7("sra",  Opcode::ADD, 5, 32, InstAction::WRITE_REG, |a, b| { ((a as i32) >> (b & 31)) as u32 }),
            InstImp::bin_f3_f7("or",   Opcode::ADD, 6,  0, InstAction::WRITE_REG, |a, b| { a | b }),
            InstImp::bin_f3_f7("and",  Opcode::ADD, 7,  0, InstAction::WRITE_REG, |a, b| { a & b }),
        ];
        let r3_insts = InstGroupVariant {
            insts: r3_inst_imps,
            get_operands: |req| [req.rs1, req.rs2],
        };

        let i2_inst_imps: Vec<InstImp<2>> = vec![
            InstImp::bin_f3("lb",  Opcode::LOAD, 0, InstAction::MEM_LOAD, |a, b| { a + b }),
            InstImp::bin_f3("lh",  Opcode::LOAD, 1, InstAction::MEM_LOAD, |a, b| { a + b }),
            InstImp::bin_f3("lw",  Opcode::LOAD, 2, InstAction::MEM_LOAD, |a, b| { a + b }),
            /* InstImp::bin_f3("ld",  Opcode::LOAD, 3, InstAction::MEM_LOAD, |a, b| { a + b }), */
            InstImp::bin_f3("lbu", Opcode::LOAD, 4, InstAction::MEM_LOAD, |a, b| { a + b }),
            InstImp::bin_f3("lhu", Opcode::LOAD, 5, InstAction::MEM_LOAD, |a, b| { a + b }),
            InstImp::bin_f3("lwu", Opcode::LOAD, 6, InstAction::MEM_LOAD, |a, b| { a + b }),

            InstImp::bin_f3("fence", Opcode::FENCE, 0, InstAction::FENCE, |a, b| { todo!() }),

            InstImp::bin_f3("addi",    Opcode::ADDI, 0,     InstAction::WRITE_REG, |a, b| { a + b }),
            InstImp::bin_f3_f7("slli", Opcode::ADDI, 1,  0, InstAction::WRITE_REG, |a, b| { a << (b & 31) }),
            InstImp::bin_f3("slti",    Opcode::ADDI, 2,     InstAction::WRITE_REG, |a, b| { if (a as i32) < (b as i32) { 1 } else { 0 } }),
            InstImp::bin_f3("sltiu",   Opcode::ADDI, 3,     InstAction::WRITE_REG, |a, b| { if a < b { 1 } else { 0 } }),
            InstImp::bin_f3("xori",    Opcode::ADDI, 4,     InstAction::WRITE_REG, |a, b| { a ^ b }),
            InstImp::bin_f3_f7("srli", Opcode::ADDI, 5,  0, InstAction::WRITE_REG, |a, b| { a >> (b & 31) }),
            InstImp::bin_f3_f7("srai", Opcode::ADDI, 5, 32, InstAction::WRITE_REG, |a, b| { ((a as i32) >> (b & 31)) as u32 }),
            InstImp::bin_f3("ori",     Opcode::ADDI, 6,     InstAction::WRITE_REG, |a, b| { a | b }),
            InstImp::bin_f3("andi",    Opcode::ADDI, 7,     InstAction::WRITE_REG, |a, b| { a & b }),

            InstImp::bin_f3("jalr", Opcode::JALR, 0, InstAction::SET_PC | InstAction::LINK, |a, b| { a + b }),
        ];
        let i2_insts = InstGroupVariant {
            insts: i2_inst_imps,
            get_operands: |req| [req.rs1, req.imm32 as u32],
        };

        // does not return anything
        let s_inst_imps: Vec<InstImp<2>> = vec![
            InstImp::bin_f3("sb", Opcode::STORE, 0, InstAction::MEM_STORE, |a, imm| { a + imm }),
            InstImp::bin_f3("sh", Opcode::STORE, 1, InstAction::MEM_STORE, |a, imm| { a + imm }),
            InstImp::bin_f3("sw", Opcode::STORE, 2, InstAction::MEM_STORE, |a, imm| { a + imm }),
            /* InstImp::bin_f3("sd", Opcode::STORE, 3, InstAction::MEM_STORE, |a, imm| { a + imm }), */
        ];
        let s_insts = InstGroupVariant {
            insts: s_inst_imps,
            get_operands: |req| [req.rs1, req.imm24 as u32],
        };

        // binary op returns branch offset if taken, 0 if not
        let sb_inst_imps: Vec<InstImp<3>> = vec![
            InstImp::ter_f3("beq",  Opcode::BRANCH, 0, InstAction::SET_PC, |a, b, imm| { if a == b { imm } else { 0 }  }),
            InstImp::ter_f3("bne",  Opcode::BRANCH, 1, InstAction::SET_PC, |a, b, imm| { if a != b { imm } else { 0 }  }),
            InstImp::ter_f3("blt",  Opcode::BRANCH, 4, InstAction::SET_PC, |a, b, imm| { if (a as i32) < (b as i32) { imm } else { 0 }  }),
            InstImp::ter_f3("bge",  Opcode::BRANCH, 5, InstAction::SET_PC, |a, b, imm| { if (a as i32) >= (b as i32) { imm } else { 0 }  }),
            InstImp::ter_f3("bltu", Opcode::BRANCH, 6, InstAction::SET_PC, |a, b, imm| { if a < b { imm } else { 0 }  }),
            InstImp::ter_f3("bgeu", Opcode::BRANCH, 7, InstAction::SET_PC, |a, b, imm| { if a >= b { imm } else { 0 }  }),
        ];
        let sb_insts = InstGroupVariant {
            insts: sb_inst_imps,
            get_operands: |req| [req.rs1, req.rs2, req.imm24 as u32],
        };

        let pcrel_inst_imps: Vec<InstImp<2>> = vec![
            InstImp::bin("auipc", Opcode::AUIPC, InstAction::WRITE_REG, |a, b| { a + b }),
            InstImp::bin("jal", Opcode::JAL, InstAction::SET_PC | InstAction::LINK, |a, b| { a + b }),
        ];
        let pcrel_insts = InstGroupVariant {
            insts: pcrel_inst_imps,
            get_operands: |req| [req.pc, req.imm32 as u32],
        };

        // let lui_inst_imp: Vec<InstImp<1>> = vec![
        //     InstImp::una("lui", Opcode::LUI, InstAction::WRITE_REG, |a| { a }),
        // ];
        // let lui_inst = InstOpcode {
        //     insts: lui_inst_imp,
        //     get_operands: |req| [req.imm32 as u32],
        // };

        // let all_una_insts = vec![
        //     Box::new(lui_inst),
        // ];

        vec![
            Box::new(InstGroup::Binary(r3_insts)),
            Box::new(InstGroup::Binary(i2_insts)),
            Box::new(InstGroup::Binary(s_insts)),
            Box::new(InstGroup::Binary(pcrel_insts)),
            Box::new(InstGroup::Ternary(sb_insts)),
        ]
    }
}
