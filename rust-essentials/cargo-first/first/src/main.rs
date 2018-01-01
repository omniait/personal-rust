/// Simple "game"
// Global Costants
static GAME_NAME: &str = "Hello, World";
static MAX_HEALTH: i8 = 100;

fn main() 
{
    // Statically allocated variable without value
    let v: i32;

    println!("Welcome to the Game");

    println!("The Game you are playing is called {}.", GAME_NAME); 
    println!("You start with {} health points.", MAX_HEALTH);

    // Example of println! with formatting:
    // * println!( "Two written in binary is {:b}", 2);
    /*  Formatting inside print
        o: For octal
        x: For lower hexadecimal
        X: For upper hexadecimal
        p: For a pointer
        b: For binary
        e: For lower exponential notation
        E: For upper exponential notation
        ?: For debugging purposes

        Referencing the same Variables inside print
    // * println!("In the Game {0} you start with {1} % health, yes you read it correctly: {1} points!", GAME_NAME, MAX_HEALTH); 
    */

    /*
    // Types
    Numerical Types:
    (Signed) Integers
        i8 	
        i16 	
        i32 	
        i64 	
        isize

    (Unsigned) Integers
        u8 	
        u16 	
        u32 	
        u64 	
        usize

    Floating Point
        f32 	
        f64 

    Experimental Types
        i128
        u128

    Arrays:
        array 
        tuple
        slice

    Boolean:
        bool

    Characters:
        char
        str

    Pointers:
        fn
        pointer 	

    // ? reference(unclear ?)
    */

    // Scope, shadowing
    let outer = 42; 
    { // start of code block 
        let inner = 3.14; 
        println!("block variable: {}", inner); 
        let outer = 99; // shadows the first outer variable 
        println!("block variable outer: {}", outer); 
    } // end of code block 
    println!("outer variable: {}", outer); 

    // String concatenation
    let player1 = "Rob"; 
    let player2 = "Jane"; 
    // ! Error let player3 = player1 + player2; 
    let player3 = player1.to_string() + " " + player2; 
    println!("{}", player3);
    let player3 = format!("{} {}", player1, player2); 
    println!("{}", player3);
    
    // Type Conversion
    let points = 10i32; 
    let mut saved_points: u32 = 0; 
    // ! Error saved_points = points;
    // Use this
    saved_points = points as u32;
    println!("{}", saved_points);
    let f = 67.2367; 
    saved_points = f as u32; // truncation 
    println!("{}", saved_points);
    
    // Aliasing
    type IAmALongVariableName = u16;
    let svn: IAmALongVariableName = 7800;
    println!("{}", svn);

    // Expressions
    let a = 2;    // a binds to 2 
    let b = 5;    // b binds to 5 
    let n = a + b;   // n binds to 7 
    println!("{}", n);
    let mut n = 0; 
    let mut m = 1; 
    let t = m; m = n; n = t; 
    println!("{} {} {}", n, m, t); // 1 0 1

    let n1 = 
    { 
        let a = 2; 
        let b = 5; 
        a + b   // * <-- no semicolon! 
    }; 
    println!("n1 is: {}", n1);  // prints: n1 is 7 
    let n2 = 
    { 
        let a = 2; 
        let b = 5; 
        a + b; 
    }; 
    println!("n2 is: {:?}", n2);  // prints: n2 is () 

    // Conditions
    let dead = false; 
    let health = 48; 
    if dead 
    { 
        println!("Game over!"); 
        return; 
    } 
    else 
    { 
        println!("You still have a chance to win!"); 
    } 
    if health >= 50 
    {
        println!("Continue to fight!"); 
    } 
    else if health >= 20  
    { 
        println!("Stop the battle and gain strength!"); 
    } 
    else 
    { 
        println!("Hide and try to recover!"); 
    } 
        // Expressive conditions
    let active = if health >= 50 { true } else { false }; 
    println!("Am I active? {}", active); 
    let adult = true; 
    let age = if adult { "+18" } else { "-18" }; 
    println!("Age is {}", age); 



    // Looping
    let max_power = 10; 
    let mut power = 1; 
    
    while power < max_power 
    { 
        print!("{} ", power); // prints without newline 
        power += 1;           // increment counter 
    } 
        // infinite loop
    loop 
    { 
        power += 1; 
        if power == 42 
        { 
            // Skip the rest of this iteration 
            continue; 
        } 
        print!("{}  ", power); 
        
        if power == 50 
        { 
            print!("OK, that's enough for today"); 
            break;  // exit the loop 
        } 
    }

    'outer: loop 
    { 
        println!("Entered the outer dungeon."); 
        'inner: loop 
        { 
            println!("Entered the inner dungeon."); 
            // break;    // this would break out of the inner loop 
            break 'outer; // breaks to the outer loop 
        } 
        println!("This treasure can sadly never be reached."); 
    } 
    println!("Exited the outer dungeon!");  
        // for loops
        // for var in a..b 
    for n in 1..11 
    { 
      println!("The square of {} is {}", n, n * n); 
    }
    // don't allocate any variable
    for _ in 1..11 
    { 
        println!("AAAAAAAAAAAAAAAAAAAAAH"); 
    }

    let mut x = 10; 
    for _ in 1..x 
    { 
        x -= 1; print!("."); 
    }

    // Functions

    let hero1 = "Pac Man"; 
    let hero2 = "Riddick"; 
    greet(hero2); 
    greet_both(hero1, hero2); 

    // fn functionName(var1: type, var2: type) 


} 
 
fn greet(name: &str) 
{ 
  println!("Hi mighty {}, what brings you here?", name); 
} 
 
fn greet_both(name1: &str, name2: &str) 
{ 
  greet(name1); 
  greet(name2); 
}

