use colors_rs::Color;
use equ::{Core, Types};
use rustyline::{Editor, history::FileHistory};
use sbew::{App, Apps, Args, ArgsFn, Groups, PATH, Run};
use std::{collections::HashMap, env, process::Command};

use crate::Help;

pub fn main(editor: &mut Editor<Help, FileHistory>, text: &str, name: &str, path: &PATH) {
    let mut apps = Apps::new();
    let mut input = match editor.readline(text) {
        Ok(v) => v,
        Err(_) => return,
    };

    let cd = apps.add(App {
        name: "cd".to_string(),
        len: 1,
        main: cd,
    });

    let alias_list = apps.add(App {
        name: "alias".to_string(),
        len: 0,
        main: list,
    });

    let alias_add = apps.add(App {
        name: "alias-add".to_string(),
        len: 2,
        main: add,
    });

    let alias_rem = apps.add(App {
        name: "alias-rem".to_string(),
        len: 1,
        main: del,
    });

    let say = apps.add(App {
        name: "say".to_string(),
        len: 1,
        main: say,
    });

    let help = apps.add(App {
        name: "help".to_string(),
        len: 0,
        main: help,
    });

    let alias = match equ::new("/config/shell/alias").load() {
        Some(v) => v,
        None => return,
    };

    editor.add_history_entry(input.trim()).unwrap();
    editor.save_history("/config/shell/history.txt").unwrap();

    for i in alias {
        match i.1 {
            Types::Vector(_) => continue,
            Types::String(v) => {
                if input.contains(&i.0) && apps.not(&input) {
                    input = input.replace(&i.0.as_str(), &v);
                }
            }
        }
    }

    for cmd in input.split("&&") {
        match cmd.trim() {
            "" => {}
            "me" => println!("{}", &name),
            "pwd" => println!("{:?}", env::current_dir().unwrap()),
            "clear" => editor.clear_screen().unwrap(),
            x if cd.name(x) => cd.run(x),
            x if alias_list.name(x) => alias_list.run(x),
            x if alias_add.name(x) => alias_add.run(x),
            x if alias_rem.name(x) => alias_rem.run(x),
            x if say.name(x) => say.run(x),
            x if help.name(x) => help.run(x),
            _ => {
                let mut args: Vec<&str> = cmd.split_whitespace().collect();
                let name = args.remove(0);

                for i in &path.data {
                    if name == i.0.trim() {
                        for cmd2 in i.1.split("&&") {
                            let mut x: Vec<&str> = cmd2.split_whitespace().collect();
                            let prg = x.remove(0);

                            for i in x.iter().rev() {
                                args.insert(0, i);
                            }

                            match Command::new(prg).args(&args).status() {
                                Ok(_) => {}
                                Err(_) => println!("Bin or alias not found"),
                            }
                        }
                    }
                }
            }
        }
    }
}

fn cd(args: Args) {
    let i = args.arg(1).unwrap();

    match env::set_current_dir(&i.value) {
        Err(_) => {
            if env::current_dir()
                .unwrap()
                .ends_with(format!("/{}", i.value))
            {
            } else {
                println!("This is not a dir")
            }
        }
        _ => {}
    }
}

fn list(_: Args) {
    let alias = match equ::new("/config/shell/alias").load() {
        Some(v) => v,
        None => return,
    };

    for i in alias {
        let cmd = match i.1 {
            Types::String(v) => v,
            Types::Vector(_) => continue,
        };
        println!("{}: {}", i.0.purple(), cmd);
    }
    println!(
        "\nType {} <name> <alias> to create a new alias\nType {} <name> to delete an alias",
        "alias-add".purple(),
        "alias-rem".purple(),
    );
}

fn add(args: Args) {
    let mut map: HashMap<String, Types> = HashMap::new();
    let name = args.arg(1).unwrap().value.clone();

    if name.contains(" ") {
        println!("Bad alias");
        return;
    }

    map.insert(name, Types::String(args.arg(2).unwrap().value.clone()));

    match equ::new("/config/shell/alias").add(&map) {
        Ok(_) => println!("Success: type alias-list to see new alias"),
        Err(e) => println!("{}", e),
    };
}

fn del(args: Args) {
    match equ::new("/config/shell/alias").del(&args.arg(1).unwrap().value) {
        Ok(_) => println!("Success: type alias-list to see new alias"),
        Err(e) => println!("{}", e),
    };
}

fn say(args: Args) {
    let word = args.arg(1).unwrap();
    println!("{}", word.value)
}

fn help(_: Args) {
    println!(
        "wtf commands:\n   {} - help menu\n   {} <path> - change dir\n   {} <text> - print your phrase\n\nAlias:\n   {} - print aliases\n   {} <name> <command> - add alias\n   {} <name> - remove alias",
        "help".purple().underlined().bold(),
        "cd".purple().underlined().bold(),
        "say".purple().underlined().bold(),
        "alias".purple().underlined().bold(),
        "alias-add".purple().underlined().bold(),
        "alias-rem".purple().underlined().bold()
    )
}
