// use std::io; 

enum IpaddrKind
{
    v4,
    v6
}

struct Ipaddr{
    kind: IpaddrKind,
    address: String,
}

fn main() {
    // Custom data type 
    // let x = Rectangle{
    //     width : 32,
    //     height : 12
    // };

    // println!("{:?}",x.width);

// Method and function

    // For rectangle
    // let x = Rectangle{
    //     width : 3,
    //     height : 2,
    // };
    // let y = Rectangle{
    //     width: 4,
    //     height: 10,
    // };

    // println!("The area of x rectangle : {}",x.area());
    // println!("The area of y rectangle : {}",y.area());

    // For student


    // let mut std1 = Student{
    //     name : String::from("Dan"),
    //     marks : 8,
    //     suspended : false,
    // };
    // println!("The student is suspended: {}",std1.suspended);

    // std1.do_suspend();

    // println!("The marks are : {}",std1.print_marks() );
    // println!("The student is suspended : {}",std1.suspended);
    // std1.chg_marks(34);

    // println!("The new marks are: {}",std1.marks );
    // Student::calculate_avg_per(32,243,43);

    // User Input
    // println!("Please enter value : ");
    // let mut x = String::new();
    // io::stdin().read_line(&mut x);
    // let x:i32 = x.trim().parse().unwrap();
    // println!("First number is: {}",x);

    // let mut y = String::new();
    // println!("Enter second value: ");
    // io::stdin().read_line(&mut y);
    // let y:i32 = y.trim().parse().unwrap();
    // println!("Second number is: {}",y);


    // println!("The sum of {} and {} is {}",x,y,x+y);
    
    // println!("The subtraction of {} and {} is {}",x,y,x-y);
    
    // println!("The Multiplication of {} and {} is {}",x,y,x*y);
    
    // println!("The division of {} and {} is {}",x,y,x/y);
    
    // println!("The Remainder of {} and {} is {}",x,y,x%y);

    // Enum
    let four = IpaddrKind::v4;
    let six =     IpaddrKind::v6;

    let home = Ipaddr{
        kind: IpaddrKind::v4,
        address: String::from("127.0.0.1"),

    };

    let office = Ipaddr{
        kind: IpaddrKind::v6,
        address: String::from("123.4.56.1"),
    };


    println!("The ip of office is : {:?}",office.);


}
// #[derive(Debug)]
// struct Rectangle{
//     width : u32,
//     height : u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32{
//         self.height * self.width
//     }
// }

// struct Student{
//     name : String,
//     marks : u32,
//     suspended: bool,
// }

// impl Student{
//     fn print_marks(&self) -> u32{
//         self.marks
//     }

//     fn do_suspend(&mut self){
//         self.suspended = true;
//     }
//     fn chg_marks(&mut self, new_marks: u32){
//         self.marks = new_marks;
//     }

//     fn calculate_avg_per(a:u32,b:u32,c:u32){
//         let sum = a + b + c;
//         let avg = (sum * 300)/100;
//         println!("{}",avg );


//     }
// }