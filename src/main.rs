use std::io;

#[derive(Debug)]
struct Task{
    name: String,
    completed : bool,
}

fn add_task(tasks: &mut Vec<Task>) {
    println!("Enter the name of the task: ");
    let mut task_name = String::new();
    io::stdin().read_line(&mut task_name).expect("Faild to read line!");

    let task = Task{
        name : task_name.trim().to_string(),
        completed : false,
    };
    tasks.push(task);

    println!("Task added successfully.");
}



fn view_task(tasks: &mut Vec<Task>){
    println!("View task is working now!");

    if tasks.is_empty(){
        println!("Your task list is empty!");
        return;
    }

    // Way 1---------------------------------
    // for task in tasks.iter(){
    //     println!("{:?}", task);
    // }

    // Way 2-----------------------------------------
    // for task in tasks.iter(){
    //     println!("Title: {}, Completed status: {}", task.name, task.completed);
    // }

    // Way 3 -----------------------------------------
    for (index, task) in tasks.iter().enumerate(){
        println!("{}. {} [{}]", index + 1, task.name, if task.completed {"Completed"} else {"Not Competed"});
    }

}

fn remove_task(tasks : &mut Vec<Task>){
    if tasks.is_empty() {
        println!("There is no task to remove! Task is empty!");
        return;
    }

    println!("Enter the index value which you want to remove from the task list: ");
    
    // Way 1---------------------------------------------------
    // let mut index = String::new();
    // io::stdin().read_line(&mut index).expect("Faild to read line!");

    // let parsed_index : usize = index.trim().parse().expect("Invalid input");
    // tasks.remove(parsed_index);

    // Way 2 --------------------------------------------------
    let mut index_str = String::new();
    io::stdin().read_line(&mut index_str).expect("Failed to read line!");

    let index: usize = match index_str.trim().parse() {
        Ok(num) => num,
        Err(_) =>{
            println!("Please enter a valid number.");
            return;
        },
    };

    if index > 0 && index <= tasks.len() {
        tasks.remove(index-1);
        println!("Task removed successfully.");
    }
    else{
        println!("There's no task with that number.")
    }

    println!("Task Removed Completed!");


}


fn main() {
    println!("Command-Line To-Do List Application\n");

    let mut tasks: Vec<Task> = Vec::new();

    loop{
        println!("What would you like to do?\n1. Add a task\n2. View tasks\n3. Remove a task\n4. Exit\n");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        match choice.trim() {
            "1" => add_task(&mut tasks),
            "2" => view_task(&mut tasks),
            "3" => remove_task(&mut tasks),
            "4" => break,
            _ => println!("Invalid option, please try again"),
        }
    }
}
