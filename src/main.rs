// src/main.rs

// dependencies
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};

// struct type to represent a code Snippet
#[derive(Debug, Default)]
struct Snippet {
    items: HashMap<String, String>,
}

// methods for the Snippet type
impl Snippet {
    // create method; creates a new key, value pair
    fn create(&mut self, key: &str, value: &str) -> Option<String> {
        self.items.insert(key.to_string(), value.to_string())
    }

    // retrieve method; retrieves a given value given a key
    fn retrieve(&self, key: &str) -> Option<&String>{
        self.items.get(key)
    }

    // update method; updates the value associated with a given key
    fn update(&mut self, key: &str, updated_value: &str) -> Result<(), String> {
        if let Some(item) = self.items.get_mut(key) {
            *item = updated_value.to_string();
            Ok(())
        } else {
            Err(format!("Item '{}' not found", key))
        }
    }

    // delete method; deletes a key, value pair given a key
    fn delete(&mut self, key: &str) -> Option<(String, String)> {
        self.items.remove_entry(key)
    }
}

// enum type for the application menu
enum Menu {
    Create,
    Retrieve,
    Update,
    Delete,
    Exit,
}

// function to read in any saved input
fn read_data() -> Vec<u8> {
    fs::read("data/items.txt").unwrap()
}

// function to convert the saved input into our Snippet type
fn input_to_snippet(
    raw_data: Vec<u8>,
    mut snippet: Snippet,
) -> Snippet {
    let string_data = String::from_utf8(raw_data).unwrap();
    let key_value_pairs = string_data.split("\n").collect::<Vec<&str>>();
    let pairs: Vec<(&str, &str)> = key_value_pairs
        .iter()
        .filter_map(|pair| pair.split_once(":"))
        .collect();

    for (key, value) in &pairs {
        snippet.items.insert(key.to_string(), value.to_string());
    }
    snippet
}

// function to get user input and pass it back for use
fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim().to_string();
    input
}

// function which triggers the appropriate program functionality, based on the user choice
fn handle_menu_choice(
    choice: Menu,
    snippet: &mut Snippet,
    stdout: &mut dyn Write,
) {
    match choice {
        Menu::Create => {
            write!(stdout, "Enter the new key: ").unwrap();
            io::stdout().flush().unwrap();
            let new_key = get_user_input();
            write!(stdout, "Enter the new value for that key: ").unwrap();
            io::stdout().flush().unwrap();
            let new_value = get_user_input();
            let new_item = Snippet::create(snippet, new_key.as_str(), new_value.as_str());
            writeln!(stdout, "Created: {:?}", new_item).unwrap();
        }
        Menu::Retrieve => {
            write!(stdout, "Enter the desired key: ").unwrap();
            io::stdout().flush().unwrap();
            let key = get_user_input();
            let result = Snippet::retrieve(snippet, key.as_str());
            writeln!(stdout, "Retrieved: {:?}", result).unwrap();
        },
        Menu::Update => {
            write!(stdout, "Enter the desired key to update: ").unwrap();
            io::stdout().flush().unwrap();
            let key = get_user_input();
            write!(stdout, "Enter the desired new value: ").unwrap();
            io::stdout().flush().unwrap();
            let updated_value = get_user_input();
            let updated_item = Snippet::update(snippet, key.as_str(), updated_value.as_str());
            writeln!(stdout, "Updated: {:?}", updated_item).unwrap();
        },
        Menu::Delete => {
            write!(stdout, "Enter the desired key to delete: ").unwrap();
            io::stdout().flush().unwrap();
            let key = get_user_input();
            let result = Snippet::delete(snippet, key.as_str());
            writeln!(stdout, "Deleted: {:?}", result).unwrap();
        }
        Menu::Exit => {
            writeln!(stdout, "Exiting the program.").unwrap();
        }
    }
}

// main function
fn main() {
    // initialize stdout for output to the terminal
    let mut stdout = io::stdout();

    // initialize an instance of our snippet type
    let snippet = Snippet::default();

    // load input from the saved file
    let raw_data = read_data();

    // convert the input into the Snippet type
    let mut data = input_to_snippet(raw_data, snippet);

    // the main program loop; display the menu choices, act on them, exit the program if "E" is selected
    loop {
        // display the menu options
        writeln!(stdout, "Menu: ").unwrap();
        writeln!(stdout, "C - Create").unwrap();
        writeln!(stdout, "R - Retrieve").unwrap();
        writeln!(stdout, "U - Update").unwrap();
        writeln!(stdout, "D - Delete").unwrap();
        writeln!(stdout, "E - Exit").unwrap();

        // display a message asing for the user to make a menu choice
        write!(stdout, "Enter your choice: ").unwrap();
        io::stdout().flush().unwrap();
       
        // trigger the appropriate menu option based on the user's choice
        match get_user_input().to_uppercase().as_str() {
            "C" => handle_menu_choice(Menu::Create, &mut data, &mut stdout),
            "R" => handle_menu_choice(Menu::Retrieve, &mut data, &mut stdout),
            "U" => handle_menu_choice(Menu::Update, &mut data, &mut stdout),
            "D" => handle_menu_choice(Menu::Delete, &mut data,&mut stdout),
            "E" => {
                handle_menu_choice(Menu::Exit, &mut data, &mut stdout);
                break;
            }
            _ => {
                writeln!(stdout, "Invalid Choice. Please enter C, R, U, D, or E.").unwrap();

            }
        };
    }
}
