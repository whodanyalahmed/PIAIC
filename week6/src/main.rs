fn main() {
        // Scope
        // let s = String::from("Hello!");
        // {

        //     let s1 = String::from("Hello!");
        //     println!("{}",s1 );
        // }

        // OwnerShip
        // ---------

        // let s1 = String::from("Hello!");

        // let s2 = s1;

        // println!("{}",s2 );
        // println!("{}",s1);


    // take_ownership(s1);
    // println!("{}",s1 );
    // let s2= 32;
    // take_int(32);



// Reference
        // let s1 = String::from("Hello!");
    // take_ownership(&s1);
    // println!("{}",s1 );
    // let len = Calculate_length(s1);
    // println!("{}",len );

// Mutable Reference
// let mut s = String::from("Hello!");
// change(&mut s);
// println!("{}",s );

// Dangling Reference

// let ref_to_nothing = dangle();

let s = String::from("Hello!");

let c = &s;
let d = &s;
let e = &s;



}
    // fn dangle() -> &String {
    //     let s = String::from("Hello!");
    //     &s
    // }

    // fn change(some_string:&mut String ){
    //     some_string.push_str(", world");
    // }
    // fn Calculate_length(s:String ) -> usize{
    //     s.len()
    // }


    // fn take_ownership(x: &String){
    //     println!("{}",x);
    // }
    // fn take_int(x:i32){
    //     println!("{}",x );
    // }