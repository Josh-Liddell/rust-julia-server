use std::fmt::Debug;
use std::str::FromStr;

use crate::price::{Binomial, EuropeanOption, MarketData, OptionContract, Priceable};
use dialoguer::{Input, Select, theme::ColorfulTheme};

pub fn price_option_rust() {
    let selections = &["EuropeanCall", "EurpoeanPut"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick an option")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();
    let strike: f64 = dialog_input("Enter the strike");
    let expiry: f64 = dialog_input("Enter the expiry");

    println!("\nBinomial");
    let steps: i32 = dialog_input("Enter the binomial number");

    println!("\nMarket Data");
    let spot: f64 = dialog_input("Enter spot");
    let rate: f64 = dialog_input("Enter rate");
    let vol: f64 = dialog_input("Enter vol");
    let div: f64 = dialog_input("Enter div");

    // right now ITS ONLY doing european option
    let option = EuropeanOption {
        contract: OptionContract {
            strike,
            expiry,
            option_type: selections[selection].parse().unwrap(),
        },
    };

    let b = Binomial {
        steps: steps as usize,
    };

    let data = MarketData {
        spot,
        rate,
        vol,
        div,
    };

    let result = option.price(&b, &data);
    println!("\nThe result is {result}");
}

fn dialog_input<T, S>(prompt: S) -> T
where
    S: Into<String>,
    T: FromStr,
    T::Err: Debug,
{
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .validate_with(|input: &String| -> Result<(), &str> {
            match input.parse::<T>() {
                Ok(_) => Ok(()),
                Err(_) => Err("You must enter the correct thing"),
            }
        })
        .interact_text()
        .unwrap()
        .parse::<T>()
        .unwrap()
}

pub fn example_dialog() {
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
