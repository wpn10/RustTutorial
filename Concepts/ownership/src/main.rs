fn main() {
    // Ownersip rules
    // 1. Each value in rust has a variable that is its owner.
    // 2. there can only be on owner at a time
    // 3. when the owner goes out of scope, the value will be dropped

    { // new scope s is not valid 
        // let s = "hello"; // s here is a string literal and fixed in size
        let s = String::from("hello"); //s allocated on the heap automatically deallocated after scope ends
        // do stuff with s 
    } // s is out of scope the value is droopped now  so s is no longer valid 

    let s1 = String::from("hello");
    let s2 = s1; //Move not a shallow copy ownership of heap data is transferred to s2
    let s3 = s2.clone(); //expensive copy

    //owenership and function
    let s = String::from("hello");
    takes_ownership(s); //s no longer valid after giving ownership but it does not work on int and bool since they are copied not moved which is scary!
    let snew = String::from("new hello");
    let sowner = takes_and_gives_back(snew); //This is giving ownership and taking it back
    //borrowing is passing in references in function
    //parameters by doing &var and for mutable references
    //where we can change the values it is &mut var 
    //Only one mutable reference allowed to be paassed 

    let mut s1 = String::from("Hello");
    let len = calculate_length(&mut s1); //Mutable reference and only one mutable reference is possible for any variable at any time which stops data races 
    println!("The lenght of {} is {}", s1, len);

}
fn calculate_length(string: &mut String) -> usize{
    let length = string.len();
    length
}
fn takes_ownership(somestring: String){
    println!("Received the ownership of the string s to some string {}", somestring);
}

fn takes_and_gives_back(somestring: String) -> String {
    somestring
}
