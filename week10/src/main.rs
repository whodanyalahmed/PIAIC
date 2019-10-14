fn main() {
    // Vectors - Dynamic array
    
    let _v:Vec<i32> =  Vec::new();
    let _v2 = vec!(1,2,3);

    let mut shopping_list = vec!("Veggie","snacks","eggs");

    shopping_list.push("Chicken");
    println!("shopping list is : {:?}",shopping_list);

    shopping_list.pop();
    println!("Shopping list updated: {:?}",shopping_list );


    // Second Element
    let second = &shopping_list[2];
    println!("second element is : {}",second );

    let second_shopping = shopping_list.get(1);
    println!("the second element is: {:?}",second_shopping );
    
    match second_shopping{
        Some(d) => println!("The value is : {}",d ),
        None => println!("Data doesnt exists", )
    }

    enum Human{
        height(f32),
        Name(String),
        Age(i32),
    };
    
    let mut Humar_vec :Vec<Human> = Vec::new(); 
    Humar_vec.push(Human::height(3.3));
    Humar_vec.push(Human::Name(String::from("Dan")));
    Humar_vec.push(Human::Age(20));
    let mut vector_in_vector:Vec<Vec<i32>> = Vec::new();
    let inner_vector = vec!(1,2,3);
    let innter_vector2 = vec!(4,5,6);
    vector_in_vector.push(inner_vector);
    vector_in_vector.push(innter_vector2);
    print!("{:?}",vector_in_vector );



    let mut tuple_vector :Vec<(i32,i32)> = Vec::new();




}
