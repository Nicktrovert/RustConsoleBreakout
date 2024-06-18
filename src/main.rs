extern crate winapi;
extern crate user32;
extern crate kernel32;
use std::*;
use std::collections::HashSet;
use std::future::poll_fn;
use std::io::*;
use std::ptr::read;
use std::time::*;
use rand::prelude::*;
use crossterm::*;
use crossterm::Result;
use colored::Colorize;
use crossterm::cursor::{MoveTo, position};
use crossterm::event::{EnableMouseCapture, Event, KeyCode, poll};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

struct Obstacle{
    coords: (i16, i16),
    content: String,
}

impl Obstacle{
    fn get_default_content() -> String {
        return "████".to_string();
    }

    fn new() -> Obstacle {
        return Obstacle{
            coords: (0, 0),
            content: Obstacle::get_default_content().to_string(),
        }
    }
}


static console_interface: &str = "╔═══════════════════════════════════════════════════════════════════════════════════════════════════════════════╗\n\
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
    print!("{esc}c", esc = 27 as char);
    print!("\x1B[2J\x1B[1;1H");
    unsafe { print!("{}", console_interface.to_string()); }
    stdout().execute(MoveTo(1, 1)).expect("");
}

fn InitializeNew_SetOf_Obstacles(rows: i32, columns: i32) -> Vec<Vec<Obstacle>> {
    let mut obstacleList: Vec<Vec<Obstacle>> = Vec::new();
    for i in 1..rows{
        let mut obstacleListElement: Vec<Obstacle> = Vec::new();
        for j in 1..columns / (Obstacle::get_default_content().to_string().chars().count() as i32) - 5{
            let new_obstacle = Obstacle{
                coords: ((j * ((Obstacle::get_default_content().to_string().chars().count() as i32) + 1) - 3) as i16, i as i16),
                content: Obstacle::get_default_content(),
            };
 obstacleListElement.push(new_obstacle);
        }
        obstacleList.push(obstacleListElement);
    }
    return obstacleList;
}

fn Render_Game(obstacles_list: &Vec<Vec<Obstacle>>){
    Render_Canvas();
    Render_Obstacles(obstacles_list);

}

fn Render_Obstacles(obstacles_list: &Vec<Vec<Obstacle>>){
    let color_vec = vec!["red", "green", "blue", "yellow", "magenta", "orange"];
    let mut color_iterator = 0;
    for Obstacles in obstacles_list {
        for obstacle in Obstacles{
            stdout().execute(crossterm::cursor::MoveTo(obstacle.coords.0 as u16, obstacle.coords.1 as u16)).expect("");
            write!(stdout(), "{}", obstacle.content.color(color_vec[color_iterator % color_vec.iter().count()])).expect("");
            color_iterator += 1;
        }
        color_iterator += 1;
    }
    stdout().flush().unwrap();
}

fn Render_Canvas(){
    let time_now: SystemTime = SystemTime::now();
    let mut stringToPrint = "".to_string();
    stdout().execute(MoveTo(0, 0)).expect("");
    for i in 0..terminal::size().unwrap().1 - 1{
        for j in 0..terminal::size().unwrap().0 - 1{

            let mut charToPrint = ' ';

            if ((i == 0 || i == terminal::size().unwrap().1 || i == (terminal::size().unwrap().1) - 3)){
                if (j != 0 && j != terminal::size().unwrap().0 && i != terminal::size().unwrap().1 - 1){
                    stringToPrint += ("═").as_str();
                    continue;
                }
                else if (i == terminal::size().unwrap().1 - 3 && j == 0){
                    charToPrint = '╠';
                }
                else if (i == terminal::size().unwrap().1 - 3 && j == terminal::size().unwrap().0){
                    charToPrint = '╣';
                }
                if (i == 0 && j == 0){
                    charToPrint = '╔';
                }
                else if (i == 0  && j == terminal::size().unwrap().0){
                    charToPrint = '╗';
                }
                if (i == terminal::size().unwrap().1 && j == 0){
                    charToPrint = '╚';
                }
                else if (i == terminal::size().unwrap().1 && j == terminal::size().unwrap().0){
                    charToPrint = '╝';
                }
            }
            else if (j == 0 || j == terminal::size().unwrap().0){
                charToPrint = '║';
            }

            stringToPrint += charToPrint.to_string().as_str();
        }
        if (i != terminal::size().unwrap().1){
            stringToPrint += "\n";
        }
    }
    write!(stdout(), "{}", stringToPrint).expect("");
    stdout().flush().unwrap();
    let _ = write!(stdout(), "{0} seconds passed", time_now.elapsed().unwrap().as_secs_f64());
}

fn print_events() -> Result<()> {
        // Wait up to 1s for another event
        if poll(Duration::from_millis(10))? {
            // It's guaranteed that read() wont block if `poll` returns `Ok(true)`
            let event =  crossterm::event::read()?;

            stdout().execute(MoveTo(39, 29)).expect("");
            write!(stdout(), "Event::{:?}\r", event).expect("");

            /*if event == Event::Key(KeyCode::Char('c').into()) {
                println!("Cursor position: {:?}\r", position());
            }

            if event == Event::Key(KeyCode::Esc.into()) {
                break;
            }*/
        } else {
            // Timeout expired, no event for 1s
        }

    Ok(())
}

fn main() {
    enable_raw_mode().expect("");
    let mut obstacleList: Vec<Vec<Obstacle>> = InitializeNew_SetOf_Obstacles(23, 113);
    while terminal::size().unwrap() != (113, 31){
        stdout().execute(MoveTo(0, 0)).expect("");
        println!("Current Terminal Size: X={0};Y={1}", terminal::size().unwrap().0, terminal::size().unwrap().1);
        println!("Expected Terminal Size: X={0};Y={1}", 113, 31);
        write!(stdout(), "Please resize Terminal to start the game...").expect("");
        stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(120));
        print!("{esc}c", esc = 27 as char);
        print!("\x1B[2J\x1B[1;1H");
    }
    clear_console();
    loop{
        Render_Game(&obstacleList);
        let _ = print_events();
    }
    disable_raw_mode().expect("");
    clear_console();
    stdout().execute(MoveTo(39, 29)).expect("");
    write!(stdout(), "Press enter to close console...").expect("");
    stdout().flush().unwrap();
}