use std::*;
use std::io::*;
use std::time::*;
use rand::prelude::*;
use crossterm::*;

fn clear_console(){
    let cursor: TerminalCursor = cursor();
    print!("{esc}c", esc = 27 as char);
    print!("\x1B[2J\x1B[1;1H");
    print!("╔═══════════════════════════════════════════════════════════════════════════════════════════════════════════════╗\n\
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
            ║                                                     ┌─┐                                                       ║\n\
            ║                                                     └─┘                                                       ║\n\
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
            ╚═══════════════════════════════════════════════════════════════════════════════════════════════════════════════╝");
    cursor.goto(1, 1).expect("");
}

fn main() {
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
