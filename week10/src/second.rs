extern crate slib;

fn main
(){
    // let mut v = vec!(10,20,30,40);


    // // Ownership transfered to loop
    // for i in &mut v{
    //     // Dereferencing
    //     *i +=100;
    // }

    // println!("{:?}",v );

    let s = "Hello";
    let data = s.to_string();
    println!("{}",data);

    // let hello = String::from("Dobrý den"); let hello = String::from("Hello"); let hello = String::from("לוֹםָ );"שׁ let hello = String::from("नमे"); let hello = String::from("こんにちは"); let hello = String::from("안녕하세요"); let hello = String::from("你好"); let hello = String::from("Olá"); let hello = String::from("Здравствуйте");
    // println!("{}",hello );

    let mut name = String::from("Danyal");
    let fu = name.push_str("Ahmed");
    println!("{:?}",fu );



    let s1 = String::from("PIAIC");
    let s2 = String::from(" IOT");
    let s3 = String::from(" Batch");
    // Concat   
    // println!("{} ",s1+&s2 +&s3);

    // format
    
    // let s = format!("{}-{}-{}",s1,s2,s3);
    // println!("{}",s );


    // let s = s1[0];
    // println!("{}",s );

    
//     let len = String::from("Здравствуйте").len();
//     println!("{}",len );

// let hello ="pakistan";
// let dan = &hello[0..4];
// print!("{}",dan);