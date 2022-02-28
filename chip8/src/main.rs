struct CPU {
    registers: [u8; 16],
    position_in_memory: usize,
    memory: [u8; 0x1000],
    stack: [u16; 16],
    stack_pointer: usize,
}

impl CPU {
    fn read_opcode(&self) -> u16 {
        let p = self.position_in_memory;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;

        op_byte1 << 8 | op_byte2
    }

    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            self.position_in_memory += 2;

            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = ((opcode & 0x000F) >> 0) as u8;

            // println!("{:?}, {:?}, {:?}, {:?}", &c, &x, &y, &d);

            let nnn = opcode & 0x0FFF;

            match (c, x, y, d) {
                (0, 0, 0, 0) => {
                    return;
                }
                (0, 0, 0xE, 0xE) => self.ret(),
                (0x2, _, _, _) => self.call(nnn),
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                _ => todo!("opcode {:04x}", opcode),
            }
        }
    }

    fn call(&mut self, addr: u16) {
        let sp = self.stack_pointer;
        let stack = &mut self.stack;

        if sp > stack.len() {
            panic!("Stack overflow!")
        }

        stack[sp] = self.position_in_memory as u16;
        self.stack_pointer += 1;
        self.position_in_memory = addr as usize;
    }

    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack underflow!")
        }

        self.stack_pointer -= 1;
        let addr = self.stack[self.stack_pointer];
        self.position_in_memory = addr as usize;
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (val, overflow) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = val;

        if overflow {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }
}

fn main() {
    // let mut memory: [u8; 4096] = [0; 4096];
    // let mem = &mut memory;

    // let add_twice = [
    //     0x80, 0x14,
    //     0x80, 0x14,
    //     0x00, 0xEE,
    // ];

    // mem[0x100..0x106].copy_from_slice(&add_twice);

    // mem[0x100] = 0x80; mem[0x101] = 0x14;
    // mem[0x102] = 0x80; mem[0x103] = 0x14;
    // mem[0x104] = 0x00; mem[0x105] = 0xEE;

    // println!("{:?}", &mem[0x100..0x106]);

    let mut cpu = CPU {
        registers: [0; 16],
        memory: [0; 4096],
        position_in_memory: 0,
        stack: [0; 16],
        stack_pointer: 0,
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    // cpu.registers[2] = 10;
    // cpu.registers[3] = 10;

    let mem = &mut cpu.memory;
    mem[0x000] = 0x21;
    mem[0x001] = 0x00;
    mem[0x002] = 0x21;
    mem[0x003] = 0x00;
    mem[0x004] = 0x00;
    mem[0x005] = 0x00;

    mem[0x100] = 0x80;
    mem[0x101] = 0x14;
    mem[0x102] = 0x80;
    mem[0x103] = 0x14;
    mem[0x104] = 0x00;
    mem[0x105] = 0xEE;

    // mem[0] = 0x80;
    // mem[1] = 0x14;
    // mem[2] = 0x80;
    // mem[3] = 0x24;
    // mem[4] = 0x80;
    // mem[5] = 0x34;

    cpu.run();

    assert_eq!(cpu.registers[0], 45);
    println!("5 + (10 * 2)+ (10 * 2) = {}", cpu.registers[0]);

    // let opcode: u16 = 0x71E4;

    // let c = (opcode & 0xF000) >> 12;
    // let x = (opcode & 0x0F00) >> 8;
    // let y = (opcode & 0x00F0) >> 4;
    // let d = (opcode & 0x000F) >> 0;

    // assert_eq!(c, 0x7);
    // assert_eq!(x, 0x1);
    // assert_eq!(y, 0xE);
    // assert_eq!(d, 0x4);

    // let nnn = opcode & 0x0FFF;
    // let kk = opcode & 0x00FF;

    // assert_eq!(nnn, 0x1E4);
    // assert_eq!(kk, 0xE4);
}
