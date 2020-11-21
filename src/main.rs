use std::collections::HashMap;
use serde_derive::{Serialize, Deserialize};
use clap::Clap;

#[derive(Clap)]
#[clap(version = "1.0", author = "Olivier D. <olivier.dolle@protonmail.com>")]
struct Opts {    
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    Add(Add),
    Draw(Draw),
    Reset(Reset),
    Status(Status),
}

#[derive(Clap)]
struct Add {
    // TODO: add several users at once
    user: String
}

#[derive(Clap)]
struct Draw {
}

#[derive(Clap)]
struct Reset {
}

#[derive(Clap)]
struct Status {
}

#[derive(Debug, Serialize, Deserialize)]
struct State {
    user_map: HashMap<String, u8>,
}

impl ::std::default::Default for State {
    fn default() -> Self {
        Self {
            user_map: HashMap::<String, u8>::new()
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut state: State = confy::load("wtii")?;

    let user_map = &mut state.user_map;

    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Add(t) => {
            add_user(t.user, user_map, None);
            println!("{:?}", user_map);
        }
        SubCommand::Draw(_) => {
            match draw(user_map) {
                Some(user) => println!("{:?}", user),
                None => println!("No user to draw."),
            }
        }
        SubCommand::Reset(_) => {
            user_map.clear();
        }
        SubCommand::Status(_) => {
            println!("{:?}", user_map);
        }
    }
    
    confy::store("wtii", state)?;
    Ok(())
}


fn add_user(user: String, user_map: &mut HashMap<String, u8>, value: Option<u8>) {
    user_map.insert(user, value.unwrap_or(0));
}

fn draw(user_map: &mut HashMap<String, u8>) -> Option<String> {
    let mut min: u8 = 100;
    let mut candidates: Vec<String> = vec![] ;
    for (k, v) in user_map.iter() {
        if *v < min {
            min = *v;
            candidates = vec![k.clone()];
        } else if *v == min {
            candidates.push(k.clone());
        } else {}
    }

    let chosen_one = candidates.pop()?;

    let chosen_one_value = user_map.entry(chosen_one.clone()).or_insert(0);
    *chosen_one_value += 1;

    Some(chosen_one)
}

#[test]
fn test_add_user() {
    let mut user_map = HashMap::<String, u8>::new();
    add_user("olivier".into(), &mut user_map, Some(1));

    assert_eq!(*user_map.get("olivier".into()).unwrap(), 1)
}
