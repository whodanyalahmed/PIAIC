// Author = Danyal Ahmed 
// facebook = fb.com/whodanyalahmed
// twitter = twitter.com/whodanyalahmed
// insta = @whodanyalahmed
fn main() {
    
    // Question 1
    let x = "PAKISTAN ZINDABAD";
    println!("Q1:\n {}",x );
    println!(" Length of PAKISTAN ZINDABAD is {}",x.len() );


    let y:u64 = 85;
    let z:i16 = -550;


    // Question 2
    println!("Q2:\n {} \n{}",y,z ); 


    // Question 3
    let a:f32 = 56.6;
    println!("Q3:\n {}",a );


    // Question 4

    let b = 76;
    let c = 23;
    println!("Q4:\n b + c = {}",(b+c) );
    println!(" b - c = {}",(b-c) );
    println!(" b * c = {}",(b*c) );
    println!(" b / c = {}",(b/c) );
    println!(" b % c = {}",(b%c) );


    // Question 5
    let d = [100, 150, 200, 250,300];
    println!("Q5: \n {:?}\n {}\n {}",d,d[1],d[3] );


    // Question 6
    let e = ("IoT","AI","Cloud",500.65, 8645, 65.4);
    println!("Q6:\n {:?}\n {}\n {}\n {}", e,e.2,e.4,e.5);


    // Question 7
    println!("Q7: ");
    fn add(x: i16,y: i16,z: i16){
        println!(" {}", (x+y+z));
    }


    add(10,20,30);

    // Question 8
    println!("Q8: ");
    fn mul(x: f32,y: f32,z: f32) -> f32{
        x*y*z
    }


    println!(" {}",mul(5.6,2.4,10.2));

    
    // Question 9
    println!("Q9: ");
    let marks = 62;
    if marks >= 80 {
        println!(" Grade A+");
    }
    else if marks >= 70{
        println!(" Grade A");
    }
    else if marks >= 60{
        println!(" Grade B");
    }
    
    else if marks >= 50 {
        println!(" Grade C");
    }
    
    else if marks >= 70 {
        println!(" Grade D");
    }
    else{
        println!(" Grade F")
    }

    // Question 10
    println!("Q10: ");

    let year = 2019;
    if year%4==0 {
        println!(" {} is a leap year", year);
    }
    else{
        println!(" {} is not a leap year", year);
    }


    // Question 11
    println!("Q11: ");
    for i in 2..20 {
        if i%2 == 0 {
            print!(" {}",i)
        }
    }

    
    // Question 12
    println!("\nQ12: ");
    let mut num = 1;

    while num <= 20 {
        print!(" {}",num );
        num = num + 2;
    }

    
    // Question 13
    println!("\nQ13: ");
    let table = 5;
    for tab in 1..13 {
        println!(" {} * {} = {}",table,tab,table*tab );
    }


}
