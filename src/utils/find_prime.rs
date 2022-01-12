use gloo::console::log;
pub fn isprime(number: u32) -> bool {
    if number == 1 {
        return false;
    }
    let num: f32 = number as f32;
    let square_root = num.sqrt() as i32;
    log!("Squarenumber", square_root);
    for x in 2..square_root + 1 {
        if number % x as u32 == 0 {
            return false;
        }
    }
    true
}
