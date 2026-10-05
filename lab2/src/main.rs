fn cmmdc(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    return a;
}

fn prime(a: i32) -> bool {
    if a < 2 {
        return false;
    }
    if a % 2 == 0 && a != 2 {
        return false;
    }
    let mut d = 3;
    while d * d <= a {
        if a % d == 0 {
            return false;
        }
        d += 2;
    }
    return true;
}

fn bottle() {
    for i in (3..=99).rev() {
        println!("{i} bottles of beer on the wall, ");
        println!("{i} bottles of beer, ");
        println!("Take one down, pass it around, ");
        println!("{} bottles of beer on the wall", i - 1);
    }
    println!("1 bottle of beer on the wall,");
    println!("1 bottle of beer.");
    println!("Take one down, pass it around,");
    println!("No bottles of beer on the wall.");
}

fn main() {
    //pb1
    let mut i = 0;
    while i <= 100 {
        if prime(i) == true {
            println!("{} ", i);
        }
        i += 1;
    }
    //pb2
    let mut i = 0;
    while i < 100 {
        let mut j = i + 1;
        while j <= 100 {
            if cmmdc(i, j) == 1 {
                println!("{} {}\n", i, j);
            }
            j += 1;
        }
        i += 1;
    }

    //pb3
    bottle();
}
