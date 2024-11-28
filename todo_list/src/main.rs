struct Task{
    description: String,
    is_completed: bool
}

fn create_task(tasks: &mut Vec<Task>){
    println!("Enter the task to create");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to get the input");
    tasks.push(Task{description: input, is_completed: false});
}
fn view_tasks(tasks: & Vec<Task>){
    if tasks.is_empty(){
        println!("No tasks till now");
    }
    for (idx, task) in tasks.iter().enumerate(){
        let state = if task.is_completed {"Done"} else {"Not done"};
        println!("{} {} {}", idx+1, task.description, state.trim().to_string());
    }
}
fn delete_task(tasks: &mut Vec<Task>){
    println!("Enter the task id to delete");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to catch a input");
    match input.trim().parse::<usize>(){
        Ok(index) => {
            if index>0 && index<=tasks.len(){
                tasks.remove(index-1);
                println!("Task removed successfully!");
            }
            else{
                println!("out of bounds index");
            }
        },
        Err(_) => {println!("Error parsing the integer!")},
    };

}
fn complete_task(tasks: &mut Vec<Task>){
    println!("Enter the task to complete");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Could not capture input!");

    match input.trim().parse::<usize>(){
        Ok(index) => {
            if index>0 && index<=tasks.len(){
                tasks[index-1].is_completed = true;
                println!("Task is completed!");
            }
            else{
                println!("Out of bounds")
            }
        },
        Err(_) => {},
    };
}
fn main(){
    let mut tasks: Vec<Task> = Vec::new();

    loop{
        println!("\n1. Create New Tasks\n2. View all tasks\n3. Delete a task\n4. complete a task \n 5. Exit");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to get an input");

        match input.trim(){
            "1" => create_task(&mut tasks),
            "2" => view_tasks(& tasks),
            "3" => delete_task(&mut tasks),
            "4" => complete_task(&mut tasks),
            "5" => {break;},
            &_ => {println!("The index did not match your options try again")}
        };
    }
}
