use std::io; //perlude
use rand::Rng; //trait 可以看成接口
use std::cmp::Ordering;

pub mod fun_test {
    
    pub fn add_to_waitlist(){
        println!("{}", "invoke add_to_waitlist ")
    }

    pub fn getGuessNumber(){
        println!("猜数游戏");

        let secret_number = rand::thread_rng().gen_range(1, 101);

        //   println!("神秘数字是：{}", secret_number);12

        loop {
            println!("猜测一个数字");
            let mut guess = String::new();

            io::stdin().read_line(&mut guess).expect("无法读取行");

            let guess:u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            println!("你猜测的数是：{}", guess);

            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small"),
                Ordering::Greater => println!("You big"),
                Ordering::Equal => {
                    println!("You win!");
                    break;
                }
            }
        }
    }


}