#![recursion_limit = "1024"]
#[macro_use]
extern crate clap;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate hyper;
#[macro_use]
extern crate serde_derive;

extern crate futures;
extern crate hyper_tls;
extern crate serde_yaml;
extern crate tokio_core;

mod bonusly_bot;

mod errors {
    error_chain! {
        types {
            BBError, BBErrorKind, BBResultExt, BBResult;
        }
    }
}

use errors::*;

use std::path::Path;
use std::env;

quick_main!(run);

const CHERRY: &'static str = "
🍒 🍒 🍒 🍒 🍒 🍒 🍒 🍒 🍒 🍒 🍒 🍒 🍒 🍒 🍒 🍒 🍒 🍒 🍒 🍒 🍒 🍒 🍒 🍒 🍒 🍒 🍒 🍒 🍒
🍒 MMMMMMMMMMMMMMMMMMNyyhhhhddddddhhhyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy 🍒
🍒 :::::::::::::://+++://+oossssssoo+/:--...``````````````````````````````````````` 🍒
🍒 `````````.-:/+oossyyyyyyyyyyhhhhhhhhhhyys+/:-``````````````````````````````````` 🍒
🍒 `````-:/+osssssyyyyyysssyyyyyyyyyhhhhhhhhhhhys+:.``````````````````````````````` 🍒
🍒 .....ssso+++oossssssssyyyyyyyyyyhhhhhhhhhhhhhhhhyo:::::::::::::::::::::::::::::: 🍒
🍒 :::::++oss++///++++++ooosssssyyyyyyyyyyhhhhhhyhhhhhs+/////////////////////////// 🍒
🍒 ///////oyyso++++//////////++++ooosssssyyyyyyyhhhhhhhhs+///////////////////////// 🍒
🍒 ::::::::/shso++++++////////////+++++++oooossssssyyhhhhh+-....................... 🍒
🍒 `````````.:oso/+++++/////////////////////////++++oooooo/.``````````````````````` 🍒
🍒 ```````````.+sso//++++////////:::::::::///////+++++/:--........................` 🍒
🍒 ...........`./yhho/+o:::////////////++++++++//:--.`````````````````````````````` 🍒
🍒 `````````````./hhhhhhsssssosyy+/:--:::::---..................................... 🍒
🍒 /////////////:.oyhhhhhhhhhhhhhyhhddddddddddddddddddddddddddddddddddddddddddddddd 🍒
🍒 ddddddddddddddh:oyhhhhhhhhhhhhosyhdddddddddddddddddddddddddddddddddddddddddddddd 🍒
🍒 dddddddddddddddo+oyhhhhhhhhhhhhhysyhdddddddddddddddddddddddddddddddddddddddddddd 🍒
🍒 dddddddddddddddd::ohhhhhhhhhhhhhhyoyhhdddddddddddddddddddddddddddddddddddddddddd 🍒
🍒 ddddddddddddddddy+/yhhhhhhhhhhhhhhhhyyhhdddddddddddddddddddddddddddddddddddddddd 🍒
🍒 ddddddddddddddddd+-+hhhhhhhhhhhhhhhhhhhyhddddddddddddh+ossyyyyyyyyyyyyyyyyyyyyyy 🍒
🍒 ssssssssssssssssso+-shdhddddddddhdddddhhhhhddmhhs-:/+++ossyyyyyyyyyyyyyyyyyyyyyy 🍒
🍒 yyyyyyyyyyyyyyyyyyo`oyhdddddddddddddddddddhhhmysosyysssyyyyyyyyyyyyyyyyyyyyyyyyy 🍒
🍒 yyyyyyyyyyyyyyyyyyyy:syys.--:://///:-.````:yhhdmmhsoossyyyyyyyyyyyhhhhhhhhhhhhhh 🍒
🍒 hhhhhhhhhhhhhysy::://ooooooossssssooo++/-:oyhddmmdsssyyyyssyyyyyyyyyyhhhhhhhhhhh 🍒
🍒 hhhhhhhhhhhhs/.:/++++///:://++ooooosoooo+++oyhhhhyyyyyys+//+ossyyyyyyyhhhhhhhhhh 🍒
🍒 hhhhhhhhhhs//+o+//:--.......-:+ooooossssoo+++osyyyyyyso:--:/++ossyyyyyyhhhhhhhhh 🍒
🍒 hhhhhhhhy/:ooo+:--.......`...-/+++ooossssssooosyyyyso/-.-://++oosyyyyyyyhhhhhhhh 🍒
🍒 hhhhhhhyo/sso+/-----::::::--:://///++ossyyyysosyyso/:---:/+++oossyyyyyyyyhhhhhhh 🍒
🍒 hhhhhhhs+sssoo/////+++++++//////////+oosyyyyyssss+----:/++++oosssyyyyyyyyhhhhhhh 🍒
🍒 hhhhhhhsoysssoo++++ooo+++++++++++++ooossyyyyyyyso/--:/++++ooosssyyyyyyyyyhhhhhhh 🍒
🍒 hhhhhh++oyyyysssoooooooooooo++ooooossssyyyyyhyyss++++ooooosssyyyyyyyyyyyyhhhhhhh 🍒
🍒 hhhhhhsooysyyyyyssssssoooooooooossssyyyyyyyhhyyyssooooossssyyyyyyyyyyyyyhhhhhhhh 🍒
🍒 hhhhhhhs+ssssyyyyyyyyyssssssssssyyyyyyyyyyyyyyyyyyssssssyyyyyyyyyyyyyyyyhhhhhhhh 🍒
🍒 hhhhhhhs+oysssyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyhhhhyyyyyy 🍒
🍒 yyyyyyyyy+syyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyhyyyyyyyyyyyyyyyyyhhhhhhhyyyyyyy 🍒
🍒 yyyyyyyys+oyyyyhhhhhhhhhhhhhhhhhhhhhyyyyyyyysshhhhhhyyyyyyyyhhhhhhhhhhyyyyyyyyyy 🍒
🍒 yyyyyyyyyy:+syhhhhhhhhhhhhhhhhhhhhhhhyyyyyssohhhhhhhhhhhhhhhhhhhhhhhhyssssssssss 🍒
🍒 sssssssssss+/oyhhhhhhhhhhhhhhhhhhhhhhyyyys+:+hdddhhhhhhhhhhhhhhdhhysoooooooooooo 🍒
🍒 oooooooooooo++/syhhhhhhhhhhhhhhhhhhhhhyyo:.`.-/oyhhhhhhhhdddhhyyo+++++++++++++++ 🍒
🍒 ///////////////:/oyhhhhhhhhhhhhhhhhhys/-        `.://++++++/::------------------ 🍒
🍒 ...................:+osyyyhhyyyso+/-.``````````````````````````````````````````` 🍒
🍒                      ``..-----.``                                                🍒
🍒 🍒 🍒 🍒 🍒 🍒 🍒 🍒 🍒 🍒 🍒 🍒 🍒 🍒 🍒 🍒 🍒 🍒 🍒 🍒 🍒 🍒 🍒 🍒 🍒 🍒 🍒 🍒 🍒
";

fn run() -> BBResult<()> {
    use clap::{Arg, App};

    let matches = App::new("Bonusly Bot")
        .version("0.0.0")
        .author(crate_authors!())
        .about(CHERRY)
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Path to configuration file")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("message")
                .short("m")
                .long("message")
                .value_name("MESSAGE")
                .help("Contents of the bonusly message")
                .takes_value(true)
        )
        .get_matches();

    // TODO(ssloboda) Figure out how to put this in the None match arm.
    let mut home = env::home_dir().expect("Cannot get home directory");

    let config_file = match matches.value_of("config") {
        Some(path) => Path::new(path),
        None => {
            home.push(".bonusly.yml");
            home.as_path()
        },
    };
    let mut bbot = bonusly_bot::BonuslyBot::from_config_file(config_file)?;

    // FIXME uncomment
    match matches.value_of("message") {
        Some(msg) => bbot.give_raw_bonus(msg),
        None => bbot.give_random_bonus(),
    }

    // // FIXME remove
    // println!("{:?}", bbot.get_users()?);
    // Ok(())

}
