const FONTSET: [u8; 80] = [ 
	0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
	0x20, 0x60, 0x20, 0x20, 0x70, // 1
	0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
	0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
	0x90, 0x90, 0xF0, 0x10, 0x10, // 4
	0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
	0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
	0xF0, 0x10, 0x20, 0x40, 0x40, // 7
	0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
	0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
	0xF0, 0x90, 0xF0, 0x90, 0x90, // A
	0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
	0xF0, 0x80, 0x80, 0x80, 0xF0, // C
	0xE0, 0x90, 0x90, 0x90, 0xE0, // D
	0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
	0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

pub struct Chip8 {
    // src: https://www.cs.columbia.edu/~sedwards/classes/2016/4840-spring/designs/Chip8.pdf
    opcode: u16,
    memory: [u8; 4096],
    v: [u8; 16],
    i: u16,
    pc: u16,
    gfx: [u8; 64*32],
    sound_buffer: u8,
    delay_buffer: u8,
    stack: [u16; 16],
    sp: u8,
    keys: [u8; 16],
    clr: bool
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            opcode: 0x0000,
            memory: [0; 4096],
            v: [0; 16],
            i: 0,
            pc: 0,
            gfx: [0; 64*32],
            sound_buffer: 0,
            delay_buffer: 0,
            stack: [0; 16],
            sp: 0,
            keys: [0; 16],
            clr: true
        }
    }

    pub fn emulate_cycle(&mut self) {
        match self.opcode & 0xF000 {
            // TODO: implement the state machine for opcode
            0x0000 => {
                match self.opcode & 0x000F {
                    0x0000 => op_00e0(),
                    0x000E => op_00ee(),
                    0x0004 => op_8xy4(&self.opcode, &mut self.v, &mut self.pc),
                    _ => ()
                }
            },
            0x2000 => op_2nnn(&self.opcode, &mut self.stack, &mut self.sp, &mut self.pc),
            _ => ()
        }
    }
}

impl Default for Chip8 {
    fn default() -> Self {
        Self::new()
    }
}

// Register's opcode functions
// src: https://chip8.gulrak.net/
fn op_0nnn() {}

fn op_00e0() {}

fn op_00ee() {}

fn op_1nnn() {}

fn op_2nnn(opcode: &u16, stack: &mut [u16; 16], sp: &mut u8, pc: &mut u16) {
    let idx = *sp as usize;
    stack[idx] = *pc;
    *sp += 1;
    *pc = opcode & 0x0FFF
}

fn op_3xnn() {}

fn op_4xnn() {}

fn op_5xy0() {}

fn op_6xnn(opcode: &u16, v: &mut [u8; 16]) {}

fn op_7xnn(opcode: &u16, v: &mut [u8; 16]) {}

fn op_8xy0() {}

fn op_8xy1() {}

fn op_8xy2() {}

fn op_8xy3() {}

fn op_8xy4(opcode: &u16, v: &mut [u8; 16], pc: &mut u16) {
    let idx0 = opcode & 0x00F0 >> 4;
    let idx1 = opcode & 0x0F00 >> 8;

    if v[idx0 as usize] > v[idx1 as usize] {
        
    } else {

    }
}

fn op_8xy5() {}

fn op_8xy6() {}

fn op_8xy7() {}

fn op_8xye() {}

fn op_9xy0() {}

fn op_annn(opcode: &u16, i: &mut u16, pc: &mut u16) {}

fn op_bnnn() {}

fn op_cxnn() {}

fn op_dxyn() {}

fn op_ex9e() {}

fn op_exa1() {}

fn op_fx07() {}

fn op_fx0a() {}

fn op_fx15() {}

fn op_fx18() {}

fn op_fx1e() {}

fn op_fx29() {}

fn op_fx33() {}

fn op_fx55() {}

fn op_fx65() {}
