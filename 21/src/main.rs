// numeric addresses
fn main() {
    // hello world in f32
    let address = [1.1431391e27, 6.6578487e28, 7.739293e-19, 1.4e-44, 9.5946e-41, 1.04704e-40, 9.5982e-41, 1.05005e-40, 1.05219e-40, 9.5946e-41, 9.5946e-41, 9.5946e-41];
    let mut address_u32: Vec<u32> = Vec::new();
    for x in &address {
        unsafe { 
            let words_f32: &u32 = std::mem::transmute::<&f32, &u32>(x);
            address_u32.push(*words_f32);
        }
    }
    unsafe {
        // let fin_words = 

    }
    println!("{:?}", address_u32);
}

