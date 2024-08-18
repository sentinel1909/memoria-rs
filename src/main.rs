// src/main.rs

// dependencies
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};

// type aliases
type UpdateResult = std::result::Result<(), String>;

// struct type to represent a code Snippet
#[derive(Debug, Default)]
struct Snippet {
    items: HashMap<String, String>,
}

// methods for the Snippet type
impl Snippet {
    // create method; creates a new key, value pair
    fn create(&mut self, key: String, value: String) -> Option<String> {
        self.items.insert(key, value)
    }

    // retrieve method; retrieves a given value given a key
    fn retrieve(&self, key: String) -> Option<&String> {
        self.items.get(&key)
    }

    // update method; updates the value associated with a given key
    fn update(&mut self, key: String, updated_value: String) -> UpdateResult {
        self.items
            .get_mut(&key)
            .map(|value| *value = updated_value)
            .ok_or_else(|| format!("Item '{}' not found", key))
    }

    // delete method; deletes a key, value pair given a key
    fn delete(&mut self, key: String) -> Option<String> {
        self.items.remove(&key)
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
fn read_data() -> std::io::Result<Vec<u8>> {
    let data = fs::read("data/items.txt")?;
    Ok(data)
}

// function to convert the saved input into our Snippet type
fn input_to_snippet(raw_data: Vec<u8>, mut snippet: Snippet) -> Snippet {
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

// function to write output to stdout
fn write_message(message: &[u8], writer: &mut dyn Write) {
    if let Err(e) = writer.write_all(message) {
        eprintln!("Error writing to stdout: {}", e);
    }

    if let Err(e) = writer.flush() {
        eprintln!("Error flushing stdout: {}", e);
    }
}

// function which triggers the appropriate program functionality, based on the user choice
fn handle_menu_choice(choice: Menu, snippet: &mut Snippet, mut handle: &mut dyn Write) {
    match choice {
        Menu::Create => {
            write_message(b"Enter the new key:", &mut handle);
            let new_key = get_user_input();
            write_message(b"Enter the new value for that key: ", &mut handle);
            let new_value = get_user_input();
            Snippet::create(snippet, new_key.clone(), new_value.clone());
            write_message(
                format!("Created new key: {} with value: {}\n", new_key, new_value).as_bytes(),
                &mut handle,
            );
        }
        Menu::Retrieve => {
            write_message(b"Enter the desired key: ", &mut handle);
            let key = get_user_input();
            match Snippet::retrieve(snippet, key) {
                Some(value) => {
                    write_message(format!("Retrieved: {:?}\n", value).as_bytes(), &mut handle)
                }
                None => {
                    write_message(b"There is no key value pair that matches\n", &mut handle);
                }
            };
        }
        Menu::Update => {
            write_message(b"Enter the desired key to update: ", &mut handle);
            let key = get_user_input();
            write_message(b"Enter the desired new value: ", &mut handle);
            let updated_value = get_user_input();
            let _result = Snippet::update(snippet, key.clone(), updated_value);
            write_message(format!("Updated {:?} successfully.\n", key).as_bytes(), &mut handle);
        }
        Menu::Delete => {
            write_message(b"Enter the desired key to delete: ", &mut handle);
            let key = get_user_input();
            let result = Snippet::delete(snippet, key);
            if let Some(deleted) = result {
                write_message(format!("Deleted: {:?}\n", deleted).as_bytes(), &mut handle);
            }
            
        }
        Menu::Exit => {
            write_message(b"Exiting the program.\n", &mut handle);
        }
    }
}

// main function
fn main() -> std::io::Result<()> {
    // initialize stdout for output to the terminal
    let mut handle = io::stdout();

    // initialize an instance of our snippet type
    let snippet = Snippet::default();

    // load input from the saved file
    let raw_data = read_data()?;

    // convert the input into the Snippet type
    let mut data = input_to_snippet(raw_data, snippet);

    // the main program loop; display the menu choices, act on them, exit the program if "E" is selected
    loop {
        // display the menu options
        write_message(b"Menu: \n", &mut handle);
        write_message(b"C - Create \n", &mut handle);
        write_message(b"R - Retrieve \n", &mut handle);
        write_message(b"U - Update \n", &mut handle);
        write_message(b"D - Delete \n", &mut handle);
        write_message(b"E - Exit \n", &mut handle);

        // display a message asing for the user to make a menu choice
        write_message(b"Enter your choice: \n", &mut handle);

        // trigger the appropriate menu option based on the user's choice
        match get_user_input().to_uppercase().as_str() {
            "C" => handle_menu_choice(Menu::Create, &mut data, &mut handle),
            "R" => handle_menu_choice(Menu::Retrieve, &mut data, &mut handle),
            "U" => handle_menu_choice(Menu::Update, &mut data, &mut handle),
            "D" => handle_menu_choice(Menu::Delete, &mut data, &mut handle),
            "E" => {
                handle_menu_choice(Menu::Exit, &mut data, &mut handle);
                break;
            }
            _ => {
                writeln!(handle, "Invalid Choice. Please enter C, R, U, D, or E.\n").unwrap();
            }
        };
    }

    Ok(())
}
