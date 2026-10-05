fn cmmdc (mut a: i32, mut b:i32) -> i32
{
    while b!=0
    {
        let r=a%b;
        a=b;
        b=r;
    }
    return a;
}

fn prime (mut a: i32) -> bool
{
    if a<2
    {
        return false;
    }
    if a%2==0 && a!=2
    {
        return false;
    }
    let mut d= 3;
    while d*d<=a
    {
        if a%d==0
        {
            return false;
        }
        d+=2;
    }
    return true;
}

fn main() {
    let mut i=0;
    while i<=100
    {
        if prime(i)==true
        {
            println!("{} ",i);
        }
        i+=1;
    }
}
