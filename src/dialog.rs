use dialoguer::{Input, Select, theme::ColorfulTheme};

pub struct UserInputs {
    pub option_type: String,
    pub strike: f64,
    pub expiry: f64,
    pub binomial: f64,
    pub spot: f64,
    pub rate: f64,
    pub vol: f64,
    pub div: f64,
}

pub fn get_option_inputs() -> UserInputs {
    let selections = &["EuropeanCall", "EurpoeanPut"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick an option")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();
    let strike = float_input("Enter the strike");
    let expiry = float_input("Enter the expiry");

    println!("\nBinomial");
    let binomial = float_input("Enter the binomial number");

    println!("\nMarket Data");
    let spot = float_input("Enter spot");
    let rate = float_input("Enter rate");
    let vol = float_input("Enter vol");
    let div = float_input("Enter div");

    UserInputs {
        option_type: selections[selection].to_string(),
        strike,
        expiry,
        binomial,
        spot,
        rate,
        vol,
        div,
    }
}

fn float_input<T>(prompt: T) -> f64
where
    T: Into<String>,
{
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .validate_with(|input: &String| -> Result<(), &str> {
            match input.parse::<f64>() {
                Ok(_) => Ok(()),
                Err(_) => Err("You must enter an f64"),
            }
        })
        .interact_text()
        .unwrap()
        .parse()
        .unwrap()
}

pub fn example_thing() {
    let selections = &[
        "Ice Cream",
        "Vanilla Cupcake",
        "Chocolate Muffin",
        "A Pile of sweet, sweet mustard",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick your flavor")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    println!("Enjoy your {}!", selections[selection]);

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Optionally pick your flavor")
        .default(0)
        .items(&selections[..])
        .interact_opt()
        .unwrap();

    if let Some(selection) = selection {
        println!("Enjoy your {}!", selections[selection]);
    } else {
        println!("You didn't select anything!");
    }

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick your flavor, hint it might be on the second page")
        .default(0)
        .max_length(2)
        .items(&selections[..])
        .interact()
        .unwrap();

    println!("Enjoy your {}!", selections[selection]);
}
