extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let seclet_number = rand::thread_rng().gen_range(1,101);

    loop{
        println!("Hello, Guess number");
        let mut guessed_number = String::new();                     // mutableなので，可変であることを示す.デフォルトではimmutable.
        io::stdin().read_line(&mut guessed_number)                  // &でアドレスを渡しているかつ，デフォルトではimmutableなのでmutを明示.
            .expect("faild");
        let guessed_number: u32 = match guessed_number.trim().parse() {   // rust のshadowという機能で，変数の再利用が可能.
            Ok(num) => num,
            Err(_) => continue,
        };
    
        match guessed_number.cmp(&seclet_number){                   // すんごいhaskellを思い出した.
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

//cargo doc --openを使用することで，現状の依存状態に応じて専用のdocumentが生成される.
//例外処理が.expectで書けるのは気持ちが良い.
//概念がすごく関数型のプログラミング言語に似ているきがするので，プログラミング初学者がrustを始めるのは良い意味でも悪い意味でも大変そう.