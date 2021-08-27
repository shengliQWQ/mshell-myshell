use std::io::*;
use std::path::*;
use std::env;
use std::process::*;
fn main(){
    loop {
        print!("$ ",);
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        // i write it is so hard
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
                "exit" => return,
                ">" => {
                    let workspace = env::current_dir().unwrap();//read the workspace
                    println!("{}",workspace.display());
                },
                command => {
                    let stdin = previous_command
                        .map_or(
                            Stdio::inherit(),
                            |output: Child| Stdio::from(output.stdout.unwrap())
                        );
                    let stdout = if commands.peek().is_some() {
                        // ok away
                        Stdio::piped()
                    } else {
                        // no more command
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
            // When the command is finish then loop
            final_command.wait().unwrap();
        }

    }
}
