#![allow(unused_parens)]

extern crate ncurses;
extern crate eventual;

use ncurses::*;
use eventual::Timer;
use std::env;
use std::str::FromStr;

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
    
    initscr();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    getmaxyx(stdscr(), &mut rows, &mut cols);

    let timer = Timer::new();
    let ticks = timer.interval_ms(1000).iter();

    print_centered_message(&rows,&cols,&"Starting!");
    let mut i = i32::from_str(&args[1]).unwrap();
    for _ in ticks {
        if(i < 0){
            break;
        }
        
        print_centered_message(&rows,&cols,&format!("{}s", i));

        i -= 1;
    }

    getch();
    endwin();
}

fn print_centered_message(rows: &i32,cols: &i32, msg: &str) {
        clear();
        mvprintw(rows/2, cols/2, &format!("{}", msg));
        refresh();
}


fn print_usage() {
    println!("Usage: consoletimer [time in seconds]");
}