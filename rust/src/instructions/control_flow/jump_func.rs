use crate::types::value::Value;

pub fn jump_func(ip: &mut usize, target_ip: usize, stack: &mut Vec<Value>) {
    // Logic truncate stack lo bisa ditaruh sini
    if stack.len() > 50 {
        stack.truncate(50);
    }
    *ip = target_ip;
}
