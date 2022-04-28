use std::io::stdin;

fn ask_name() -> String {
    let mut name = String::new();
    stdin()
        .read_line(&mut name)
        .expect("Failed to read line for name");

    // no semicolon here is Rust shorthand for a return
    name
}

fn ask_name_with_trim() -> String {
    let mut name = String::new();
    stdin()
        .read_line(&mut name)
        .expect("Failed to read line for name"); //

    name.trim().to_string()
}

#[derive(Debug)]
enum VisitorAction {
    Accept,                          // simple enumeration
    AcceptWithNote { note: String }, // enumeration with data
    Refuse,
    Probation,
}

#[derive(Debug)]
struct Visitor {
    name: String,
    action: VisitorAction,
    age: i8,
}

impl Visitor {
    // associated function, cant use self, use Visitor::new()
    // instead
    fn new(name: &str, age: i8, action: VisitorAction) -> Self {
        // note the lack of a ; - for the implicit return
        Self {
            name: name.to_string(),
            age, // Rust will automatically use the contents of the variable with the same name
            action,
        }
    }

    // // regular member function/method
    // fn greet_visitor(&self) {
    //     println!("{}", self.greeting);
    // }

    fn greet_visitor(&self) {
        match &self.action {
            VisitorAction::Accept => println!("Welcome to the Treehouse, {}", self.name),
            // include the fields with data in the curly brackets. This is called
            // destructuring
            VisitorAction::AcceptWithNote { note } => {
                println!("Welcome to the Treehouse, {}", self.name);
                println!("{}", note);
                if self.age < 16 {
                    println!("Do not serve any alcohol to {}!", self.name);
                }
            }
            VisitorAction::Probation => println!("{} is now a probationary member.", self.name),
            VisitorAction::Refuse => println!("Do not allow {} in!", self.name),
        }
    }
}

fn main() {
    println!("Hello, what is your name?");
    let your_name = ask_name();

    // either use the ask_name_with_trim()-function above
    // or trim the read name manually.

    // {:?} instead of {} is the debug placeholder instead
    // of the regular placeholder. This will print additional info.
    // {:#?} is the pretty print function.

    println!("Hello, {:?}", your_name);
    println!("Hello, {:#?}", your_name);
    println!("Now with trim():");
    println!("Hello, {:?}", your_name.trim());

    let name_trimmed = your_name.trim();

    // these arrays are fixed size.
    // let visitor_list = [
    //     Visitor::new("enrico", "Hello Enrico, enjoy the treehouse."),
    //     Visitor::new("maria", "How are you today?"),
    // ];

    let mut visitors = vec![
        Visitor::new("Enrico", 35, VisitorAction::Accept),
        Visitor::new("Maria", 34, VisitorAction::Accept),
        Visitor::new(
            "Martin",
            15,
            VisitorAction::AcceptWithNote {
                note: String::from("Milk is in the fridge."),
            },
        ),
        Visitor::new("Bert", 45, VisitorAction::Probation),
        Visitor::new("Mark", 30, VisitorAction::Refuse),
    ];

    loop {
        println!("Hello, what's your name? (leave empty to quit)");
        let name = ask_name_with_trim();

        // use an iterator instead of a for-loop
        // the closure in find() will store the result in known_visitor
        let known_visitor = visitors
            .iter()
            .find(|visitor| visitor.name.to_lowercase() == name.to_lowercase());

        // Use an Option to check if the visitor was in the list
        match known_visitor {
            Some(visitor) => visitor.greet_visitor(),
            None => {
                if name.is_empty() {
                    break;
                } else {
                    println!("You are not in the visitor list, {name}. Yet...");
                    visitors.push(Visitor::new(&name, 0, VisitorAction::Probation));
                }
            }
        }
    }

    println!("The final list of visitors:");
    println!("{:#?}", visitors);
}
