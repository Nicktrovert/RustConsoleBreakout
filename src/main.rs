use std::*;
use std::collections::HashSet;
use std::io::*;
use std::time::*;
use rand::prelude::*;
use crossterm::*;

struct Obstacle{
    coords: (i16, i16),
    content: string,
}

impl Obstacle{
    fn get_content() -> string {
        return "████";
    }

    fn new() -> Obstacle {
        return Obstacle{
            coords: (0, 0),
            content: "████",
        }
    }
}


static console_interface: &str = "╔═══════════════════════════════════════════════════════════════════════════════════════════════════════════════╗\n\
                                  ║ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ║\n\
                                  ║ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ║\n\
                                  ║ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ║\n\
                                  ║ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ║\n\
                                  ║ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ║\n\
                                  ║ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ║\n\
                                  ║ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ████ ║\n\
                                  ║                                                                                                               ║\n\
                                  ║                                                                                                               ║\n\
                                  ║                                                                                                               ║\n\
                                  ║                                                                                                               ║\n\
                                  ║                                                                                                               ║\n\
                                  ║                                                                                                               ║\n\
                                  ║                                                                                                               ║\n\
                                  ║                                                                                                               ║\n\
                                  ║                                                       O                                                       ║\n\
                                  ║                                                                                                               ║\n\
                                  ║                                                                                                               ║\n\
                                  ║                                                                                                               ║\n\
                                  ║                                                                                                               ║\n\
                                  ║                                                                                                               ║\n\
                                  ║                                                                                                               ║\n\
                                  ║                                                                                                               ║\n\
                                  ║                                                                                                               ║\n\
                                  ║                                                                                                               ║\n\
                                  ║                                                                                                               ║\n\
                                  ║                                                     ▀▀▀▀                                                      ║\n\
                                  ╠───────────────────────────────────────────────────────────────────────────────────────────────────────────────╣\n\
                                  ║   <                                                                                                       >   ║\n\
                                  ╚═══════════════════════════════════════════════════════════════════════════════════════════════════════════════╝";

fn clear_console(){
    let cursor: TerminalCursor = cursor();
    print!("{esc}c", esc = 27 as char);
    print!("\x1B[2J\x1B[1;1H");
    unsafe { print!("{}", console_interface.to_string()); }
    cursor.goto(1, 1).expect("");
}

fn main() {
    let mut obstacleList: Vec<Vec<Obstacle>> = Vec::new();
    for _i in 1..13{
        let mut obstacleListElement: Vec<Obstacle> = Vec::new();
        for _j in 1..113 / 4 - 2{
            obstacleListElement.push(Obstacle::new());
        }
        obstacleList.push(obstacleListElement);
    }
    let cursor: TerminalCursor = cursor();
    while terminal().size().unwrap() != (113, 31){
        cursor.goto(0, 0).expect("");
        println!("Current Terminal Size: X={0};Y={1}", terminal().size().unwrap().0, terminal().size().unwrap().1);
        println!("Expected Terminal Size: X={0};Y={1}", 113, 31);
        write!(stdout(), "Please resize Terminal to start the game...").expect("");
        stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(120));
        print!("{esc}c", esc = 27 as char);
        print!("\x1B[2J\x1B[1;1H");
    }
    clear_console();
    loop{
        break;
    }
    clear_console();
    cursor.goto(39, 29).expect("");
    write!(stdout(), "Press enter to close console...").expect("");
    stdout().flush().unwrap();
    TerminalInput::new().read_char().expect("");
}
