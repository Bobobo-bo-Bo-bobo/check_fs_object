mod check;
mod constants;
mod fsobj;
mod metadata;
mod usage;

use std::collections::HashSet;

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    let mut options = getopts::Options::new();
    let mut objlist = HashSet::new();
    let mut critical = Vec::<String>::new();
    let mut warning = Vec::<String>::new();
    let mut ok = Vec::<String>::new();

    options.optflag("h", "help", "Show help");
    options.optflag("f", "follow", "Follow symbolic link");
    options.optflag(
        "C",
        "critical",
        "If type/mode doesn't match, report critical state",
    );
    options.optflag("V", "version", "Show version");
    options.optopt("t", "type", "", "Type");
    options.optopt("m", "mode", "", "Mode");

    let opts = match options.parse(&argv[1..]) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: Can't parse command line arguments: {}", e);
            std::process::exit(constants::STATUS_UNKNOWN);
        }
    };

    if opts.opt_present("h") {
        usage::show_usage();
        std::process::exit(constants::STATUS_OK);
    }

    if opts.opt_present("V") {
        usage::show_version();
        std::process::exit(constants::STATUS_OK);
    }

    let report_critical = opts.opt_present("C");
    let follow_symlink = opts.opt_present("f");

    let ftype = match opts.opt_str("t") {
        Some(v) => v,
        None => String::new(),
    };

    let mode: i32 = match opts.opt_str("m") {
        Some(v) => match i32::from_str_radix(&v, 8) {
            Ok(vv) => vv,
            Err(e) => {
                eprintln!("Error: Can't convert {} to an integer: {}", v, e);
                std::process::exit(constants::STATUS_UNKNOWN);
            }
        },
        None => -1,
    };

    if ftype.is_empty() {
        eprintln!("Error: Type option is required");
        std::process::exit(constants::STATUS_UNKNOWN);
    }

    match ftype.as_str() {
        "F" | "b" | "c" | "d" | "f" | "l" | "s" => {}
        _ => {
            eprintln!("Error: Invalid type {}", ftype);
            std::process::exit(constants::STATUS_UNKNOWN);
        }
    };

    if mode == -1 {
        eprintln!("Error: Mode options is required");
        std::process::exit(constants::STATUS_UNKNOWN);
    }

    let objs = opts.free;

    if objs.is_empty() {
        eprintln!("Error: No objects to check");
        std::process::exit(constants::STATUS_UNKNOWN);
    }

    // remove duplicates
    for obj in objs {
        objlist.insert(obj);
    }

    for obj in objlist.iter() {
        let meta = match metadata::get_metadata(obj, follow_symlink) {
            Ok(v) => v,
            Err(e) => {
                critical.push(format!("Can't read meta data for {}: {}", obj, e));
                continue;
            }
        };

        let fsobj = fsobj::FSObj {
            name: obj.to_string(),
            meta,
            requested_type: ftype.to_string(),
            mode: mode as u32,
        };
        check::metadata(fsobj, &mut ok, &mut warning, &mut critical, report_critical);
    }

    if !critical.is_empty() {
        println!("CRITICAL - {}", critical.join(", "));
        std::process::exit(constants::STATUS_CRITICAL);
    }
    if !warning.is_empty() {
        println!("WARNING - {}", warning.join(", "));
        std::process::exit(constants::STATUS_WARNING);
    }
    if !ok.is_empty() {
        println!("OK - {}", ok.join(", "));
        std::process::exit(constants::STATUS_OK);
    }

    println!("Neither critical, warning nor ok set");
    std::process::exit(constants::STATUS_UNKNOWN);
}
