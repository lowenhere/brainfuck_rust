// Brainfuck has 8 basic instructions:
// > - move the pointer right
// < - move the pointer left
// + - increment the current cell
// - - decrement the current cell
// . - output the value of the current cell
// , - replace the value of the current cell with input
// [ - jump to the matching ] instruction if the current value is zero
// ] - jump to the matching [ instruction if the current value is not zero

pub const MOVE_RIGHT: u8 = b'>';
pub const MOVE_LEFT: u8 = b'<';
pub const INCREMENT: u8 = b'+';
pub const DECREMENT: u8 = b'-';
pub const LOOP_START: u8 = b'[';
pub const LOOP_END: u8 = b']';
pub const INPUT: u8 = b',';
pub const OUTPUT: u8 = b'.';
