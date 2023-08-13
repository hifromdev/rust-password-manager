use std::io;
use std::io::Write;
use rpassword;
use json;


// View stores the data related to different pages in the application.
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

// Changes application text as the user navigates through the application.
fn change_view(current_view: &View, choice: &u8) -> View {
    // Add code to clear and print new pages based on the selected View.
}

// This function processes the user input on a given page,
// checks if the input is valid, and opens a child loop 
// when it navigates into another view.
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

        process_input(&change_view(current_view, choice), prompt);

        // Breaks all loops
        if prompt == "-1\n" { 
            break 'app; 
        }

    }
    
}

fn main() {

    println!("--- Password Manager ---");
    print!("Master Key: ");
    io::stdout().flush().unwrap();

    let key: String = rpassword::read_password().unwrap();
    if key != "asdf" { std::process::exit(1) } // This is temporary.

    // let home: View = build_view("Home", []);
    // let add_item: View = build_view("Add item", [])

    let mut prompt: String = String::new();

    

    println!("\n[1] Add an item\n[2] Access an item\n[0] Exit\n");

    loop {

        print!("Input-: ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut prompt)
            .expect("Failed to read line.");

        process_input(home, prompt)

        if prompt == "-1\n" { 
            break; 
        }

    };    

}
