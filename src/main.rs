// Copyright 2018 Johannes M. Griebler
//
// This file is part of mycon.
//
// mycon is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// mycon is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with mycon.  If not, see <https://www.gnu.org/licenses/>.

use std::fs::File;
use std::io::{self, Read, Write};
use std::process;
use std::time::{Duration, Instant};
use std::thread;

use ansi_term::Colour;
use clap::{App, Arg, crate_version};

use mycon::*;

macro_rules! print_error {
    ($fmt:expr $(, $arg:expr)*) => {
        eprintln!(concat!("{} ", $fmt), Colour::Red.bold().paint("error:"), $($arg),*);
    };
}

macro_rules! print_info {
    ($fmt:expr $(, $arg:expr)*) => {
        eprintln!(concat!("{} ", $fmt), Colour::Cyan.paint("mycon:"), $($arg),*);
    };
}

fn run() -> i32 {
    let t0 = Instant::now();

    let matches = App::new("mycon")
        .version(crate_version!())
        .author("Johannes M. Griebler <johannes.griebler@gmail.com>")
        .about("Befunge-98 interpreter")
        .arg(Arg::with_name("SOURCE_FILE")
             .help("the source file to be interpreted")
             .required(true))
        .arg(Arg::with_name("TIME")
             .help("report the wall-clock execution time")
             .short("t")
             .long("time"))
        .arg(Arg::with_name("VERBOSITY")
             .help("trace command execution")
             .short("v")
             .long("verbose"))
        .arg(Arg::with_name("SLEEP")
             .help("duration to sleep after each tick, in milliseconds")
             .short("s")
             .long("sleep")
             .takes_value(true)
             .value_name("time"))
        .get_matches();

    let mut timing = if matches.is_present("TIME") {
        let t1 = Instant::now();
        Some((t0, t1))
    } else {
        None
    };

    let path = matches.value_of("SOURCE_FILE").unwrap();

    let code = {
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(e) => {
                print_error!("The file \"{}\" could not be opened: {}", path, e);
                return 1;
            }
        };

        let mut buf = Vec::new();
        if let Err(e) = file.read_to_end(&mut buf) {
            print_error!("The file \"{}\" could not be read: {}", path, e);
            return 1;
        }

        String::from_utf8(buf).unwrap_or_else(|e| {
            e.into_bytes().into_iter().map(char::from).collect()
        })
    };

    let mut config = Config::new();

    if matches.is_present("VERBOSITY") {
        config = config
            .trace(true)
            .trace_format(|trace| {
                let id = Colour::Green.paint(trace.id());
                let cmd = Colour::Purple.paint(trace.command());
                let pos = Colour::Blue.paint(trace.position());
                let stacks = Colour::Yellow.paint(trace.stacks());
                print_info!("IP {} hit {} at {}; stacks: {}", id, cmd, pos, stacks);
            });
    }

    let mut prog = Program::read(&code).config(config);

    if let Some((t0, t1)) = timing {
        let t2 = Instant::now();
        let elapsed = t2.duration_since(t1);

        print_info!("loaded program in {:?}", elapsed);

        timing = Some((t0, t2));
    }

    let exit = {
        if let Some(n) = matches.value_of("SLEEP").and_then(|s| s.parse::<u64>().ok()) {
            let dur = Duration::from_millis(n);

            loop {
                prog.step_all();

                if let Some(exit) = prog.exit_status() {
                    break exit;
                }

                thread::sleep(dur);
            }
        } else {
            prog.run()
        }
    };

    if let Some((t0, t2)) = timing {
        let exec = t2.elapsed();
        let _ = io::stdout().flush();
        let total = t0.elapsed();

        print_info!("executed in {:?}", exec);
        print_info!("total time {:?}", total);
    }

    exit
}

fn main() {
    let exit = run();

    process::exit(exit);
}
