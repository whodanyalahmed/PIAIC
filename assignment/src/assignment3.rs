
enum Type{
    Online,
    Onsite
}

fn main(){

#[derive(Debug)]
struct Candidate{
name: String,
email: String,
// category: Type
}


    let can1 = Candidate{
        name: String::from("Dan"),
        email: String::from("dan@gmail.com"),
        // category: Type::Online
    };
    let ca:Type = Type::Online;

    match ca{
        Type::Online => println!("is online."),
        Type::Onsite => println!("is onsite"),
    }

}

