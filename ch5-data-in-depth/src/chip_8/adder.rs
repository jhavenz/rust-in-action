
/**
Steps:
    1. Initialize a CPU
    2. Load `u8` values into registers
    3. Load the addition opcode into `current_operation`
    4. Perform the addition operation
*/

struct CPU {
    current_operation: u16,
    registers: [u8; 2],
}

impl CPU {
    fn read_opcode(&self) -> u16 {
        self.current_operation
    }

    /**
        The Emulator's main loop.
        Calling 'run' will emulate the CPU cycles:
        1. Read the opcode (eventually, from memory)
        2. Decode the instruction
        3. Match decoded instruction to known opcodes
        4. Dispatch execution of the operation to a specific function
     */
    fn run(&mut self) {
        // loop {
            let opcode = self.read_opcode();

            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >>  8) as u8;
            let y = ((opcode & 0x00F0) >>  4) as u8;
            let d = ((opcode & 0x000F) >>  0) as u8;

            let nnn = opcode & 0x0FFF;
            let kk = opcode & 0x00FF;

            match (c, x, y, d) {
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                _ => todo!("opcode {:04x}", opcode)
            }
        // }
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        self.registers[x as usize] += self.registers[y as usize];
    }
}

pub fn execute() {
    let mut cpu = CPU {
        current_operation: 0,
        registers: [0; 2],
    };

    /**
       8: signifies that the operation involves two registers
       0: maps to the first register (cpu.registers[0])
       1: maps to the second register (cpu.registers[1])
       4: signifies that the operation is addition

       Terms used to refer to parts of the CHIP-8 opcodes:
       Using 0x73EE as an example:
       [73] (u8) referred to as the 'High byte'
       [7] (u4) referred to as the 'High nibble'
       [3] (u4) referred to as the 'Low nibble'
       [EE] (u8) referred to as the 'Low byte'
       [E] (u4) referred to as the 'High nibble'
       [E] (u4) referred to as the 'Low nibble'

       Documentation manuals for the CHIP-8 introduce several variables,
       including kk, nnn, x, and y.

       The decoding process involves matching on
       the high nibble of the first byte and then
       applying one of three strategies:

       (1) 0x73EE:
       [7] is the 'Opcode group'
       [3] is the 'Register x'
       [EE] act as arguments to the operation
       Interpretation:
       -> add 238 (0xEE) to register 3

       (2) 0x1200
       [1] is the 'Opcode group'
       [200] is the 'Address'
       Interpretation:
       -> jump to memory address (0x200)

       (3) 0x8231
       [8] is the 'Opcode group'
       [2] is the 'Register x'
       [3] is the 'Register y'
       [1] is the 'Opcode subtype'
       Interpretation:
       -> perform a bitwise OR operation with registers x and y. Store the result in register x
    */
    let op_code = 0x8014;

    cpu.current_operation = op_code;
    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    cpu.run();

    assert_eq!(cpu.registers[0], 15);

    println!("5 + 10 = {}", cpu.registers[0]);
}
