use std::io;
use std::io::Write;
use rpassword;
use json;

// Tree and TreeItem creates an Tree-Like lookup structure to reference corresponding Menu Text in the application.
// struct MenuTree {
//     items: [(String, u8, u8)],
// }

fn main() {

    println!("--- Password Manager ---");
    print!("Master Key: ");
    io::stdout().flush().unwrap();

    let key: String = rpassword::read_password().unwrap();
    if key != "asdf" {
        std::process::exit(1)
    }

    let mut prompt: String = String::new();

    // let MENU_ITEMS: MenuTree = MenuTree{
    //     items: [
    //         ("\n[1] Add new item\n[2] Access an item\n", 0, 2),
    //     ]
    // }

    println!("\n[1] Add new item\n[2] Access an item\n");

    loop {
        
        print!("Input-: ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut prompt)
            .expect("Failed to read line.");

        break;

    };    

}
