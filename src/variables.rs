pub fn variables() {
    let patata = "patata";
    println!("i eat {patata}");

    let mut x = 5;
    println!("the value of x is -> {x}");
    x = 10;
    println!("the value of x is -> {x}");

    //constantes
    const THREE_HOURS_IN_SECONDS: i32 = 60 * 60 * 3;
    println!("three hours in seconds: {}", THREE_HOURS_IN_SECONDS);

    //shadowing
    let x = x + 1;
    {
        let x = x * 2;
        println!("the value of x in the inner scope is -> {x}")
    }

    println!("the value of x is: {x}");

    //let mut spaces = " "; -> error
    let spaces = " ";
    let spaces = spaces.len();

    println!("{spaces}");

}
