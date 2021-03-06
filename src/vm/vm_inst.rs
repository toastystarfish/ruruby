use crate::*;

pub struct Inst;
impl Inst {
    pub const PUSH_FIXNUM: u8 = 1;
    pub const PUSH_FLONUM: u8 = 2;
    pub const PUSH_TRUE: u8 = 3;
    pub const PUSH_FALSE: u8 = 4;
    pub const PUSH_NIL: u8 = 5;
    pub const PUSH_STRING: u8 = 6;
    pub const PUSH_SYMBOL: u8 = 7;
    pub const PUSH_SELF: u8 = 8;

    pub const ADD: u8 = 10;
    pub const SUB: u8 = 11;
    pub const MUL: u8 = 12;
    pub const DIV: u8 = 13;
    pub const REM: u8 = 14;
    pub const EQ: u8 = 15;
    pub const NE: u8 = 16;
    pub const TEQ: u8 = 17;
    pub const GT: u8 = 18;
    pub const GE: u8 = 19;
    pub const NOT: u8 = 20;
    pub const SHR: u8 = 21;
    pub const SHL: u8 = 22;
    pub const BIT_OR: u8 = 23;
    pub const BIT_AND: u8 = 24;
    pub const BIT_XOR: u8 = 25;
    pub const BIT_NOT: u8 = 26;
    pub const POW: u8 = 27;
    pub const CMP: u8 = 28;

    pub const ADDI: u8 = 30;
    pub const SUBI: u8 = 31;
    pub const IVAR_ADDI: u8 = 32;

    pub const SET_LOCAL: u8 = 40;
    pub const GET_LOCAL: u8 = 41;
    pub const GET_CONST: u8 = 42;
    pub const SET_CONST: u8 = 43;
    pub const GET_CONST_TOP: u8 = 44;
    pub const GET_SCOPE: u8 = 45;

    pub const GET_IVAR: u8 = 46;
    pub const SET_IVAR: u8 = 47;
    pub const GET_GVAR: u8 = 48;
    pub const SET_GVAR: u8 = 49;
    pub const GET_INDEX: u8 = 50;
    pub const SET_INDEX: u8 = 51;

    pub const CHECK_LOCAL: u8 = 52;

    pub const SEND: u8 = 60;
    pub const SEND_SELF: u8 = 61;

    pub const CREATE_RANGE: u8 = 70;
    pub const CREATE_ARRAY: u8 = 71;
    pub const CREATE_PROC: u8 = 72;
    pub const CREATE_HASH: u8 = 73;
    pub const CREATE_REGEXP: u8 = 74;

    pub const POP: u8 = 80;
    pub const DUP: u8 = 81;
    pub const TAKE: u8 = 82;
    pub const SPLAT: u8 = 83;
    pub const CONCAT_STRING: u8 = 84;
    pub const TO_S: u8 = 85;

    pub const DEF_CLASS: u8 = 90;
    pub const DEF_METHOD: u8 = 91;
    pub const DEF_SMETHOD: u8 = 92;

    pub const JMP: u8 = 100;
    pub const JMP_IF_FALSE: u8 = 101;
    pub const END: u8 = 102;
    pub const RETURN: u8 = 103;
    pub const OPT_CASE: u8 = 104;
    pub const MRETURN: u8 = 105;
    pub const YIELD: u8 = 106;
}

#[allow(dead_code)]
impl Inst {
    pub fn inst_name(inst: u8) -> &'static str {
        match inst {
            Inst::PUSH_FIXNUM => "PUSH_FIXNUM",
            Inst::PUSH_FLONUM => "PUSH_FLONUM",
            Inst::PUSH_TRUE => "PUSH_TRUE",
            Inst::PUSH_FALSE => "PUSH_FALSE",
            Inst::PUSH_NIL => "PUSH_NIL",
            Inst::PUSH_STRING => "PUSH_STRING",
            Inst::PUSH_SYMBOL => "PUSH_SYMBOL",
            Inst::PUSH_SELF => "PUSH_SELF",

            Inst::ADD => "ADD",
            Inst::SUB => "SUB",
            Inst::MUL => "MUL",
            Inst::DIV => "DIV",
            Inst::REM => "REM",
            Inst::EQ => "EQ",
            Inst::NE => "NE",
            Inst::TEQ => "TEQ",
            Inst::GT => "GT",
            Inst::GE => "GE",
            Inst::CMP => "CMP",
            Inst::NOT => "NOT",
            Inst::SHR => "SHR",
            Inst::SHL => "SHL",
            Inst::BIT_OR => "BIT_OR",
            Inst::BIT_AND => "BIT_AND",
            Inst::BIT_XOR => "BIT_XOR",
            Inst::BIT_NOT => "BIT_NOT",
            Inst::POW => "POW",

            Inst::ADDI => "ADDI",
            Inst::SUBI => "SUBI",
            Inst::IVAR_ADDI => "IVAR_ADDI",

            Inst::SET_LOCAL => "SET_LOCAL",
            Inst::GET_LOCAL => "GET_LOCAL",
            Inst::GET_CONST => "GET_CONST",
            Inst::SET_CONST => "SET_CONST",
            Inst::GET_CONST_TOP => "GET_CONSTTOP",
            Inst::GET_SCOPE => "GET_SCOPE",

            Inst::GET_IVAR => "GET_IVAR",
            Inst::SET_IVAR => "SET_IVAR",
            Inst::GET_GVAR => "GET_GVAR",
            Inst::SET_GVAR => "SET_GVAR",
            Inst::GET_INDEX => "GET_INDEX",
            Inst::SET_INDEX => "SET_INDEX",

            Inst::SEND => "SEND",
            Inst::SEND_SELF => "SEND_SELF",

            Inst::CHECK_LOCAL => "CHECK_LOCAL",

            Inst::CREATE_RANGE => "CREATE_RANGE",
            Inst::CREATE_ARRAY => "CREATE_ARRAY",
            Inst::CREATE_PROC => "CREATE_PROC",
            Inst::CREATE_HASH => "CREATE_HASH",
            Inst::CREATE_REGEXP => "CREATE_REGEX",

            Inst::POP => "POP",
            Inst::DUP => "DUP",
            Inst::TAKE => "TAKE",
            Inst::SPLAT => "SPLAT",
            Inst::CONCAT_STRING => "CONCAT_STR",
            Inst::TO_S => "TO_S",

            Inst::DEF_CLASS => "DEF_CLASS",
            Inst::DEF_METHOD => "DEF_METHOD",
            Inst::DEF_SMETHOD => "DEF_CMETHOD",

            Inst::JMP => "JMP",
            Inst::JMP_IF_FALSE => "JMP_IF_FALSE",
            Inst::END => "END",
            Inst::RETURN => "RETURN",
            Inst::OPT_CASE => "OPT_CASE",
            Inst::MRETURN => "MRETURN",
            Inst::YIELD => "YIELD",

            _ => "undefined",
        }
    }

    pub fn inst_size(inst: u8) -> usize {
        match inst {
            Inst::END
            | Inst::PUSH_NIL
            | Inst::PUSH_TRUE
            | Inst::PUSH_FALSE
            | Inst::PUSH_SELF
            | Inst::DIV
            | Inst::REM
            | Inst::POW
            | Inst::EQ
            | Inst::NE
            | Inst::GT
            | Inst::GE
            | Inst::CMP
            | Inst::NOT
            | Inst::SHR
            | Inst::BIT_OR
            | Inst::BIT_AND
            | Inst::BIT_XOR
            | Inst::BIT_NOT
            | Inst::CONCAT_STRING
            | Inst::CREATE_RANGE
            | Inst::CREATE_REGEXP
            | Inst::TO_S
            | Inst::SPLAT
            | Inst::POP
            | Inst::RETURN
            | Inst::MRETURN => 1,
                                        // operand
            Inst::PUSH_STRING           // IdentId: u32
            | Inst::PUSH_SYMBOL         // IdentId: u32
            | Inst::GET_CONST           // IdentId: u32
            | Inst::SET_CONST           // IdentId: u32
            | Inst::GET_CONST_TOP       // IdentId: u32
            | Inst::GET_SCOPE           // IdentId: u32
            | Inst::GET_IVAR            // IdentId: u32
            | Inst::SET_IVAR            // IdentId: u32
            | Inst::GET_GVAR            // IdentId: u32
            | Inst::SET_GVAR            // IdentId: u32
            | Inst::GET_INDEX
            | Inst::SET_INDEX
            | Inst::CREATE_ARRAY        // number of items: u32
            | Inst::CREATE_PROC
            | Inst::JMP                 // disp: u32
            | Inst::JMP_IF_FALSE        // disp: u32
            | Inst::DUP                 // number of items: u32
            | Inst::TAKE                // number of items: u32
            | Inst::ADD                 // inline cache: u32
            | Inst::SUB                 // inline cache: u32
            | Inst::MUL                 // inline cache: u32
            | Inst::ADDI                // inline cache: u32
            | Inst::SUBI                // inline cache: u32
            | Inst::SHL                 // inline cache: u32
            | Inst::CREATE_HASH         // number of items: u32
            | Inst::YIELD               // number of items: u32
            => 5,

            Inst::PUSH_FIXNUM
            | Inst::PUSH_FLONUM
            | Inst::SET_LOCAL
            | Inst::GET_LOCAL
            | Inst::DEF_METHOD
            | Inst::DEF_SMETHOD
            | Inst::OPT_CASE
            | Inst::IVAR_ADDI => 9,
            Inst::DEF_CLASS => 10,
            Inst::SEND | Inst::SEND_SELF => 17,
            _ => 1,
        }
    }

    pub fn inst_info(globals: &mut Globals, iseq_ref: ISeqRef, pc: usize) -> String {
        let iseq = &iseq_ref.iseq;
        match iseq[pc] {
            Inst::END
            | Inst::PUSH_NIL
            | Inst::PUSH_TRUE
            | Inst::PUSH_FALSE
            | Inst::PUSH_SELF
            | Inst::ADD
            | Inst::SUB
            | Inst::MUL
            | Inst::DIV
            | Inst::REM
            | Inst::EQ
            | Inst::NE
            | Inst::GT
            | Inst::GE
            | Inst::CMP
            | Inst::NOT
            | Inst::SHR
            | Inst::SHL
            | Inst::BIT_OR
            | Inst::BIT_AND
            | Inst::BIT_XOR
            | Inst::BIT_NOT
            | Inst::CONCAT_STRING
            | Inst::CREATE_RANGE
            | Inst::CREATE_REGEXP
            | Inst::RETURN
            | Inst::TO_S
            | Inst::SPLAT
            | Inst::POP
            | Inst::YIELD => format!("{}", Inst::inst_name(iseq[pc])),
            Inst::PUSH_STRING => format!("PUSH_STRING {}", Inst::read32(iseq, pc + 1) as i32),
            Inst::PUSH_SYMBOL => format!("PUSH_SYMBOL {}", Inst::read32(iseq, pc + 1) as i32),
            Inst::ADDI => format!("ADDI {}", Inst::read32(iseq, pc + 1) as i32),
            Inst::SUBI => format!("SUBI {}", Inst::read32(iseq, pc + 1) as i32),
            Inst::PUSH_FIXNUM => format!("PUSH_FIXNUM {}", Inst::read64(iseq, pc + 1) as i64),
            Inst::PUSH_FLONUM => {
                format!("PUSH_FLONUM {}", f64::from_bits(Inst::read64(iseq, pc + 1)))
            }

            Inst::JMP => format!(
                "JMP {:>05x}",
                pc as i32 + 5 + Inst::read32(iseq, pc + 1) as i32
            ),
            Inst::JMP_IF_FALSE => format!(
                "JMP_IF_FALSE {:>05x}",
                pc as i32 + 5 + Inst::read32(iseq, pc + 1) as i32
            ),
            Inst::OPT_CASE => {
                //let val = Value::from(Inst::read64(iseq, pc + 1));
                //let info = val.as_hash().unwrap();
                /*let map = match info.inner_mut() {
                    HashInfo::Map(map) => map,
                    _ => panic!(),
                };*/
                format!(
                    "OPT_CASE {:>05}",
                    pc as i32 + 13 + Inst::read32(iseq, pc + 9) as i32,
                )
            }
            Inst::SET_LOCAL => {
                let frame = Inst::read32(iseq, pc + 5);
                let id = Inst::read32(iseq, pc + 1) as usize;
                let ident_id = iseq_ref.lvar.get_name(LvarId::from_usize(id));
                let name = globals.get_ident_name(ident_id);
                format!("SET_LOCAL '{}' outer:{} LvarId:{}", name, frame, id)
            }
            Inst::GET_LOCAL => {
                let frame = Inst::read32(iseq, pc + 5);
                let id = Inst::read32(iseq, pc + 1) as usize;
                let ident_id = iseq_ref.lvar.get_name(LvarId::from_usize(id));
                let name = globals.get_ident_name(ident_id);
                format!("GET_LOCAL '{}' outer:{} LvarId:{}", name, frame, id)
            }
            Inst::CHECK_LOCAL => {
                let frame = Inst::read32(iseq, pc + 5);
                let id = Inst::read32(iseq, pc + 1) as usize;
                let ident_id = iseq_ref.lvar.get_name(LvarId::from_usize(id));
                let name = globals.get_ident_name(ident_id);
                format!("CHECK_LOCAL '{}' outer:{} LvarId:{}", name, frame, id)
            }
            Inst::GET_CONST => format!("GET_CONST '{}'", Inst::ident_name(globals, iseq, pc + 1)),
            Inst::GET_CONST_TOP => format!(
                "GET_CONST_TOP '{}'",
                Inst::ident_name(globals, iseq, pc + 1)
            ),
            Inst::SET_CONST => format!("SET_CONST '{}'", Inst::ident_name(globals, iseq, pc + 1)),
            Inst::GET_SCOPE => format!("GET_SCOPE '{}'", Inst::ident_name(globals, iseq, pc + 1)),
            Inst::GET_IVAR => format!("GET_IVAR '{}'", Inst::ident_name(globals, iseq, pc + 1)),
            Inst::SET_IVAR => format!("SET_IVAR '{}'", Inst::ident_name(globals, iseq, pc + 1)),
            Inst::GET_INDEX => format!("GET_INDEX {} items", Inst::read32(iseq, pc + 1)),
            Inst::SET_INDEX => format!("SET_INDEX {} items", Inst::read32(iseq, pc + 1)),
            Inst::SEND => format!(
                "SEND '{}' {} items",
                Inst::ident_name(globals, iseq, pc + 1),
                Inst::read32(iseq, pc + 5)
            ),
            Inst::SEND_SELF => format!(
                "SEND_SELF '{}' {} items",
                Inst::ident_name(globals, iseq, pc + 1),
                Inst::read32(iseq, pc + 5)
            ),

            Inst::CREATE_ARRAY => format!("CREATE_ARRAY {} items", Inst::read32(iseq, pc + 1)),
            Inst::CREATE_PROC => format!("CREATE_PROC method:{}", Inst::read32(iseq, pc + 1)),
            Inst::CREATE_HASH => format!("CREATE_HASH {} items", Inst::read32(iseq, pc + 1)),
            Inst::DUP => format!("DUP {}", Inst::read32(iseq, pc + 1)),
            Inst::TAKE => format!("TAKE {}", Inst::read32(iseq, pc + 1)),
            Inst::DEF_CLASS => format!(
                "DEF_CLASS {} '{}' method:{}",
                if Inst::read8(iseq, pc + 1) == 1 {
                    "module"
                } else {
                    "class"
                },
                Inst::ident_name(globals, iseq, pc + 2),
                Inst::read32(iseq, pc)
            ),
            Inst::DEF_METHOD => format!("DEF_METHOD '{}'", Inst::ident_name(globals, iseq, pc + 1)),
            Inst::DEF_SMETHOD => {
                format!("DEF_SMETHOD '{}'", Inst::ident_name(globals, iseq, pc + 1))
            }
            _ => format!("undefined"),
        }
    }

    fn read64(iseq: &ISeq, pc: usize) -> u64 {
        let ptr = iseq[pc..pc + 1].as_ptr() as *const u64;
        unsafe { *ptr }
    }

    fn read32(iseq: &ISeq, pc: usize) -> u32 {
        let ptr = iseq[pc..pc + 1].as_ptr() as *const u32;
        unsafe { *ptr }
    }

    fn read8(iseq: &ISeq, pc: usize) -> u8 {
        iseq[pc]
    }

    fn ident_name<'a>(globals: &'a Globals, iseq: &ISeq, pc: usize) -> &'a str {
        globals.get_ident_name(IdentId::from(Inst::read32(iseq, pc)))
    }
}
