use std::borrow::Cow;
use std::env;
use std::ffi::OsStr;
mod core;
use colors_rs::Color;
use equ::Core;
use equ::Types;
use rustyline::Completer;
use rustyline::Editor;
use rustyline::Helper;
use rustyline::Hinter;
use rustyline::Validator;
use rustyline::completion::FilenameCompleter;
use rustyline::config::Configurer;
use rustyline::highlight::Highlighter;
use rustyline::hint::HistoryHinter;
use rustyline::history::FileHistory;
use sbew::PATH;

#[derive(Helper, Completer, Hinter, Validator)]
pub struct Help {
    #[rustyline(Completer)]
    comp: FilenameCompleter,
    #[rustyline(Hinter)]
    hint: HistoryHinter,
}

impl Highlighter for Help {
    fn highlight_hint<'h>(&self, hint: &'h str) -> std::borrow::Cow<'h, str> {
        Cow::Owned(hint.purple().italic())
    }

    fn highlight<'l>(&self, line: &'l str, _: usize) -> Cow<'l, str> {
        let cmd: Vec<&str> = line.split("&&").collect();
        let mut res = line.to_string();

        let path = PATH::new().data;
        let alias = equ::new("/config/shell/alias").load().unwrap();

        let mut bins: Vec<String> = vec![
            "cd".into(),
            "alias".into(),
            "alias-add".into(),
            "alias-rem".into(),
            "help".into(),
            "say".into(),
            "clear".into(),
            "pwd".into(),
            "me".into(),
        ];

        for i in alias {
            bins.push(i.0);
        }

        for i in path {
            bins.push(i.0);
        }

        for i in cmd {
            let inp: Vec<&str> = i.split_whitespace().collect();
            for c in inp {
                if bins.contains(&c.to_string()) {
                    res = res.replace(c, &c.green().bold());
                }
            }
        }
        Cow::Owned(res)
    }

    fn highlight_char(&self, _: &str, _: usize, _: rustyline::highlight::CmdKind) -> bool {
        true
    }
}

fn main() {
    let mut editor: Editor<Help, FileHistory> = Editor::new().unwrap();
    editor.load_history("/config/shell/history.txt").unwrap();
    editor.set_max_history_size(100).unwrap();
    editor.set_helper(Some(Help {
        comp: FilenameCompleter::new(),
        hint: HistoryHinter::new(),
    }));

    let user_file = equ::new("/config/user.dat").load();

    let user_data = match user_file {
        Some(v) => v,
        None => return,
    };

    let name: &String = match user_data.get("name") {
        Some(v) => match v {
            Types::String(v) => v,
            Types::Vector(_) => return,
        },
        None => return,
    };

    let path = PATH::new();

    print!("\x1b[2J\x1b[3J\x1b[H");
    println!("Welcome to {}, {}!\n", "w1nOS".purple(), &name.blue());

    loop {
        let cur_temp = env::current_dir().unwrap();
        let cur: &str = cur_temp
            .file_name()
            .unwrap_or(OsStr::new(&name))
            .to_str()
            .unwrap();
        let text = format!("{}{}{} ) ", name.purple(), "@".purple(), cur.purple());
        core::main(&mut editor, &text, name, &path);
    }
}
