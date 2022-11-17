use std::collections::VecDeque;
mod symbols;

const VM_MEMORY_BLOCKS: usize = 30000;
pub struct VM {
    mem: [u8; VM_MEMORY_BLOCKS],
    mem_cursor: usize,
    program_buffer: Vec<u8>,
    program_cursor: usize,
    loop_stack: Vec<usize>,
    input_buffer: VecDeque<u8>,
    output_buffer: VecDeque<u8>,
}

impl VM {
    pub fn new() -> VM {
        VM {
            mem: [0_u8; VM_MEMORY_BLOCKS],
            mem_cursor: 0,
            program_buffer: Vec::new(),
            program_cursor: 0,
            loop_stack: Vec::new(),
            input_buffer: VecDeque::new(),
            output_buffer: VecDeque::new(),
        }
    }

    pub fn load_program(&mut self, p: &[u8]) {
        self.program_buffer.extend_from_slice(p)
    }

    pub fn input(&mut self, c: &u8) {
        self.input_buffer.push_back(*c)
    }

    pub fn output(&mut self) -> Option<u8> {
        self.output_buffer.pop_front()
    }

    pub fn step(&mut self) -> bool {
        let c: u8;

        match self.program_buffer.get(self.program_cursor) {
            Some(i) => c = *i,
            None => return false,
        }

        match c {
            symbols::MOVE_RIGHT => {
                self.mem_cursor = (self.mem_cursor + 1) % (VM_MEMORY_BLOCKS + 1);
                self.program_cursor += 1;
            }
            symbols::MOVE_LEFT => {
                if self.mem_cursor <= 0 {
                    self.mem_cursor = 30000;
                } else {
                    self.mem_cursor -= 1;
                }

                self.program_cursor += 1;
            }
            symbols::INCREMENT => {
                self.mem[self.mem_cursor] += 1;

                self.program_cursor += 1;
            }
            symbols::DECREMENT => {
                self.mem[self.mem_cursor] -= 1;

                self.program_cursor += 1;
            }
            symbols::LOOP_START => {
                self.loop_stack.push(self.program_cursor);

                self.program_cursor += 1;
            }
            symbols::LOOP_END => {
                if self.mem[self.mem_cursor] == 0 {
                    match self.loop_stack.pop() {
                        Some(_) => self.program_cursor += 1,
                        None => panic!("loop end instruction without corresponding loop start"),
                    }
                } else {
                    match self.loop_stack.last() {
                        Some(pc) => self.program_cursor = *pc,
                        None => panic!("loop end instruction without corresponding loop start"),
                    }
                }
            }
            symbols::INPUT => {
                match self.input_buffer.pop_front() {
                    Some(i) => self.mem[self.mem_cursor] = i,
                    None => panic!("input buffer empty while executing input instruction"),
                }

                self.program_cursor += 1;
            }
            symbols::OUTPUT => {
                self.output_buffer.push_back(self.mem[self.mem_cursor]);
                self.program_cursor += 1;
            }
            _ => {
                self.program_cursor += 1;
            }
        }

        true
    }
}
