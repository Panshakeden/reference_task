fn main(){

    print_console();
}



//Creating a dangling reference

fn dangling_fn() -> &string {

    let dangling_string = String::from("Rustaceans");

    &dangling_string 
}

//creating function that accept mutable reference

fn mutable_reference(my_state:&mut String){

    let mut my_state= String::from("Bauchi");

    let current_state= &mut my_state;

}

// print on console

fn print_console(){
    println!("Let's get rusty in the console");
}
