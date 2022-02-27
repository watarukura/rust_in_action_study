struct CPU {
    current_operation: u16,
    registers: [u8; 2],
}

impl CPU {
    fn read_opcode(&self) -> u16 {
        self.current_operation
    }

    fn run(&mut self) {
        let opcode = self.read_opcode();

        let c = ((opcode & 0xF000) >> 12) as u8;
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;
        let d = ((opcode & 0x000F) >> 0) as u8;

        // println!("{:?}, {:?}, {:?}, {:?}", &c, &x, &y, &d);

        match (c, x, y, d) {
            (0x8, _, _, 0x4) => self.add_xy(x, y),
            _ => todo!("opcode {:04x}", opcode),
        }
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        self.registers[x as usize] += self.registers[y as usize];
    }
}

fn main() {
    let mut cpu = CPU {
        current_operation: 0, // 0: no-op(何もしない)
        registers: [0; 2],
    };

    cpu.current_operation = 0x8014;
    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    cpu.run();

    assert_eq!(cpu.registers[0], 15);
    println!("5 + 10 = {}", cpu.registers[0]);

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
