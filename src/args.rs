pub fn parse_args(mut args: std::env::Args) {
    let pname = args.next().unwrap();

    let command = args.next();

    if command.is_none() || [ "--help", "-h", "help" ].contains(&command.clone().unwrap().as_str()) {
        help_msg(pname, command.clone());
    }

    let command = command.unwrap();

    match command.as_str() {
        "run" => {}
        "eval" => {}
        "repl" => {}
        "version" => {
            println!("Dyn 0.1.0")
        }
        _ => {}
    }
}

fn help_msg(pname: String, topic: Option<String>) {
    if let Some(_topic) = topic {
    } else {
        println!("Purely functional, single expression language.
Usage: {pname} run [FILE] [OPTIONS]
       {pname} eval [EXPR] [OPTIONS]
       {pname} help [COMMAND]
       {pname} repl [OPTIONS]
       {pname} version");
    }
}
