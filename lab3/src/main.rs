#[derive(Debug)]
enum Errors{
    OverflowAdunare,
    OverflowInmultire,
    NotAscii,
    NotDigit,
    NotB16,
    NotLetter,
    NotPrintable,
}

fn isprim(x: u16) -> bool {
    if x <= 1 {
        return false;
    }
    
    let mut d = 2;
    while d <= x / d {
        if x.is_multiple_of(d) {
            return false;
        }
        d += 1;
    }
    
    true
}
fn next_prime(x: u16) -> Option<u16> {
    if x == 65535 {
        return None;
    }
    let mut i = x + 1;
    loop {
        if i < 65535 {
            if isprim(i) {
                return Some(i);
            } else {
                i += 1;
            }
        } else {
            return None;
        }
    }
}
fn addition (x : u32 , y : u32) -> u32
{
    let z= x as u64 + y as u64;
    if z>4294967295{
        panic!("depasit limita adunare");
    }
    else{
         z as u32
    }
}
fn multiplication (x : u32 , y : u32) -> u32
{
    let z = x as u64 * y as u64;
    if z>4294967295{
        panic!("depasit limita inmultire");
    }
    else{
        z as u32
    }
}
fn addit (x : u32, y : u32) -> Result <u32 , Errors>
{
    let z = x as u64 + y as u64;
    if z>4294967295{
        Err(Errors::OverflowAdunare)
    }
    else{
        Ok(z as u32)
    }
}
fn mult (x : u32 , y : u32) -> Result <u32, Errors>
{
    let z = x as u64 * y as u64;
    if z>4294967295{
        Err(Errors::OverflowInmultire)
    }
    else{
        Ok(z as u32)
    }
}
fn operatie_combinata (x : u32, y : u32, z : u32) ->Result<u32,Errors>
{
    let suma = addit(x,y)?;
    let rez = mult(suma,z)?;
    Ok(rez)
}
fn to_uppercase (c : char) -> Result<char,Errors>
{
    if c.is_ascii_lowercase(){
        let caps = (c as u8 - 32) as char;
        Ok(caps)
    }
    else {
        Err(Errors::NotLetter)
    }
}
fn to_lowercase (c : char) -> Result<char,Errors>
{
    if c.is_ascii_uppercase() {
        let caps = (c as u8 + 32) as char;
        Ok(caps)
    }
    else {
        Err(Errors::NotLetter)
    }
}
fn print_char (c : char) -> Result<(),Errors>
{
    if c.is_ascii_control(){
        Err(Errors::NotPrintable)
    }
    else {
        println!("{}",c);
        Ok(())
    }
}
fn to_number (c : char) -> Result<u32,Errors>
{
    if !c.is_ascii()
    {
        return Err(Errors::NotAscii);
    }
    if c.is_ascii_digit()
    {
        let nmb = (c as u8 - b'0') as u32;
        Ok(nmb)
    }
    else{
        Err(Errors::NotDigit)
    }
}
fn char_to_number_hex(c: char) -> Result<u32, Errors> {
    if !c.is_ascii() {
        return Err(Errors::NotAscii);
    }
    if !c.is_ascii_hexdigit() {
        return Err(Errors::NotB16);
    }
    if c.is_ascii_digit() {
        return Ok((c as u8 - b'0') as u32);
    } 
    Ok((c as u8 - b'A' + 10_u8) as u32)
}
fn print_error (err : Errors)
{
    match err{
        Errors::OverflowAdunare => println!("adunarea da overflow"),
        Errors::OverflowInmultire => println!("inmultirea da overflow"),
        Errors::NotAscii => println!("caracterul nu este ascii"),
        Errors::NotDigit => println!("caracterul nu este cifra"),
        Errors::NotB16 => println!("caracterul nu este hex"),
        Errors::NotLetter => println!("caracterul nu este litera"),
        Errors::NotPrintable => println!("caracterul nu este printable"),
    }
}
fn calculeaza_cifra_control(n: u32) -> Option<u32> {
    if n == 0 {
        return None; 
    }
    
    if n.is_multiple_of(9) {
        Some(9)
    } else {
        Some(n % 9)
    }
}
fn main() {
     let mut curent = 0;
     while let Some(x) = next_prime(curent) {
        println!("{}", x);
        curent = x;
    }
    println!("{}",multiplication(4,8));
    println!("{}",addition(4,4));
    let testadd = addit(5,6);
    println!("{:?}",testadd);
    let testmult = mult(4000000000,2);
    println!("{:?}",testmult);
    let testcomb = operatie_combinata(4,5,6);
    println!("{:?}",testcomb);
    let caps = to_uppercase('c');
    println!("{:?}",caps);
    let lows = to_lowercase('U');
    println!("{:?}",lows);
    let _ = print_char('x');
    let ch = to_number('8');
    println!("{:?}",ch);
    let ch16 = char_to_number_hex('9');
    println!("{:?}",ch16);
    print_error(Errors::NotAscii);

    let test = calculeaza_cifra_control(12345);
    match test {
        Some(cifra) => println!("Cifra de control este: {}", cifra),
        None => println!("Eroare: Nu calculăm pentru 0."),
    }
}
