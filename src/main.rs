use std::io;
use std::io::Write;
use rpassword;
use json;


// 'View' stores the relationship between parent-child views.
// Specifically, 'View' stores the contents of a page and its links to children pages.
struct View {
    title: String,
    options: Vec<View>,
    has_field: bool,
}

fn build_view(title: String, options: Vec<View>, has_field: bool) -> View {
    View {
        title,
        options,
        has_field,
    }
}

// This changes the contents of the application.
fn change_view(current_view: &View, choice: &u8) -> View {
    // Add code to clear and print new pages based on the selected View.

    let new_view: View = current_view.options[choice];

    print!("\x1b[2J\x1b[1;1H"); // Clears the page and sets the cursor to one line below the top line. (Unable to place it at 0,0)
    io::stdout().flush().unwrap();

    // if not has_field... then do below. else has_field prompt. Add this code.
    println!("{}", new_view.title);
    println!("[0] Exit");
    for n in 0..new_view.options.len()-2 {
        println!("[{n+1}] {new_view[n].title}");
    }
    
    return new_view;

}

// This function processes the user input within a view,
// checks if the input is valid, and, if required, opens 
// a child loop for another view.
fn process_input(current_view: &View, prompt: String) {

    loop {
    
        prompt.clear();

        print!("Input-: ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut prompt)
            .expect("Failed to read line.");

        // Add validation code to check if input exists as a view.

        // Breaks the current loop and goes to the parent loop and parent view. 
        if prompt == "0\n" { 
            break;
        }

        process_input(&change_view(current_view, &prompt[..1].parse().unwrap()), prompt);

        // Breaks all loops
        if prompt == "-1\n" { 
            break; 
        }

    }
    
}

fn main() {

    println!("--- Password Manager ---");
    print!("Master Key: ");
    io::stdout().flush().unwrap();

    let key: String = rpassword::read_password().unwrap();
    if key != "asdf" { std::process::exit(1) } // This is temporary.

    let access_item: View = build_view("Access item".to_string(), vec![], false);
    let add_item: View = build_view("Add item".to_string(), vec![], false);
    let home: View = build_view("Home".to_string(), vec![add_item, access_item], false);
    

    let mut prompt: String = String::new();

    
    // Redesign this section and make it tidier.
    println!("\n[1] Add an item\n[2] Access an item\n[-1] Exit\n");

    loop {

        print!("Input-: ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut prompt)
            .expect("Failed to read line.");

        process_input(&home, prompt);

        if prompt == "-1\n" { 
            break; 
        }

    };    

}
