fn main()
{
    let s1 = String::from("hello");
    let s2 = s1;

    //println!("{}, world!", s1); if it was an integer instead of string it would have worked
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}
