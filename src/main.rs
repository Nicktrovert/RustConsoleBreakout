extern crate winapi;
extern crate user32;
extern crate kernel32;
use std::*;
use std::io::*;
use std::time::*;
use crossterm::*;
use colored::Colorize;
use crossterm::cursor::{MoveTo};
use crossterm::event::{Event, KeyCode, poll};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

struct Obstacle{
    coords: (i16, i16),
    content: String,
}

impl Obstacle{
    fn get_default_content() -> String {
        return "████".to_string();
    }

    /*fn new() -> Obstacle {
        return Obstacle{
            coords: (0, 0),
            content: Obstacle::get_default_content().to_string(),
        }
    }*/
}

fn clear_console(){
    print!("{esc}c", esc = 27 as char);
    print!("\x1B[2J\x1B[1;1H");
    let mut stdout = BufWriter::with_capacity(terminal::size().unwrap().0 as usize * terminal::size().unwrap().1 as usize * 100, stdout());
    render_canvas(&mut stdout);
    write!(stdout, "{}", MoveTo(1, 1)).expect("");
    stdout.flush().unwrap();
}

fn initialize_new_set_of_obstacles(rows: i32, columns: i32) -> Vec<Vec<Obstacle>> {
    let mut obstacle_list: Vec<Vec<Obstacle>> = Vec::new();
    for i in 1..rows{
        let mut obstacle_list_element: Vec<Obstacle> = Vec::new();
        for j in 1..columns / (Obstacle::get_default_content().to_string().chars().count() as i32) - 5{
            let new_obstacle = Obstacle{
                coords: ((j * ((Obstacle::get_default_content().to_string().chars().count() as i32) + 1) - 3) as i16, i as i16),
                content: Obstacle::get_default_content(),
            };
            obstacle_list_element.push(new_obstacle);
        }
        obstacle_list.push(obstacle_list_element);
    }
    return obstacle_list;
}

fn render_game(obstacles_list: &mut Vec<Vec<Obstacle>>){
    let mut stdout = BufWriter::with_capacity(terminal::size().unwrap().0 as usize * terminal::size().unwrap().1 as usize * 100, stdout());

    render_canvas(&mut stdout);
    render_obstacles(obstacles_list, &mut stdout);

    write!(stdout, "{}", MoveTo(0, 0)).expect("");
}

fn render_obstacles(obstacles_list: &mut Vec<Vec<Obstacle>>, writer: &mut BufWriter<Stdout>){
    let color_vec = vec!["red", "green", "blue", "yellow", "magenta", "orange"];
    let mut y = 0;
    let mut color_iterator = 0;
    for obstacles in obstacles_list {
        for obstacle in obstacles {
            if obstacle.coords.1 != y{
                y = obstacle.coords.1;
                write!(writer, "{}", crossterm::cursor::MoveTo(2, obstacle.coords.1 as u16)).expect("");
            }
            write!(writer, "{} ", obstacle.content.color(color_vec[color_iterator % color_vec.iter().count()])).expect("");
            //color_iterator += 1;
        }
        color_iterator += 1;
    }
}

fn render_canvas(writer: &mut BufWriter<Stdout>) {
    write!(writer, "{}", MoveTo(0, 0)).expect("");
    let mut string_to_print = String::new();
    let (width, height) = terminal::size().unwrap();

    for i in 0..height {
        for j in 0..width {
            let mut char_to_print = ' ';

            match (i, j) {
                (0, 0) => char_to_print = '╔',
                (0, w) if w == width - 1 => char_to_print = '╗',
                (h, 0) if h == height - 1 => char_to_print = '╚',
                (h, w) if h == height - 1 && w == width - 1 => char_to_print = '╝',
                (h, w) if h == height - 3 => {
                    if w == 0 || w == width - 1 {
                        char_to_print = if w == 0 { '╠' } else { '╣' };
                    } else {
                        char_to_print = '═';
                    }
                }
                (_h, w) | (_h, w) if w == 0 || w == width - 1 => char_to_print = '║',
                _ => {
                    if (i == 0 || i == height - 1) && (j != 0 && j != width - 1) {
                        char_to_print = '═';
                    }
                }
            }

            string_to_print.push(char_to_print);
        }
        if i != height - 1 {
            string_to_print.push('\n');
        }
    }

    write!(writer, "{}", string_to_print).expect("");
}


fn print_events(obstacles_list: &mut Vec<Vec<Obstacle>>) -> String {
        if poll(Duration::from_millis(0)).expect("") {
            let event =  crossterm::event::read().unwrap();

            write!(stdout(), "{}", MoveTo(terminal::size().unwrap().0 / 2, terminal::size().unwrap().1 - 2)).expect("");
            write!(stdout(), "Event::{:?}\r", event).expect("");

            if event == Event::Key(KeyCode::Esc.into()) {
                return "esc".to_string();
            }
            if event == Event::Key(KeyCode::Left.into()) || event == Event::Key(KeyCode::Char('a').into()){
                for i in 0..obstacles_list.len(){
                    if obstacles_list[i].len() == 0{
                        continue;
                    }
                    else{
                        obstacles_list[i].remove(0);
                        break;
                    }
                }
                return "left".to_string();
            }
        } else {
            // Timeout expired, no event
        }
    return "".to_string();
}

async fn on_tick(obstacle_list: &mut Vec<Vec<Obstacle>>) -> Duration {
    let time_now = std::time::Instant::now();
    render_game(obstacle_list);
    match print_events(obstacle_list).to_lowercase().as_str(){
        "esc" => return Duration::from_secs(999),
        "left" => {},
        "right" => {},
        _ => {}
    }
    write!(stdout(), "{}", MoveTo(0, 0)).expect("");
    let elapsed_seconds = time_now.elapsed().as_secs_f64();
    write!(stdout(), "{} seconds passed", elapsed_seconds).expect("");
    stdout().flush().unwrap();
    return time_now.elapsed();
}

#[tokio::main]
async fn main() {
    enable_raw_mode().expect("");
    let mut obstacle_list: Vec<Vec<Obstacle>> = initialize_new_set_of_obstacles(16, 113);
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

    let refresh_rate = 30;
    let millis_per_second = 1000;
    let mut delay = tokio::time::interval(std::time::Duration::from_millis(&millis_per_second / refresh_rate));
    loop {
        delay.tick().await;
        let time_to_run = on_tick(&mut obstacle_list).await;
        if time_to_run.as_secs() == 999{
            break;
        }
        let calculated_delay_offset = time_to_run.as_secs() * 1000;
        delay = tokio::time::interval(std::time::Duration::from_millis((millis_per_second - calculated_delay_offset) / refresh_rate));
    }

    disable_raw_mode().expect("");
    clear_console();
    stdout().execute(MoveTo((terminal::size().unwrap().0 / 2) - "Press key to close...".len() as u16 / 2, terminal::size().unwrap().1 - 2)).expect("");
    write!(stdout(), "Press key to close...").expect("");
    stdout().flush().unwrap();
    let _ = crossterm::event::read().unwrap();
}