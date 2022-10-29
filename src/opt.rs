use std::collections::{HashMap, BTreeMap};
use std::io::{Error, ErrorKind};

pub enum OptArg<'a> { None, Mandatory(&'a str) }
pub type OptCheck<'a> = BTreeMap<u8, (OptArg<'a>, &'a str)>;

pub fn get_opt_per<'a>(args: &'a [String], opt_check: &OptCheck<'a>) -> 
Result<(HashMap<u8, &'a str>, Vec<&'a str>), Error> {
    let (mut options, mut operands): (HashMap<u8, &str>, Vec<&str>) = 
    (HashMap::new(), Vec::new());

    for (k, v) in opt_check {
        match v.0 {
            OptArg::Mandatory(default_arg) => {
                if default_arg != "" {
                    options.insert(*k, default_arg);
                }
            }, 
            OptArg::None => (),
        }
    }

    let mut i = 0;
    while i < args.len() {
        let arg = &args[i];
        if arg == "--" {
            for e in &args[(i+1)..] {
                operands.push(e.as_str());
            }
            break
        }
        let arg_b = args[i].as_bytes();
        if arg_b.len() >= 2 && arg_b[0] == b'-' {
            let arg_b = &arg_b[1..arg_b.len()];
            for (ii, s_opt) in arg_b.iter().enumerate() {
                match opt_check.get(s_opt) {
                    Some(check) => {
                        match &check.0 {
                            OptArg::None => {
                                options.insert(*s_opt, "");
                            },
                            OptArg::Mandatory(_) => {
                                if ii+2 > arg_b.len() {
                                    if i+2 > args.len() {
                                        return Err(Error::new(ErrorKind::Other,
                                            format!("option requires an argument -- '{}'", *s_opt as char)));
                                    } 
                                    options.insert(*s_opt, args[i+1].as_str());
                                    i += 1;
                                } else {
                                    options.insert(*s_opt, &args[i][(ii+2)..(1+arg_b.len())]);
                                    break;
                                }
                            },
                        }
                    },
                    None => return Err(Error::new(ErrorKind::Other, 
                        format!("invalid option -- '{}'", *s_opt as char))),
                }
            }
        } else {
            operands.push(arg);
        }
        i += 1;
    }
    Ok((options, operands))
}

pub fn usage(opt_check: &OptCheck) -> String {
    let mut ret = Vec::new();
    if !opt_check.is_empty() {
        ret.push(String::from("OPTIONS: "));
    }
    for (_, v) in opt_check {
        ret.push(format!("\t{}", v.1));
    }
    ret.join("\n")
}