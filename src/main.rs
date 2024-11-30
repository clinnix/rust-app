
fn main(){
 
 let a1: i32 = -125;
 let a2: i32 = 0xFF;
 //八进制
 let a3: i32 = 0o13;
 //二进制
 let a4: i32 = 0b10;

 println!("{a1}, {a2}, {a3}, {a4}");

 println!("u32 max: {}", u32::MAX);
 println!("u32 min: {}", u32::MIN);
 println!("i32 max: {}", i32::MAX);
 println!("i32 min: {}", i32::MIN);
 println!("usize max: {}", usize::MAX);
 println!("isize is {} bytes", std::mem::size_of::<isize>());
 println!("usize is {} bytes", std::mem::size_of::<usize>());
 println!("u64 is {} bytes", std::mem::size_of::<u64>());
 println!("i64 is {} bytes", std::mem::size_of::<i64>());
 println!("i32 is {} bytes", std::mem::size_of::<i32>());

 let f1: f32 = 1.2323;
 let f2: f64 = 9.8888;
 println!("Fload are {:.2} {:.2}", f1, f2);

 let is_ok: bool = true;
 let can_ok: bool = false;
 println!("is ok ? {is_ok} {can_ok}");

 let char_c: char = 'c';
 let emo_char: char = '☺';
 println!("you GET {char_c} {emo_char}")
   
}

