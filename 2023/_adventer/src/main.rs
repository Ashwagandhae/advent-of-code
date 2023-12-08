pub mod resource;
use clap::{self, ValueEnum};
use resource::{print_example_input, print_example_output, print_input, print_output, ProblemPart};
pub mod update;
use update::update;
pub mod run;
use run::{run_from_name, run_from_work};
pub mod command;
pub mod language;
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
    problem: String,
    /// Test on input, example input/output, or both
    #[clap(value_enum, default_value_t = RunTarget::Input)]
    on: RunTarget,
}

#[derive(ValueEnum, Debug, Clone, Copy)]
pub enum RunTarget {
    #[clap(alias = "i")]
    Input,
    #[clap(alias = "ic")]
    InputChecked,
    #[clap(alias = "e")]
    Example,
    #[clap(alias = "ec")]
    ExampleChecked,
    #[clap(alias = "a", alias = "ei")]
    All,
    #[clap(alias = "ac", alias = "ecic")]
    AllChecked,
    #[clap(alias = "eci")]
    ExampleCheckedInput,
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
    day: i32,
}

#[derive(Args, Debug)]
pub struct OutputArgs {
    problem: String,
}

#[derive(Args, Debug)]
pub struct ExampleArgs {
    problem: String,
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
    #[clap(value_enum, default_value_t = RunTarget::ExampleCheckedInput)]
    on: RunTarget,
}

#[derive(Args, Debug)]
pub struct WorkSetArgs {
    #[clap(short, long)]
    problem: Option<String>,
    #[clap(short, long)]
    language: Option<language::Language>,
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ProblemName {
    pub day: i32,
    pub part: ProblemPart,
}

impl ProblemName {
    pub fn from_str(name: &str) -> Option<Self> {
        let (day, part) = name.split_once(".")?;
        let day = day.parse::<i32>().ok()?;
        let part = part.parse::<i32>().ok()?;
        Some(Self {
            day,
            part: match part {
                0 => ProblemPart::Zero,
                1 => ProblemPart::One,
                _ => return None,
            },
        })
    }
    pub fn to_string(&self) -> String {
        format!("{}.{}", self.day, self.part)
    }
}

impl Into<String> for ProblemName {
    fn into(self) -> String {
        self.to_string()
    }
}

fn problem_name(s: &str) -> ProblemName {
    ProblemName::from_str(s).expect("invalid problem name")
}
fn main() {
    let args = Cli::parse();
    let res = match args.command {
        Some(Commands::Update) => update(),
        Some(Commands::Run(RunArgs { problem, on })) => run_from_name(problem_name(&problem), on),
        Some(Commands::Resource(command)) => match command {
            ResourceCommands::Input(InputArgs { day }) => print_input(day),
            ResourceCommands::Output(OutputArgs { problem }) => {
                print_output(problem_name(&problem))
            }
            ResourceCommands::ExampleInput(ExampleArgs { problem }) => {
                print_example_input(problem_name(&problem))
            }
            ResourceCommands::ExampleOutput(ExampleArgs { problem }) => {
                print_example_output(problem_name(&problem))
            }
        },
        Some(Commands::Work(command)) => match command {
            WorkCommands::Run(WorkRunArgs { on }) => run_from_work(on),
            WorkCommands::Set(WorkSetArgs { problem, language }) => {
                state::set_work(problem.map(|s| problem_name(&s)), language)
            }
            WorkCommands::Save => state::save_work(),
            WorkCommands::Next => state::save_and_next_work(),
            WorkCommands::Status => state::print_work_status(),
            WorkCommands::Open => state::open_work(false),
            WorkCommands::Clear => state::clear_work(),
            WorkCommands::Init(WorkSetArgs { problem, language }) => {
                state::init_work(problem.map(|s| problem_name(&s)), language)
            }
        },
        None => Ok(()),
    };
    if let Err(e) = res {
        panic!("{}", e);
    }
}
