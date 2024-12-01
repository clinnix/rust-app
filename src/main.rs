
fn main(){
  
  let a = String::from("hello");
  let len = first_word_2(&a);
  println!("{}",len);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    
    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word_2(s: &String) -> &str {
   let bytes = s.as_bytes();
   for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
   }

   &s[..]
}
