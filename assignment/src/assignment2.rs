use std::io;
use std::convert::TryInto;
fn main(){
// Question #1
// In this code x is borrowed to y thats why x cant be printed


// Quesiton #2
let s = String::from("PAKISTAN");
    println!("{}",s);

conc(&s);
fn conc(x: &String){
    println!("{} ZINDABAD",x);
}

// Question #3

// println!("Enter three value for average: ");
// let mut a = String::new();
// io::stdin().read_line(&mut a);
// let a:i32 = a.trim().parse().unwrap();
// let mut b = String::new();
// io::stdin().read_line(&mut b);
// let mut c = String::new();
// io::stdin().read_line(&mut c);
// let b:i32 = b.trim().parse().unwrap();
// let c:i32 = c.trim().parse().unwrap();
// println!("the average of {} {} {} these num is: {} ",a,b,c,(a+b+c)/3 );

// Quesiton #4
// println!("Enter any tense: ");
// let mut x = String::new();
// io::stdin().read_line(&mut x);
// println!("The length of \"{}\" is {} ",x, ln(&x));


// fn ln(t: &String) -> u32{
//     let r = t.trim().len().try_into().unwrap();
//     r
// }



// Question #5
// println!("Enter any tense: ");
// let mut x = String::new();
// io::stdin().read_line(&mut x);
// let y:i32 = x.trim().parse().unwrap();
// for i in 0..(y+1){
//         for _r in 0..i{
//             print!("*");    
//         }
//         println!("");
//     }

// Question #6

// #[derive(Debug)]
// struct Student{
//     Name : String ,
//     Email : String ,
//     Phone_number : u32 ,
//     Gender : String ,

// }


// let student1 = Student{
//     Name : String::from("soul"),
//     Email : String::from("soul@hotmail.com"),
//     Phone_number : 3513555165,
//     Gender : String::from("Female"),
// };

// let student2 = Student{
//     Name : String::from("Dan"),
//     Email : String::from("Danthedirector@gmail.com"),
//     Phone_number : 214351553,
//     Gender : String::from("Male"),
// };

// println!("{}",student1.Email );
// println!("{:?}",student2);

// Question #7


// struct Rectangle {  
// width: u32, height: u32 
// }   
// impl Rectangle{
// fn sum(&self) -> u32{
//     self.width*self.height
// }
// }

// let mut rect1 = Rectangle{ width: 50, height: 100 };  
// rect1.width = 150;
// println!("{}",rect1.width );


// Question #8
// println!("{}",rect1.sum());

// Question #9
// struct Triangle{
//     length1 : u32,
//     length2 : u32,
//     length3 : u32,
// }
// impl Triangle{
//     fn sum(&self) -> u32{
//         self.length1 + self.length2 + self.length3
//     }
//     fn max  (&self)   -> u32{
//         if (self.length1 > self.length2){
//             if(self.length1> self.length3)
//             {
//                 self.length1
//             }
//             else{
//                 self.length3
//                 }
//                                         }
//         else{
//             if(self.length2 > self.length3){
//                 self.length2
//             }
//             else{
//                 self.length3
//                 }
//             }
//                             }       
//                 }

// let tri1 = Triangle{
//     length1 : 25,
//     length2 : 80,
//     length3 : 60,
// };

// println!("{}", tri1.sum());
// println!("{}", tri1.max() );


// // Question #10
// struct Person{
//     name: String,
//     age: String,
//     country: String,
// }


// let mut per = Person{
//     name: String::new(),
//     age: String::new(),
//     country: String::new(),
// };
// println!("Enter name: ");
// io::stdin().read_line(&mut per.name);
// println!("Enter age: ");
// io::stdin().read_line(&mut per.age);
// println!("Enter country: ");
// io::stdin().read_line(&mut per.country);

// let x = [per.name,per.age,per.country];

// println!("{:?}",x );

// let x = [per.name,per.age,per.country];

// for i in x{
//     print!("{}",i );
// }

}





