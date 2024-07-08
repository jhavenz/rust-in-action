// The CHIP-8 CPU is a simple, interpreted programming language which was used
// in the 1970s and 1980s on the COSMAC VIP and Telmac 1800 8-bit microcomputers.
// It was made by Joseph Weisbecker. The CHIP-8 language has 35 opcodes which are
// all two bytes long. It also contained games programmed by the world's first
// female video game designer, Joyce Weisbecker.

// The CHIP-8 has 4KB memory, 16 8-bit registers, and a 16-bit index register.

// Different versions of the CHIP-8 will be implemented:
// - CHIP-8 'adder'. This version only implements the addition operation with 2 8-bit registers and space a single opcode (operation).

/**
CPU RIA/1 can execute a single instruction: addition.
*/
pub mod adder;

/**
CPU RIA/2, the multiplier, can execute several instructions in sequence.
The Multiplier includes RAM, a working main loop, and a variable that
indicates which instruction to execute next, which we’ll call `position_in_memory`.

Additions from CPU RIA/1:
- RAM: 4KB
- Fully-fledged main loop with stopping conditions.
  At each step in the loop, memory at `position_in_memory` (more commonly known as: 'program counter')
  is accessed and decoded into an opcode. `position_in_memory` is then
  incremented to the next memory address, and the opcode is executed. The
  CPU runs forever until a stopping condition is encountered (an opcode of 0x0000).
- Removes the `current_operation` field from the CPU struct, which is
  replaced by a section of the main loop that decodes bytes from memory.
- Writes the opcodes into memory.
*/
pub mod multiplier;

/**
CPU RIA/3, the caller,
This section adds the ability to call functions.
There is no programming language support, however, so any programs still need to be written in binary.
In addition to implementing functions, this section validates an assertion made... that functions are also data.
Additions from CPU RIA/2:
- Call stack
  Adds the 'CALL' opcode, which sets the `position_in_memory` to the address of a function.
  Adds the 'RETURN' opcode, which sets `position_in_memory` to the address of the previous CALL opcode.
  To enable these to opcodes to work together, the CPU needs to have some specialized memory available
  for storing addresses. This is known as the stack. Each CALL opcode adds an address to the stack by
  incrementing the stack pointer and writing `nnn` to that position in the stack. Each RETURN opcode
  removes the top address by decrementing the stack pointer.
- Functions
  Within computer science, a function is just a sequence of bytes that can be executed by a CPU.
  CPUs start at the first opcode, then make their way to the end. The next few code snippets
  demonstrate how it is possible to move from a sequence of bytes, then convert that into
  executable code.
  1. Define the function:
     Our function performs two addition operations and then returns. It is three opcodes long.
     The function’s internals look like this in a notation that resembles assembly language:
     ```
     add_twice:
         0x8014
         0x8014
         0x00EE
     ```
  2. Convert opcodes into Rust data types:
     Translating these three opcodes into Rust’s array syntax involves wrapping them in square
     brackets and using a comma for each number. The function has now become a [u16;3]: We want
     to be able to deal with one byte in the next step, so we’ll decompose the [u16;3] array
     further into a [u8;6] array:
     ```
         let add_twice: [u16;3] = [
          0x8014,
          0x8014,
          0x00EE,
        ];

        let add_twice: [u8;6] = [
          0x80, 0x14,
          0x80, 0x14,
          0x00, 0xEE,
        ];
     ```

  3. Load the function into RAM:
     Assuming that we wish to load that function into memory address 0x100, here are two options.
     First, if we have our function available as a slice, we can copy it across to memory with the
     copy_from_slice() method:
        @see [caller::load_function_into_memory()]
        -> Prints [128, 20, 128, 20, 0, 238]

     An alternative approach that achieves the same effect within memory without requiring a temporary
     array is to overwrite bytes directly:
        @see [caller::load_function_into_memory_directly()]
        -> Prints [128, 20, 128, 20, 0, 238]
*/
pub mod caller;
