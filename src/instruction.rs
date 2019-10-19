use crate::Context;


construct_uint! {
    pub struct U256(4);
}

const WORD_BYTE_SIZE: usize = 32;
const NEGATIVE_BIT: u64 = std::u64::MAX / 2 + 1;

#[derive(Debug, Clone, Copy)]
pub struct Word {
    raw: [u8; WORD_BYTE_SIZE]
}

impl From<U256> for Word {
    fn from(u: U256) -> Self {
        let buf: &mut [u8; 32] = &mut [0; 32];
        u.to_big_endian(buf);
        Word { raw: *buf }
    }
}

impl From<Word> for U256 {
    fn from(w: Word) -> Self {
        U256::from_big_endian(&w.raw)
    }
}

impl U256 {
    pub fn is_negative(&self) -> bool {
        self.0[3] >= NEGATIVE_BIT
    }

    pub fn to_negative(mut self) -> Self {
        if !self.is_negative() {
            self = !self + 1
        }
        self
    }

    pub fn abs(mut self) -> Self {
        if self.is_negative() {
            self = !self + 1
        }
        self
    }
}

#[test]
fn test_ngative() {
    let mut num = U256::from(2).to_negative();
    assert_eq!(num, U256::from([
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xfe,
    ]));

    let mut num = U256::from([
        0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    ]).to_negative();
    assert_eq!(num, U256::from([
        0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
    ]));

    // if the num is negative, then no modify.
    let mut num = U256::from([
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xfe,
    ]).to_negative();
    assert_eq!(num, U256::from([
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xfe,
    ]));
}

#[test]
fn test_abs() {
    let mut num = U256::from(2).abs();
    assert_eq!(num, U256::from(2));

    let mut num = U256::from([
        0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    ]).abs();
    assert_eq!(num, U256::from([
        0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    ]));

    let mut num = U256::from([
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xfe,
    ]).abs();
    assert_eq!(num, U256::from(2));
}

pub trait OpcodeFn {
    fn gas_cost(&self) -> u128;
    fn exec(&self, ctx: Context) -> Context;
    fn instruct(&self, ctx: Context) -> Context {
        let mut res = self.exec(ctx);
        res.used_gas += self.gas_cost();
        res
    }
}

pub struct OpAdd;

pub struct OpMul;

pub struct OpSub;

pub struct OpDiv;

pub struct OpSDiv;

pub struct OpPush1;

pub struct OpPush2;

pub struct OpPush3;

pub struct OpPush4;

pub struct OpPush5;

pub struct OpPush6;

pub struct OpPush7;

pub struct OpPush8;

pub struct OpPush9;

pub struct OpPush10;

pub struct OpPush11;

pub struct OpPush12;

pub struct OpPush13;

pub struct OpPush14;

pub struct OpPush15;

pub struct OpPush16;

pub struct OpPush17;

pub struct OpPush18;

pub struct OpPush19;

pub struct OpPush20;

pub struct OpPush21;

pub struct OpPush22;

pub struct OpPush23;

pub struct OpPush24;

pub struct OpPush25;

pub struct OpPush26;

pub struct OpPush27;

pub struct OpPush28;

pub struct OpPush29;

pub struct OpPush30;

pub struct OpPush31;

pub struct OpPush32;

pub struct OpInvalid;

fn convert_word(data: &[u8], size: usize) -> Word {
    let mut bytes: [u8; WORD_BYTE_SIZE] = [0; WORD_BYTE_SIZE];
    let mut offset = WORD_BYTE_SIZE - size;
    for b in data {
        bytes[offset] = *b;
        offset += 1;
    }
    Word { raw: bytes }
}

pub trait OpPushGeneral {
    fn data_size(&self) -> u8;
    fn push_exec(&self, mut ctx: Context) -> Context {
        let start = ctx.pc + 1;
        let end = ctx.pc + 1 + self.data_size() as usize;
        ctx.stack.push(convert_word(&ctx.codes[start..end], self.data_size() as usize));
        ctx.pc += 1 + self.data_size() as usize;
        ctx
    }
}

impl<T: OpPushGeneral> OpcodeFn for T {
    fn gas_cost(&self) -> u128 { 3 }
    fn exec(&self, ctx: Context) -> Context {
        self.push_exec(ctx)
    }
}

impl OpPushGeneral for OpPush1 { fn data_size(&self) -> u8 { 1 } }

impl OpPushGeneral for OpPush2 { fn data_size(&self) -> u8 { 2 } }

impl OpPushGeneral for OpPush3 { fn data_size(&self) -> u8 { 3 } }

impl OpPushGeneral for OpPush4 { fn data_size(&self) -> u8 { 4 } }

impl OpPushGeneral for OpPush5 { fn data_size(&self) -> u8 { 5 } }

impl OpPushGeneral for OpPush6 { fn data_size(&self) -> u8 { 6 } }

impl OpPushGeneral for OpPush7 { fn data_size(&self) -> u8 { 7 } }

impl OpPushGeneral for OpPush8 { fn data_size(&self) -> u8 { 8 } }

impl OpPushGeneral for OpPush9 { fn data_size(&self) -> u8 { 9 } }

impl OpPushGeneral for OpPush10 { fn data_size(&self) -> u8 { 10 } }

impl OpPushGeneral for OpPush11 { fn data_size(&self) -> u8 { 11 } }

impl OpPushGeneral for OpPush12 { fn data_size(&self) -> u8 { 12 } }

impl OpPushGeneral for OpPush13 { fn data_size(&self) -> u8 { 13 } }

impl OpPushGeneral for OpPush14 { fn data_size(&self) -> u8 { 14 } }

impl OpPushGeneral for OpPush15 { fn data_size(&self) -> u8 { 15 } }

impl OpPushGeneral for OpPush16 { fn data_size(&self) -> u8 { 16 } }

impl OpPushGeneral for OpPush17 { fn data_size(&self) -> u8 { 17 } }

impl OpPushGeneral for OpPush18 { fn data_size(&self) -> u8 { 18 } }

impl OpPushGeneral for OpPush19 { fn data_size(&self) -> u8 { 19 } }

impl OpPushGeneral for OpPush20 { fn data_size(&self) -> u8 { 20 } }

impl OpPushGeneral for OpPush21 { fn data_size(&self) -> u8 { 21 } }

impl OpPushGeneral for OpPush22 { fn data_size(&self) -> u8 { 22 } }

impl OpPushGeneral for OpPush23 { fn data_size(&self) -> u8 { 23 } }

impl OpPushGeneral for OpPush24 { fn data_size(&self) -> u8 { 24 } }

impl OpPushGeneral for OpPush25 { fn data_size(&self) -> u8 { 25 } }

impl OpPushGeneral for OpPush26 { fn data_size(&self) -> u8 { 26 } }

impl OpPushGeneral for OpPush27 { fn data_size(&self) -> u8 { 27 } }

impl OpPushGeneral for OpPush28 { fn data_size(&self) -> u8 { 28 } }

impl OpPushGeneral for OpPush29 { fn data_size(&self) -> u8 { 29 } }

impl OpPushGeneral for OpPush30 { fn data_size(&self) -> u8 { 30 } }

impl OpPushGeneral for OpPush31 { fn data_size(&self) -> u8 { 31 } }

impl OpPushGeneral for OpPush32 { fn data_size(&self) -> u8 { 32 } }

impl OpcodeFn for OpAdd {
    fn gas_cost(&self) -> u128 { 3 }

    fn exec(&self, mut ctx: Context) -> Context {
        let result = U256::from(ctx.stack.pop().unwrap()).overflowing_add(U256::from(ctx.stack.pop().unwrap()));
        ctx.stack.push(Word::from(result.0));
        ctx.pc += 1;
        ctx
    }
}

impl OpcodeFn for OpMul {
    fn gas_cost(&self) -> u128 { 5 }

    fn exec(&self, mut ctx: Context) -> Context {
        let result = U256::from(ctx.stack.pop().unwrap()).overflowing_mul(U256::from(ctx.stack.pop().unwrap()));
        ctx.stack.push(Word::from(result.0));
        ctx.pc += 1;
        ctx
    }
}

impl OpcodeFn for OpSub {
    fn gas_cost(&self) -> u128 { 3 }

    fn exec(&self, mut ctx: Context) -> Context {
        let result = U256::from(ctx.stack.pop().unwrap()).overflowing_sub(U256::from(ctx.stack.pop().unwrap()));
        ctx.stack.push(Word::from(result.0));
        ctx.pc += 1;
        ctx
    }
}

impl OpcodeFn for OpDiv {
    fn gas_cost(&self) -> u128 { 5 }

    fn exec(&self, mut ctx: Context) -> Context {
        let a = U256::from(ctx.stack.pop().unwrap());
        let b = U256::from(ctx.stack.pop().unwrap());
        if b.is_zero() {
            ctx.stack.push(Word::from(U256::from(0)))
        } else {
            let result = a / b;
            ctx.stack.push(Word::from(result));
        }
        ctx.pc += 1;
        ctx
    }
}

impl OpcodeFn for OpSDiv {
    fn gas_cost(&self) -> u128 { 5 }

    fn exec(&self, mut ctx: Context) -> Context {
        let a = U256::from(ctx.stack.pop().unwrap());
        let b = U256::from(ctx.stack.pop().unwrap());
        if b.is_zero() {
            ctx.stack.push(Word::from(U256::from(0)))
        } else {
            if a.is_negative() ^ b.is_negative() {
                ctx.stack.push(Word::from((a.abs() / b.abs()).to_negative()));
            } else {
                ctx.stack.push(Word::from(a / b));
            }
        }
        ctx.pc += 1;
        ctx
    }
}

impl OpcodeFn for OpInvalid {
    fn gas_cost(&self) -> u128 { 0 }

    fn exec(&self, mut ctx: Context) -> Context {
        ctx.pc = ctx.codes.len();
        ctx
    }
}

pub fn decode_op(opcode: u8) -> Box<dyn OpcodeFn> {
    match opcode {
        0x01 => Box::new(OpAdd),
        0x02 => Box::new(OpMul),
        0x03 => Box::new(OpSub),
        0x04 => Box::new(OpDiv),
        0x05 => Box::new(OpSDiv),
        0x60 => Box::new(OpPush1),
        0x61 => Box::new(OpPush2),
        0x62 => Box::new(OpPush3),
        0x63 => Box::new(OpPush4),
        0x64 => Box::new(OpPush5),
        0x65 => Box::new(OpPush6),
        0x66 => Box::new(OpPush7),
        0x67 => Box::new(OpPush8),
        0x68 => Box::new(OpPush9),
        0x69 => Box::new(OpPush10),
        0x6a => Box::new(OpPush11),
        0x6b => Box::new(OpPush12),
        0x6c => Box::new(OpPush13),
        0x6d => Box::new(OpPush14),
        0x6e => Box::new(OpPush15),
        0x6f => Box::new(OpPush16),
        0x70 => Box::new(OpPush17),
        0x71 => Box::new(OpPush18),
        0x72 => Box::new(OpPush19),
        0x73 => Box::new(OpPush20),
        0x74 => Box::new(OpPush21),
        0x75 => Box::new(OpPush22),
        0x76 => Box::new(OpPush23),
        0x77 => Box::new(OpPush24),
        0x78 => Box::new(OpPush25),
        0x79 => Box::new(OpPush26),
        0x7a => Box::new(OpPush27),
        0x7b => Box::new(OpPush28),
        0x7c => Box::new(OpPush29),
        0x7d => Box::new(OpPush30),
        0x7e => Box::new(OpPush31),
        0x7f => Box::new(OpPush32),
        _ => Box::new(OpInvalid),
    }
}