use std::io::*;
use std::path::*;
use std::env;
use std::process::*;
use colored::Colorize;
fn main(){
    loop {
        print!("{} ",">".green());
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut commands = input.trim().split(" | ").peekable();
        let mut previous_command = None;
        while let Some(command) = commands.next()  {
            let mut parts = command.trim().split_whitespace();
            let command = parts.next().unwrap();
            let args = parts;
            match command {
                "cd" => {
                    let new_dir = args.peekable().peek()
                        .map_or("/", |x| *x);
                    let root = Path::new(new_dir);
                    if let Err(e) = env::set_current_dir(&root) {
                        eprintln!("{}", e);
                    }
                    previous_command = None;
                },
                "clear" => {
                    print!("\x1b[2J");stdout().flush().unwrap();
                    print!("\x1b[H");stdout().flush().unwrap();
                },
                "version" => {
                    println!("{} {}","version".white(),"0.05".yellow());
                    println!("{}{}{}{}","by".yellow(),"WodeShengli".green(),"and".yellow(),"you".green())
                },
                "exit" => return,
                ">" => {
                    let workspace = env::current_dir().unwrap();
                    println!("{}",workspace.display());
                },"help" => {
                    println!("{}{}{}",">".green(),"----","Show workspace".yellow());
                    println!("{}{}{}","cd".blue(),"----","opendir".yellow());
                    println!("{}{}{}","ls".purple(),"----","filelist".yellow());
                    println!("{}{}{}","version".cyan(),"----","print the version in the shell".yellow());
                    println!("{}{}{}","clear".red(),"----","clear the command line".yellow());
                },"ls" => {
                    let path = Path::new(".");
                    if path.is_dir(){
                        match path.read_dir() {
                            Ok(de) => {
                                println!("{}","filename".green());
                                for item in de {
                                    match item{
                                        Ok(entry) => {
                                            match entry.metadata(){
                                                Ok(_metadata) => {
                                                    let name = String::from(entry.file_name().to_str().unwrap_or(""));
                                                    print!("{}",name);
                                                    match name.len() {
                                                        0..=9 =>print!("\t\t"),
                                                        9..=20 => print!("\t"),
                                                        _ => print!("\t")
                                                    }
                                                    println!("");
                                                },
                                                Err(e) => {
                                                    println!("error{:?}",e);
                                                },
                                            }
                                        },
                                        Err(e) => {
                                            println!("error{:?}",e);
                                        },
                                    }
                                }
                            },
                            Err(e) => {
                                println!("error{:?}",e);
                            }
                        }
                    }
                },
                command => {
                    let stdin = previous_command
                        .map_or(
                            Stdio::inherit(),
                            |output: Child| Stdio::from(output.stdout.unwrap())
                        );
                    let stdout = if commands.peek().is_some() {
                        Stdio::piped()
                    } else {
                        Stdio::inherit()
                    };
                    let output = Command::new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();
                    match output {
                        Ok(output) => { previous_command = Some(output); },
                        Err(e) => {
                            previous_command = None;
                            eprintln!("{}", e);
                        },
                    };
                }
            }
        }
        if let Some(mut final_command) = previous_command {
            final_command.wait().unwrap();
        }
    }
}
