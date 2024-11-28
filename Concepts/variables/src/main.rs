fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);

    const SB_COUNT:u32 = 1000000; //This is always immutable
    
    let y = 8;
    println!("{}", y);
    let y = "sdf";
    println!("{}", y);

    //Compund data types fixed size array with related data types
    let tup = ("let's leran rust", 100_090);
    let (channel, sub_count) = tup;
    println!("{:?}{}{}", tup, sub_count, channel);

    let cnt = tup.1;
    let ary = [100, 324, 23];
    let ntfnd = ary[1];
    
    println!("{}{:?}{}", cnt, ary, ntfnd);
    let sum = my_function(11,11);
    println!("the sum is {}", sum);

    let mut counter = 0;

    let result = loop{
        counter+=1;
        if counter==10{
            break counter;
        }
    };
    println!("{}", counter);
    for element in ary.iter(){
        println!("{}", element);
    }
    for number in 1..4{
        println!("{}", number);
    }
}

fn my_function(x:i32, y:i32) -> i32{
    println!("inside the my_function(){}", x+y);
    x+y
}
// data types: integers i8, i16,.. u8, u16 signed and unsigned
// floating f32,f64
// booleans bool 
// char
// compund types
