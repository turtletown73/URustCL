use std::{fmt::Debug, fs};
use strum_macros::EnumString;
use std::str::FromStr;

impl Debug for Architecture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ROM => write!(f, "ROM"),
            Self::RAM => write!(f, "RAM"),
        }
    }
}
pub enum Architecture {
    ROM, RAM
}

impl Debug for Opcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ADD => write!(f, "ADD"),
            Self::RSH => write!(f, "RSH"),
            Self::LOD => write!(f, "LOD"),
            Self::STR => write!(f, "STR"),
            Self::BGE => write!(f, "BGE"),
            Self::NOR => write!(f, "NOR"),
            Self::IMM => write!(f, "IMM"),
            Self::SUB => write!(f, "SUB"),
            Self::JMP => write!(f, "JMP"),
            Self::MOV => write!(f, "MOV"),
            Self::NOP => write!(f, "NOP"),
            Self::LSH => write!(f, "LSH"),
            Self::INC => write!(f, "INC"),
            Self::DEC => write!(f, "DEC"),
            Self::NEG => write!(f, "NEG"),
            Self::AND => write!(f, "AND"),
            Self::OR => write!(f, "OR"),
            Self::NOT => write!(f, "NOT"),
            Self::XNOR => write!(f, "XNOR"),
            Self::XOR => write!(f, "XOR"),
            Self::NAND => write!(f, "NAND"),
            Self::BRL => write!(f, "BRL"),
            Self::BRG => write!(f, "BRG"),
            Self::BRE => write!(f, "BRE"),
            Self::BNE => write!(f, "BNE"),
            Self::BOD => write!(f, "BOD"),
            Self::BEV => write!(f, "BEV"),
            Self::BLE => write!(f, "BLE"),
            Self::BRZ => write!(f, "BRZ"),
            Self::BNZ => write!(f, "BNZ"),
            Self::BRN => write!(f, "BRN"),
            Self::BRP => write!(f, "BRP"),
            Self::PSH => write!(f, "PSH"),
            Self::POP => write!(f, "POP"),
            Self::CAL => write!(f, "CAL"),
            Self::RET => write!(f, "RET"),
            Self::HLT => write!(f, "HLT"),
            Self::CPY => write!(f, "CPY"),
            Self::BRC => write!(f, "BRC"),
            Self::BNC => write!(f, "BNC"),
            Self::MLT => write!(f, "MLT"),
            Self::DIV => write!(f, "DIV"),
            Self::MOD => write!(f, "MOD"),
            Self::BSR => write!(f, "BSR"),
            Self::BSL => write!(f, "BSL"),
            Self::SRS => write!(f, "SRS"),
            Self::BSS => write!(f, "BSS"),
            Self::SETE => write!(f, "SETE"),
            Self::SETNE => write!(f, "SETNE"),
            Self::SETG => write!(f, "SETG"),
            Self::SETL => write!(f, "SETL"),
            Self::SETGE => write!(f, "SETGE"),
            Self::SETLE => write!(f, "SETLE"),
            Self::SETC => write!(f, "SETC"),
            Self::SETNC => write!(f, "SETNC"),
            Self::LLOD => write!(f, "LLOD"),
            Self::LSTR => write!(f, "LSTR"),
            Self::IN => write!(f, "IN"),
            Self::OUT => write!(f, "OUT"),
            Self::SDIV => write!(f, "SDIV"),
            Self::SBRL => write!(f, "SBRL"),
            Self::SBRG => write!(f, "SBRG"),
            Self::SBLE => write!(f, "SBLE"),
            Self::SBGE => write!(f, "SBGE"),
            Self::SSETL => write!(f, "SSETL"),
            Self::SSETG => write!(f, "SSETG"),
            Self::SSETLE => write!(f, "SSETLE"),
            Self::SSETGE => write!(f, "SSETGE"),
            Self::ABS => write!(f, "ABS"),
            Self::__ASSERT => write!(f, "__ASSERT"),
            Self::__ASSERT0 => write!(f, "__ASSERT0"),
            Self::__ASSERT_EQ => write!(f, "__ASSERT_EQ"),
            Self::__ASSERT_NEQ => write!(f, "__ASSERT_NEQ"),
            Self::UMLT => write!(f, "UMLT"),
            Self::SUMLT => write!(f, "SUMLT"),
        }
    }
}
#[derive(EnumString)]
pub enum Opcode {
    // Core Instructions
    ADD, RSH, LOD, STR, BGE, NOR, IMM,
    // Basic Instructions
    SUB, JMP, MOV, NOP, LSH, INC, DEC, NEG,
    AND, OR, NOT, XNOR, XOR, NAND,
    BRL, BRG, BRE, BNE, BOD, BEV, BLE, BRZ,
    BNZ, BRN, BRP, PSH, POP, CAL, RET, HLT,
    CPY, BRC, BNC,

    // Complex Instructions
    MLT, DIV, MOD, BSR, BSL, SRS, BSS,
    SETE, SETNE, SETG, SETL, SETGE, SETLE,
    SETC, SETNC, LLOD, LSTR,

    // IO Instructions
    IN, OUT,
    
    // Signed Instructions
    SDIV, SBRL, SBRG, SBLE , SBGE, SSETL, SSETG, SSETLE, SSETGE,
    ABS,

    //----- Debug Instructions
    __ASSERT,
    __ASSERT0,
    __ASSERT_EQ,
    __ASSERT_NEQ,

    //----- experimental instructions
    UMLT, SUMLT
}

impl Debug for ArgumentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Register => write!(f, "Register"),
            Self::Memory => write!(f, "Memory"),
            Self::Label => write!(f, "Label"),
            Self::RelativeNumber => write!(f, "RelativeNumber"),
            Self::Port => write!(f, "Port"),
            Self::Immediate => write!(f, "Immediate"),
            Self::DefinedImmediate => write!(f, "DefinedImmediate"),
            Self::ASCII => write!(f, "ASCII"),
        }
    }
}
pub enum ArgumentType {
    Register, Memory, Label, RelativeNumber, Port, Immediate, DefinedImmediate, ASCII
}

impl Debug for ArgumentValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(arg0) => f.debug_tuple("String").field(arg0).finish(),
            Self::U32(arg0) => f.debug_tuple("U32").field(arg0).finish(),
        }
    }
}
pub enum ArgumentValue {
    String(String),
    U32(u32)
}

impl Debug for Argument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Argument").field("argtype", &self.argtype).field("value", &self.value).finish()
    }
}
pub struct Argument {
    argtype: ArgumentType,
    value: ArgumentValue
}

impl Debug for Headers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Headers").field("bits", &self.bits).field("minreg", &self.minreg).field("minheap", &self.minheap).field("architecture", &self.architecture).field("minstack", &self.minstack).finish()
    }
}
pub struct Headers {
    bits: u32,
    minreg: u32,
    minheap: u32,
    architecture: Architecture,
    minstack: u32
}

impl Default for Headers {
    fn default() -> Self {
        Headers { bits: 8, minreg: 8, minheap: 16, architecture: Architecture::ROM, minstack: 8 }
    }
}

impl Debug for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Label").field("name", &self.name).field("line", &self.line).finish()
    }
}
pub struct Label {
    name: String,
    line: u32
}

impl Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Instruction").field("opcode", &self.opcode).field("args", &self.args).finish()
    }
}
pub struct Instruction {
    opcode: Opcode,
    args: Vec<Argument>
}

impl Debug for Interpreted {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Interpreted").field("headers", &self.headers).field("lines", &self.lines).field("labels", &self.labels).field("instructions", &self.instructions).finish()
    }
}
pub struct Interpreted {
    headers: Headers,
    lines: Vec<String>,
    labels: Vec<Label>,
    instructions: Vec<Instruction>
}

pub fn interpret(dir: String) -> Interpreted {
    let contents: String  = fs::read_to_string(dir).expect("aa"); 
    let mut lines: Vec<String> = Vec::new();
    let mut headers = Headers::default();
    let mut labels: Vec<Label> = Vec::new();
    let mut instructions: Vec<Instruction> = Vec::new();
    for (i, x) in contents.split("\n").enumerate() {
        lines.push(x.to_string());
        if x.starts_with("BITS ") {
            headers.bits = x.get(5..).as_slice().concat().trim().parse().unwrap();
        } else if x.starts_with("MINREG ") {
            headers.minreg = x.get(7..).as_slice().concat().trim().parse().unwrap();
        } else if x.starts_with("MINHEAP ") {
            headers.minheap = x.get(8..).as_slice().concat().trim().parse().unwrap();
        } else if x.starts_with("RUN ROM") {
            headers.architecture = Architecture::ROM;
        } else if x.starts_with("RUN RAM") {
            headers.architecture = Architecture::RAM;
        } else if x.starts_with("MINSTACK ") {
            headers.minstack = x.get(9..).as_slice().concat().trim().parse().unwrap();
        } else if x.starts_with(".") {
            labels.push(Label { name: x.get(1..).as_slice().concat().trim().parse().unwrap(), line: i as u32 + 1});
        } else {
            let mut instruction = Instruction {opcode: Opcode::ABS, args: Vec::new()};
            for (i, x) in x.trim().split(" ").enumerate() {
                if i as u32 == 1 {
                    instruction.opcode = Opcode::from_str(x).unwrap();
                } else {
                    if (x.starts_with("R") || x.starts_with("$")) {
                        let argument = Argument {argtype: ArgumentType::Register, value: ArgumentValue::U32(x.get(1..).as_slice().concat().trim().parse().unwrap())};
                        instruction.args.push(argument);
                    } else if x.starts_with(".") {
                        let argument = Argument {argtype: ArgumentType::Label, value: ArgumentValue::String(x.get(1..).as_slice().concat())};
                        instruction.args.push(argument);
                    } else {
                        let argument = Argument {argtype: ArgumentType::Immediate, value: ArgumentValue::U32(x.trim().parse().unwrap())};
                        instruction.args.push(argument);
                    }
                }
            }
            instructions.push(instruction);
        }
    }

    return Interpreted {
        headers: headers,
        lines: lines,
        labels: labels,
        instructions: instructions
    }
}