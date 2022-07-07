
fn fizz_buzz8() {
    let fz = |x: i32| {
        match (x % 3, x % 5) {
            (0, 0) => format!("FizzBuzz"),
            (0, _) => format!("Fizz"),
            (_, 0) => format!("Buzz"),
                _  => x.to_string(),
        }
    };

    (1 ..= 100).map(fz).for_each(|x| println!("{}", x));
}

fn main() {
    fizz_buzz8();
}


/*   //所有権の話２ &をつけて参照の機能を利用する
fn main() {
    fn test_print(msg: &String) -> &String {
        println!("{} in test_print", msg);
        &msg
    }

    let s1 = "Rust Life".to_string();

    let s2 = test_print(&s1);

    println!("{} after test_print /s2", s2);
    println!("{} after test_print /s1", s1); // コンパイルエラー：所有権は`s1`にはない。←　＆をつければコンパイル可 
}
*/

/* 所有権の話１ 
fn main() {
    let s1 = "Rust Lif".to_string();
    println!("{}", s1); //OK

    let s2 = s1; // ムーブセマンティクス：所有権が`s1`から`s2`に移動している

    println!("{}", s2); // OK
    println!("{}", s1); // コンパイルエラー：所有権は`s2`に移動しているので`s1`にアクセス不可 
}
*/

/*
fn fizz_buzz7() {
    for x in 1 ..= 100 {
        let tmp;
        let s = match (x % 3, x % 5) {
            (0, 0) => "FizzBuzz",
            (0, _) => "Fizz",
            (_, 0) => "Buzz",
            _      => {tmp = x.to_string(); &tmp}, 
        };
        println!("{}", s);
    }
}

fn main() {
    fizz_buzz7();
}
*/

/*
fn fizz_buzz6() {
    for x in 1 ..= 100 {
        let s = match (x % 3, x % 5) {
            (0, 0) => "FizzBuzz".to_string(),
            (0, _) => "Fizz".to_string(),
            (_, 0) => "Buzz".to_string(),
            _      => x.to_string(),
        };
        println!("{}", s);
    }
}

fn main() {
    fizz_buzz6();
}
*/

/* パターンガードにタプルを組み合わせる
fn fizz_buzz5() {
    for x in 1 ..= 100 {
        match (x % 3, x % 5) {
            (0, 0) => println!("FizzBuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            _      => println!("{}", x),
        }
    }
}

fn main() {
    fizz_buzz5();
}
*/

/* パターンガード
fn fizz_buzz4() {
    for x in 1 ..= 100 {
        match x {
            e if e % 15 == 0 => println!("FizzBuzz"),
            e if e %  3 == 0 => println!("Fizz"),
            e if e %  5 == 0 => println!("Buzz"),
            e => println!("{}", e),
        }
    }
}

fn main() {
    fizz_buzz4();
}
*/

/*
fn fizz_buzz3() {
    for x in 1 ..= 100 {
        match x % 15 {
            0 => println!("FizzBuzz"),
            3 | 6 | 9 | 12 => println!("Fizz"),
            5 | 10 => println!("Buzz"),
            _ => println!("{}", x),
        }
    }
}

fn main() {
    fizz_buzz3();
}
*/

/*
fn main() {
    for x in 1 .. 101 {
        if x % 15 == 0 {
            println!("FizzBuzz");
        } else if x % 3 == 0 {
            println!("Fizz");
        } else if x % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", x);
        }
    }
}
*/

/* 
fn main() {
    let mut x = 1;

    while x <= 100 {
        if x % 15 == 0 {
            println!("FizzBuzz");
        } else if x % 3 == 0 {
            println!("Fizz");
        } else if x % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", x);
        }
        x += 1;
    }
}
*/


