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
    sound_timer: u8,
    delay_timer: u8,
    stack: [u16; 16],
    sp: u8,
    keys: [u8; 16],
    flag: bool
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
            sound_timer: 0,
            delay_timer: 0,
            stack: [0; 16],
            sp: 0,
            keys: [0; 16],
            flag: true
        }
    }

    pub fn emulate_cycle(&mut self) {
        match self.opcode & 0xF000 {
            0x0000 => {
                match self.opcode & 0x000F {
                    0x0000 => op_00e0(&mut self.gfx, &mut self.pc, &mut self.flag),
                    0x000E => op_00ee(&mut self.stack, &mut self.pc, &mut self.sp),
                    _ => ()
                }
            },
            0x1000 => op_1nnn(&self.opcode, &mut self.pc),
            0x2000 => op_2nnn(&self.opcode, &mut self.stack, &mut self.pc, &mut self.sp),
            0x3000 => op_3xnn(&self.opcode, &mut self.v, &mut self.pc),
            0x4000 => op_4xnn(&self.opcode, &mut self.v, &mut self.pc),
            0x5000 => op_5xy0(&self.opcode, &mut self.v, &mut self.pc),
            0x6000 => op_6xnn(&self.opcode, &mut self.v),
            0x7000 => op_7xnn(&self.opcode, &mut self.v),
            0x8000 => {
                match self.opcode & 0x000F {
                    0x0000 => op_8xy0(&self.opcode, &mut self.v, &mut self.pc),
                    0x0001 => op_8xy1(&self.opcode, &mut self.v, &mut self.pc),
                    0x0002 => op_8xy2(&self.opcode, &mut self.v, &mut self.pc),
                    0x0003 => op_8xy3(&self.opcode, &mut self.v, &mut self.pc),
                    0x0004 => op_8xy4(&self.opcode, &mut self.v, &mut self.pc),
                    0x0005 => op_8xy5(&self.opcode, &mut self.v, &mut self.pc),
                    0x0006 => op_8xy6(&self.opcode, &mut self.v, &mut self.pc),
                    0x0007 => op_8xy7(&self.opcode, &mut self.v, &mut self.pc),
                    0x000E => op_8xye(&self.opcode, &mut self.v, &mut self.pc),
                    _ => ()
                }
            },
            0x9000 => op_9xy0(&self.opcode, &mut self.v, &mut self.pc), 
            0xA000 => op_annn(&self.opcode, &mut self.i, &mut self.pc),
            0xB000 => op_bnnn(&self.opcode, &mut self.v, &mut self.pc),
            0xC000 => op_cxnn(&self.opcode, &mut self.v, &mut self.pc),
            0xD000 => op_dxyn(&self.opcode, &mut self.gfx, &mut self.memory, &mut self.v, &mut self.pc, &mut self.i, &mut self.flag),
            0xE000 => {
                match self.opcode & 0x00FF {
                    0x009E => op_ex9e(&self.opcode, &mut self.v, &self.keys, &mut self.pc),
                    0x00A1 => op_exa1(&self.opcode, &mut self.v, &self.keys, &mut self.pc),
                    _ => ()
                }
            },
            0xF000 => {
                match self.opcode & 0x00FF {
                    0x0007 => op_fx07(&self.opcode, &mut self.v, &mut self.delay_timer, &mut self.pc),
                    0x000A => op_fx0a(&self.opcode, &mut self.v, &self.keys, &mut self.pc),
                    0x0015 => op_fx15(&self.opcode, &mut self.v, &mut self.delay_timer, &mut self.pc),
                    0x0018 => op_fx18(&self.opcode, &mut self.v, &mut self.sound_timer, &mut self.pc),
                    0x001E => op_fx1e(&self.opcode, &mut self.v, &mut self.i, &mut self.pc),
                    0x0029 => op_fx29(&self.opcode, &mut self.v, &mut self.i, &mut self.pc),
                    0x0033 => op_fx33(&self.opcode, &mut self.v, &mut self.memory, &mut self.i, &mut self.pc),
                    0x0055 => op_fx55(&self.opcode, &mut self.v, &mut self.memory, &mut self.i, &mut self.pc),
                    0x0065 => op_fx55(&self.opcode, &mut self.v, &mut self.memory, &mut self.i, &mut self.pc),
                    _ => ()
                }
            },
            _ => ()
        }

        if self.delay_timer > 1 { self.delay_timer -= 1 };
        if self.sound_timer == 1 { self.sound_timer -= 1 };
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

fn op_00e0(gfx: &mut [u8; 64*32], pc: &mut u16, flag: &mut bool) {
    for p in gfx {
        *p = 0x0;
    }
    *flag = true;
    *pc += 2;
}

fn op_00ee(stack: &mut [u16; 16], pc: &mut u16, sp: &mut u8) {
    *sp -= 1;
    *pc = stack[*sp as usize];
    *pc += 2;
}

fn op_1nnn(opcode: &u16, pc: &mut u16) {
    *pc = opcode & 0x0FFF;
}

fn op_2nnn(opcode: &u16, stack: &mut [u16; 16], pc: &mut u16, sp: &mut u8) {
    let idx = *sp as usize;
    stack[idx] = *pc;
    *sp += 1;
    *pc = opcode & 0x0FFF
}

fn op_3xnn(opcode: &u16, v: &mut [u8; 16], pc: &mut u16) {
    let idx0 = opcode & 0x0F00 >> 8;
    let idx1 = opcode & 0x00FF;

    if v[idx0 as usize] == idx1 as u8 {
        *pc += 4;
    } else {
        *pc += 2;
    }
}

fn op_4xnn(opcode: &u16, v: &mut [u8; 16], pc: &mut u16) {
    let idx0 = opcode & 0x0F00 >> 8;
    let idx1 = opcode & 0x00FF;

    if v[idx0 as usize] != idx1 as u8 {
        *pc += 4;
    } else {
        *pc += 2;
    }
}

fn op_5xy0(opcode: &u16, v: &mut [u8; 16], pc: &mut u16) {
    let idx0 = opcode & 0x0F00 >> 8;
    let idx1 = opcode & 0x00F0 >> 4;

    if v[idx0 as usize] == v[idx1 as usize] {
        *pc += 4;
    } else {
        *pc += 2;
    }
}

fn op_6xnn(opcode: &u16, v: &mut [u8; 16]) {
    
}

fn op_7xnn(opcode: &u16, v: &mut [u8; 16]) {}

fn op_8xy0(opcode: &u16, v: &mut [u8; 16], pc: &mut u16) {}

fn op_8xy1(opcode: &u16, v: &mut [u8; 16], pc: &mut u16) {}

fn op_8xy2(opcode: &u16, v: &mut [u8; 16], pc: &mut u16) {}

fn op_8xy3(opcode: &u16, v: &mut [u8; 16], pc: &mut u16) {}

fn op_8xy4(opcode: &u16, v: &mut [u8; 16], pc: &mut u16) {
    let idx0 = opcode & 0x00F0 >> 4;
    let idx1 = opcode & 0x0F00 >> 8;

    if v[idx0 as usize] > v[idx1 as usize] {
        todo!();
    } else {
        todo!();
    }
}

fn op_8xy5(opcode: &u16, v: &mut [u8; 16], pc: &mut u16) {}

fn op_8xy6(opcode: &u16, v: &mut [u8; 16], pc: &mut u16) {}

fn op_8xy7(opcode: &u16, v: &mut [u8; 16], pc: &mut u16) {}

fn op_8xye(opcode: &u16, v: &mut [u8; 16], pc: &mut u16) {}

fn op_9xy0(opcode: &u16, v: &mut [u8; 16], pc: &mut u16) {}

fn op_annn(opcode: &u16, i: &mut u16, pc: &mut u16) {}

fn op_bnnn(opcode: &u16, v: &mut [u8; 16], pc: &mut u16) {}

fn op_cxnn(opcode: &u16, v: &mut [u8; 16], pc: &mut u16) {}

fn op_dxyn(opcode: &u16, gfx: &mut [u8; 64*32], memory: &mut [u8; 4096], v: &mut [u8; 16], pc: &mut u16, i: &mut u16, flag: &mut bool) {}

fn op_ex9e(opcode: &u16, v: &mut [u8; 16], keys: &[u8; 16], pc: &mut u16) {}

fn op_exa1(opcode: &u16, v: &mut [u8; 16], keys: &[u8; 16], pc: &mut u16) {}

fn op_fx07(opcode: &u16, v: &mut [u8; 16], delay: &mut u8, pc: &mut u16) {}

fn op_fx0a(opcode: &u16, v: &mut [u8; 16], keys: &[u8; 16], pc: &mut u16) {}

fn op_fx15(opcode: &u16, v: &mut [u8; 16], delay: &mut u8, pc: &mut u16) {}

fn op_fx18(opcode: &u16, v: &mut [u8; 16], sound: &mut u8, pc: &mut u16) {}

fn op_fx1e(opcode: &u16, v: &mut [u8; 16], i: &mut u16, pc: &mut u16) {}

fn op_fx29(opcode: &u16, v: &mut [u8; 16], i: &mut u16, pc: &mut u16) {}

fn op_fx33(opcode: &u16, v: &mut [u8; 16], memory: &mut [u8; 4096], i: &mut u16, pc: &mut u16) {}

fn op_fx55(opcode: &u16, v: &mut [u8; 16], memory: &mut [u8; 4096], i: &mut u16, pc: &mut u16) {}

fn op_fx65(opcode: &u16, v: &mut [u8; 16], memory: &mut [u8; 4096], i: &mut u16, pc: &mut u16) {}
