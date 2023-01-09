use atty::Stream;
use clap::Parser;
use envbed::replace_dollar_braces::{replace_dollar_braces_with_hashmap, replace_dollar_braces_with_hashmap_parallel};
use envbed::replace_double_braces::replace_double_braces_with_hashmap;
use std::io::{self};
use std::sync::{Arc, Mutex};
use std::{
    env,
    io::{Read, Write},
};
mod model;
mod replace_dollar_braces;
mod replace_double_braces;
use rustc_hash::FxHasher;
use std::{collections::HashMap, hash::BuildHasherDefault};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// specifies a target file instead of piped stdin
    #[arg(short, long, default_value = "")]
    file: String,

    /// uses env file instead of os environment variables
    #[arg(short = 'e', long, default_value = "")]
    env_from_file: String,

    /// filters envvars with this prefix (recommended for low security risks)
    #[arg(short = 'p', long, default_value = "")]
    env_prefix: String,

    /// uses {{FOO}} syntax instead of ${FOO} (avoid conflicts with OS default syntax)
    #[arg(long, default_value_t = false)]
    template_syntax_double_braces: bool,

    /// parallel replacement (consider overhead of parallelism. this is for a huge file)
    #[arg(long, default_value_t = false)]
    parallel: bool,

    /// override a target file (--file)
    #[arg(short = 'w', long, default_value_t = false)]
    override_file: bool,

    /// specifies a output file instead of stdout
    #[arg(short, long, default_value = "")]
    out: String,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    // 1. Input
    // 1.1 Input Target Text
    let mut target = String::new();
    // get from file
    if !args.file.is_empty() {
        let mut f = std::fs::File::open(&args.file)?;
        f.read_to_string(&mut target)?;
    } else {
        // get from pipe
        if !atty::is(Stream::Stdin) {
            io::stdin().read_to_string(&mut target)?;
        }
    }
    // 1.2 Get Env Vars
    let mut envvars: HashMap<String, String, BuildHasherDefault<FxHasher>> = HashMap::default();
    if !args.env_from_file.is_empty() {
        // get env from file
        let mut envfile = String::new();
        let mut fe = std::fs::File::open(args.env_from_file)?;
        fe.read_to_string(&mut envfile)?;
        // parse text
        for t in envfile.lines() {
            let kv: Vec<&str> = t.split('=').collect();
            if kv.len() == 2 {
                if !args.env_prefix.is_empty() && !kv[0].to_owned().starts_with(&args.env_prefix) {
                    continue;
                }
                envvars.insert(kv[0].to_owned(), kv[1].to_owned());
            }
        }
    } else {
        // get env from os
        for (key, value) in env::vars() {
            if !args.env_prefix.is_empty() && !key.starts_with(&args.env_prefix) {
                continue;
            }
            envvars.insert(key, value);
        }
    }

    // 2. Replace
    if args.template_syntax_double_braces {
        target = replace_double_braces_with_hashmap(&envvars, &target)
    } else {
        if args.parallel {
          target = replace_dollar_braces_with_hashmap_parallel(&envvars, target.to_owned(), 24)
        } else {
          target = replace_dollar_braces_with_hashmap(&envvars, &target)
        }
    }

    // 3. Output
    // write output to file
    if !args.out.is_empty() {
        let mut out_file = std::fs::File::create(args.out)?;
        out_file.write_all(target.as_bytes())?;
        return Ok(());
    }
    // write output to file
    if !args.out.is_empty() {
        let mut out_file = std::fs::File::create(args.out)?;
        out_file.write_all(target.as_bytes())?;
        return Ok(());
    }
    if args.override_file && !&args.file.is_empty() {
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&args.file)?;
        f.write_all(target.as_bytes())?;
        f.flush()?;
        return Ok(());
    }

    // write output to stdout
    print!("{}", target);
    Ok(())
}
