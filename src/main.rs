#![allow(unused_parens)]
#![allow(non_snake_case)]

extern crate ncurses;
extern crate eventual;
extern crate regex;

use ncurses::*;
use eventual::Timer;
use std::env;
use std::str::FromStr;
use regex::Regex;

fn main() {
    let args : Vec<String> = env::args().collect();

    if(args.len() < 2){
        print_usage();
        return;
    }
    
    run(&args);
}

fn run(args: &Vec<String>) {
    let mut cols = 0;
    let mut rows = 0;

    let seconds_or_none = get_seconds(&args[1]);
    if(seconds_or_none.is_none()){
        print_usage();
        return;
    }
    
    initscr();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    getmaxyx(stdscr(), &mut rows, &mut cols);

    let timer = Timer::new();
    let ticks = timer.interval_ms(1000).iter();

    print_centered_message(&rows,&cols,&"Starting!");

    let seconds_count = seconds_or_none.unwrap();
    let mut i = seconds_count;
    for _ in ticks {
        if(i < 0){
            break;
        }
        
        print_centered_message(&rows,&cols,&format!("{}s", i));

        i -= 1;
    }

    print_centered_message(&rows, &cols, &format!("Finished after {}s", seconds_count));

    getch();
    endwin();
}

fn print_centered_message(rows: &i32, cols: &i32, msg: &str) {
        clear();
        let posX : i32 = cols/2 - ((msg.len()/2) as i32);
        mvprintw(rows/2, posX, &msg);
        refresh();
}

fn get_seconds(time_string: &str) -> Option<i32> {
    let num_re = Regex::new(r"^(\d+)$").unwrap();
    let mins_re = Regex::new(r"^(\d+)m$").unwrap();

    if(num_re.is_match(time_string)){
        let c = num_re.captures(time_string).unwrap();
        let number = i32::from_str(&c[1]).unwrap();
        return Some(number);
    }

    if(mins_re.is_match(time_string)){
        let c = mins_re.captures(time_string).unwrap();
        let seconds = i32::from_str(&c[1]).unwrap();
        return Some(seconds*60);
    }

    return None;
}

fn print_usage() {
    println!("Usage: consoletimer [time in seconds]");
}