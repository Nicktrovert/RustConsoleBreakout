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
use crossterm::event::KeyCode::Char;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

struct GameData{
    obstacle_matrix: Vec<Vec<Obstacle>>,
    bouncer: Bouncer,
    ball: Ball,
}
impl GameData{

}

struct Bouncer{
    coords: (i16, i16),
    width: i16,
    content: String,
}
impl Bouncer{
    fn do_movement(&mut self, direction: i16){
        self.coords = (self.coords.0 - direction.clamp(-1, 1), self.coords.1);
    }
}

struct Ball{
    coords: (i16, i16),
    velocity: (i16, i16),
    content: String,
}
impl Ball{
    fn is_colliding(&self, other: Obstacle) -> bool{
        let is_in_range_lon = self.coords.0 >= other.coords.0 && self.coords.0 <= other.coords.0 + other.width;
        let is_in_range_lat = self.coords.1 == other.coords.1;
        if is_in_range_lat && is_in_range_lon{
            return true;
        }
        return false;
    }
}

struct Obstacle{
    coords: (i16, i16),
    width: i16,
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
    let (width, height) = terminal::size().unwrap();
    let mut output: Vec<Vec<char>> = vec![vec![' '; width as usize]; height as usize];
    //let mut stdout = BufWriter::with_capacity(height as usize * width as usize * 100, stdout());
    render_canvas(&mut output);
    write!(stdout(), "{}", MoveTo(0, 0)).expect("");
    for i in output{
        write!(stdout(), "{}", String::from_iter(i)).expect("");
    }
    stdout().flush().unwrap();
}

fn initialize_new_set_of_obstacles(rows: i32, columns: i32) -> Vec<Vec<Obstacle>> {
    let mut obstacle_list: Vec<Vec<Obstacle>> = Vec::new();
    for i in 1..rows{
        let mut obstacle_list_element: Vec<Obstacle> = Vec::new();
        for j in 1..columns / (Obstacle::get_default_content().to_string().chars().count() as i32) - 5{
            let new_obstacle = Obstacle{
                coords: ((j * ((Obstacle::get_default_content().to_string().chars().count() as i32) + 1) - 3) as i16, i as i16),
                content: Obstacle::get_default_content(),
                width: Obstacle::get_default_content().len() as i16,
            };
            obstacle_list_element.push(new_obstacle);
        }
        obstacle_list.push(obstacle_list_element);
    }
    return obstacle_list;
}

fn render_game(obstacles_list: &mut Vec<Vec<Obstacle>>){
    let (width, height) = terminal::size().unwrap();
    let mut output: Vec<Vec<char>> = vec![vec![' '; width as usize]; height as usize];
    //let mut stdout = BufWriter::with_capacity(terminal::size().unwrap().0 as usize * terminal::size().unwrap().1 as usize * 100, stdout());

    render_canvas(&mut output);
    render_obstacles(obstacles_list, &mut output);

    write!(stdout(), "{}", MoveTo(0, 0)).expect("");
    for i in output{
        write!(stdout(), "{}", String::from_iter(i)).expect("")
    }
}

fn render_obstacles(obstacles_list: &mut Vec<Vec<Obstacle>>, writer: &mut Vec<Vec<char>>){
    //let color_vec = vec!["red", "green", "blue", "yellow", "magenta", "orange"]; //todo - implement color once again (I broke it)
    let mut y = 0;
    let mut x:i16 = 0;
    let mut color_iterator = 0;
    for obstacles in obstacles_list {
        for obstacle in obstacles {
            if obstacle.coords.1 != y{
                y = obstacle.coords.1;
                x = 0;
                //write!(writer, "{}", crossterm::cursor::MoveTo(x as u16, obstacle.coords.1 as u16)).expect("");
            }
            for i in 0..(obstacle.content.len() / 2 - 1) as i16{
                //writer[y as usize][(x + i+1) as usize] = obstacle.content.color(color_vec[color_iterator % color_vec.iter().count()]).chars().nth(i as usize).unwrap();
                if obstacle.content.chars().take(i as usize).last() != None{
                    writer[(y) as usize][(x + i+1) as usize] = obstacle.content.chars().take(i as usize).last().unwrap();
                }
            }
            x += (obstacle.content.len() / 2 - 1) as i16;
            //write!(writer, "{} ", obstacle.content.color(color_vec[color_iterator % color_vec.iter().count()])).expect("");
            //x += Obstacle::get_default_content().len() as i16;
            //color_iterator += 1;
        }
        color_iterator += 1;
    }
}

fn render_canvas(writer: &mut Vec<Vec<char>>) {
    let (width, height) = terminal::size().unwrap();

    for i in 0..height {
        for j in 0..width {

            writer[i as usize][j as usize] = match (i, j) {
                (0, 0) => '╔',
                (0, w) if w == width - 1 => '╗',
                (h, 0) if h == height - 1 => '╚',
                (h, w) if h == height - 1 && w == width - 1 => '╝',
                (h, w) if h == height - 3 => {
                    if w == 0 || w == width - 1 {
                        if w == 0 { '╠' } else { '╣' }
                    } else {
                        '═'
                    }
                }
                (_h, w) | (_h, w) if w == 0 || w == width - 1 => '║',
                _ if (i == 0 || i == height - 1) && (j != 0 && j != width - 1) => '═',
                _ => ' '
            };
        }
    }
}


fn get_input_events(obstacles_list: &mut Vec<Vec<Obstacle>>) -> String {
        if poll(Duration::from_millis(0)).expect("") {
            let event =  crossterm::event::read().unwrap();

            //write!(stdout(), "{}", MoveTo(terminal::size().unwrap().0 / 2, terminal::size().unwrap().1 - 2)).expect("");
            //write!(stdout(), "Event::{:?}\r", event).expect("");

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

async fn on_tick(game_data: &mut GameData) -> Duration {
    let time_now = std::time::Instant::now();
    render_game(&mut game_data.obstacle_matrix);
    match get_input_events(&mut game_data.obstacle_matrix).to_lowercase().as_str(){
        "esc" => return Duration::from_secs(999),
        _ => {}
    }
    //write!(stdout(), "{}", MoveTo(0, 0)).expect("");
    //let elapsed_seconds = time_now.elapsed().as_secs_f64();
    //write!(stdout(), "{} seconds passed", elapsed_seconds).expect("");
    stdout().flush().unwrap();
    return time_now.elapsed();
}

#[tokio::main]
async fn main() {
    enable_raw_mode().expect("");
    let mut game_data: GameData = GameData{
        obstacle_matrix: initialize_new_set_of_obstacles(16, 113),
        bouncer: Bouncer{
            coords: (54, 24),
            width: Obstacle::get_default_content().len() as i16,
            content: Obstacle::get_default_content(),
        },
        ball: Ball{
            coords: (55, 22),
            content: "o".to_string(),
            velocity: (0, 0),
        },
    };

    //Ensure Correct Terminal Size
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

    //Main Game Loop
    let refresh_rate = 30;
    let millis_per_second = 1000;
    let mut delay = tokio::time::interval(std::time::Duration::from_millis(&millis_per_second / refresh_rate));
    loop {
        delay.tick().await;
        let time_to_run = on_tick(&mut game_data).await;
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