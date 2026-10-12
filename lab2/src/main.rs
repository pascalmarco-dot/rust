
fn add_chars_n (mut s : String, c : char, n : i32) -> String
{
    for _ in 0..n
    {
        s.push(c)
    }
    s
}
fn main() {
    let mut s = String::from("");
    let mut i = 0;
    while i < 26 {
        let c = (i as u8 + b'a') as char;
        s = add_chars_n(s, c, 26 - i);

        i += 1;
    }
    print!("{}", s);
}

fn add_chars_n (s : &mut String, c : char, n : i32)
{
    for _ in 0..n 
    {
        s.push(c)
    }
}
fn main() {
    let mut s = String::from("");
    let mut i = 0;
    while i < 26 {
        let c = (i as u8 + b'a') as char;
        add_chars_n(&mut s, c, 26 - i);

        i += 1;
    }
    print!("{}", s);
}
fn add_space(s: &mut String, n: u32) {
    for _ in 1..=n {
        s.push(' ')
    }
}

fn add_str(s: &mut String, text: &str) {
    s.push_str(text)
}

fn add_integer(s: &mut String, n: i32) {
    let mut s2 = String::new();
    let mut x = n.abs();
    if x == 0 {
        s2.push('0')
    }
    while x != 0 {
        let c = ((x%10) as u8 + b'0') as char;
        s2.push(c);
        x /= 10;
    }
    let mut s3 = String::new();
    if n < 0
    {
        s3.push('-')
    }
    let sz = s2.len();
    let mut remaining = sz;
    let s2_bytes = s2.as_bytes();
    for i in 0..sz {
        s3.push(s2_bytes[sz - i - 1] as char);
        remaining -= 1;
        if remaining%3 == 0 && remaining > 0 {
            s3.push('_')
        }
    }
    s.push_str(&s3);
}

fn add_float(s: &mut String, n: f32) {
    let  _s2 = String::new();
    let  int_part = n.trunc() as i32;
    let mut fract_part = n.fract().abs();
    let  _x = fract_part;
    add_integer(s, int_part);
    if fract_part > 0.0 {
        s.push('.');
        for _ in 0..=3{
            fract_part *= 10.0;
            let c = fract_part.trunc() as u8;
            s.push((c + b'0') as char);
            fract_part = fract_part.fract();
            if fract_part < 0.000001 {
                break;
            }
        }
    }
}
fn main()
{
    let mut s = String::new();
    add_space(&mut s, 40);
    add_str(&mut s, "I");
    add_space(&mut s, 1);
    add_str(&mut s, "💚\n");
    add_space(&mut s, 40);
    add_str(&mut s, "RUST.\n\n");
 
    add_space(&mut s, 4);
    add_str(&mut s, "Most");
    add_space(&mut s, 9);
    add_str(&mut s, "crate");
    add_space(&mut s, 6);
    add_integer(&mut s, 306437968);
    add_space(&mut s, 11);
    add_str(&mut s, "and");
    add_space(&mut s, 5);
    add_str(&mut s, "lastest");
    add_space(&mut s, 9);
    add_str(&mut s, "is\n");
 
    add_space(&mut s, 9);
    add_str(&mut s, "downloaded");
    add_space(&mut s, 7);
    add_str(&mut s, "has");
    add_space(&mut s, 12);
    add_str(&mut s, "downloads");
    add_space(&mut s, 5);
    add_str(&mut s, "the");
    add_space(&mut s, 7);
    add_str(&mut s, "version");
    add_space(&mut s, 4);
    add_float(&mut s, 2.038);
    add_str(&mut s, ".");
 
    println!("{}", s);
}