pub mod resource;
use clap::{self, ValueEnum};
use resource::{print_example_input, print_example_output, print_input, print_output};
pub mod update;
use update::update;
pub mod run;
use run::{run_from_name, run_from_work};
pub mod command;
pub mod language;
pub mod problem;
pub mod state;
pub mod styles;

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Adds files to myapp
    #[command(alias = "u")]
    Update,
    /// Run advent of code solution
    // this takes a file name as a value, like "advent run 1.1"
    #[command(alias = "r")]
    Run(RunArgs),
    /// Print resources (inputs, example inputs/ouputs)
    #[command(subcommand, alias = "re")]
    Resource(ResourceCommands),
    /// Work on problems,
    #[command(subcommand, alias = "w")]
    Work(WorkCommands),
}

#[derive(Args, Debug)]
struct RunArgs {
    /// Problem to run
    problem_list: String,
    /// Test on input, example input/output, or both
    #[clap(value_enum, default_value_t = RunTarget::Input)]
    on: RunTarget,
    /// Don't print program output
    #[clap(short, long)]
    quiet: bool,
}

#[derive(ValueEnum, Debug, Clone, Copy)]
pub enum RunTarget {
    #[clap(alias = "i")]
    Input,
    #[clap(alias = "is")]
    InputThenSubmit,
    #[clap(alias = "ic")]
    InputChecked,
    #[clap(alias = "e")]
    Example,
    #[clap(alias = "ec")]
    ExampleChecked,
    #[clap(alias = "a", alias = "ei")]
    All,
    #[clap(alias = "as", alias = "eis")]
    AllThenSubmit,
    #[clap(alias = "ac", alias = "ecic")]
    AllChecked,
    #[clap(alias = "eci")]
    ExampleCheckedInput,
    #[clap(alias = "ecis")]
    ExampleCheckedInputThenSubmit,
    #[clap(alias = "eic")]
    ExampleInputChecked,
}

#[derive(Subcommand)]
pub enum ResourceCommands {
    /// Print input for day
    #[command(alias = "i")]
    Input(InputArgs),
    /// Print output for day
    #[command(alias = "o")]
    Output(OutputArgs),
    /// Print example input for day
    #[command(alias = "ei")]
    ExampleInput(ExampleArgs),
    /// Print example output for day
    #[command(alias = "eo")]
    ExampleOutput(ExampleArgs),
}

#[derive(Args, Debug)]
pub struct InputArgs {
    day_list: String,
}

#[derive(Args, Debug)]
pub struct OutputArgs {
    problem_list: String,
}

#[derive(Args, Debug)]
pub struct ExampleArgs {
    problem_list: String,
}

#[derive(Subcommand)]
pub enum WorkCommands {
    /// Run code in work
    #[command(alias = "r")]
    Run(WorkRunArgs),
    /// Set work
    #[command(alias = "s")]
    Set(WorkSetArgs),
    /// Save work
    #[command(alias = "sa")]
    Save,
    /// Save and set work to next problem
    #[command(alias = "n")]
    Next,
    /// Show work status
    #[command(alias = "st")]
    Status,
    /// Clear work
    #[command(alias = "c")]
    Clear,
    /// Open work
    #[command(alias = "o")]
    Open,
    /// Init work, by setting, then clearing, then opening
    #[command(alias = "i")]
    Init(WorkSetArgs),
}

#[derive(Args, Debug)]
pub struct WorkRunArgs {
    /// Test on input, example input/output, or both
    #[clap(value_enum, default_value_t = RunTarget::ExampleCheckedInputThenSubmit)]
    on: RunTarget,
    /// Don't print program output
    #[clap(short, long)]
    quiet: bool,
}

#[derive(Args, Debug)]
pub struct WorkSetArgs {
    #[clap(short, long)]
    problem: Option<String>,
    #[clap(short, long)]
    language: Option<language::Language>,
}

use problem::{parse_list, parse_problem};

use anyhow::Result;
fn eachify<I>(func: impl Fn(I) -> Result<()>, vec: Vec<I>) -> Result<()> {
    for i in vec {
        func(i)?;
    }
    Ok(())
}

fn run() -> Result<()> {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Update) => update(),
        Some(Commands::Run(RunArgs {
            problem_list,
            on,
            quiet,
        })) => {
            // run_from_name(parse_problem(&problem), on)
            eachify(
                |problem| run_from_name(problem, on, quiet),
                parse_list(&problem_list)?,
            )
        }
        Some(Commands::Resource(command)) => match command {
            ResourceCommands::Input(InputArgs { day_list }) => {
                eachify(print_input, parse_list(&day_list)?)
            }
            ResourceCommands::Output(OutputArgs { problem_list }) => {
                eachify(print_output, parse_list(&problem_list)?)
            }
            ResourceCommands::ExampleInput(ExampleArgs { problem_list }) => {
                eachify(print_example_input, parse_list(&problem_list)?)
            }
            ResourceCommands::ExampleOutput(ExampleArgs { problem_list }) => {
                eachify(print_example_output, parse_list(&problem_list)?)
            }
        },
        Some(Commands::Work(command)) => match command {
            WorkCommands::Run(WorkRunArgs { on, quiet }) => run_from_work(on, quiet),
            WorkCommands::Set(WorkSetArgs { problem, language }) => {
                state::set_work(problem.map(|s| parse_problem(&s)).transpose()?, language)
            }
            WorkCommands::Save => state::save_work(),
            WorkCommands::Next => state::save_and_next_work(),
            WorkCommands::Open => state::open_work(false),
            WorkCommands::Clear => state::clear_work(),
            WorkCommands::Init(WorkSetArgs { problem, language }) => {
                state::init_work(problem.map(|s| parse_problem(&s)).transpose()?, language)
            }
            WorkCommands::Status => state::print_work_status(),
        },
        None => Ok(()),
    }
}
fn main() {
    if let Err(e) = run() {
        println!("{}", e);
        std::process::exit(1);
    }
}
