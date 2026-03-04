mod dialog;

use clap::{Parser, Subcommand};
use jlrs::convert::to_symbol::ToSymbol;
use jlrs::prelude::*;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    // #[arg(long)]
    // one: String,
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Play,
    Price,
    Example,
}

fn main() {
    let args = Cli::parse();

    // start runtime if needed
    let handle = if matches!(args.command, Commands::Play | Commands::Price) {
        let handle = Builder::new().start_local().expect("cannot init Julia");

        unsafe {
            handle
                .include("julia/Motoro/src/Motoro.jl")
                .expect("Failed to load Motoro.jl");
        }

        Some(handle)
    } else {
        None
    };

    match args.command {
        Commands::Play => handle.unwrap().local_scope::<_, 3>(|mut frame| {
            let motoro = Module::main(&frame)
                .submodule(&mut frame, "Motoro")
                .unwrap();

            let game_func = motoro.global(&mut frame, "game").expect("game not found");

            unsafe {
                game_func.call(&mut frame, []).expect("exception in game()");
            }
        }),
        Commands::Price => handle.unwrap().local_scope::<_, 11>(|mut frame| {
            let inputs = dialog::get_option_inputs();

            // converts data from Rust to Julia
            // I believe there is a reflect function so I can imrove this
            let strike = Value::new(&mut frame, inputs.strike);
            let expiry = Value::new(&mut frame, inputs.expiry);
            let binomial = Value::new(&mut frame, inputs.binomial);
            let spot = Value::new(&mut frame, inputs.spot);
            let rate = Value::new(&mut frame, inputs.rate);
            let vol = Value::new(&mut frame, inputs.vol);
            let div = Value::new(&mut frame, inputs.div);

            let option = call_motoro(&mut frame, &inputs.option_type, &[strike, expiry]).unwrap();
            let engine = call_motoro(&mut frame, "Binomial", &[binomial]).unwrap();
            let data = call_motoro(&mut frame, "MarketData", &[spot, rate, vol, div]).unwrap();
            let result = call_motoro(&mut frame, "price", &[option, engine, data]).unwrap();

            let price = result.unbox::<f64>().expect("price did not return Float64");

            println!("\nResult: {price}");
        }),
        Commands::Example => dialog::example_thing(),
    }
}

fn call_motoro<'target, Tgt, N>(
    target: Tgt,
    name: N,
    args: &[Value<'_, 'static>],
) -> ValueResult<'target, 'static, Tgt>
where
    Tgt: Target<'target>,
    N: ToSymbol,
{
    target.with_local_scope::<_, 2>(|target, mut frame| {
        let func = Module::main(&frame)
            .submodule(&mut frame, "Motoro")
            .unwrap()
            .global(&mut frame, name)
            .expect("item not found in Main");

        // call/instantiate
        unsafe { func.call(target, args) }
    })
}
