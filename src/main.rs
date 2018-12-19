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

    let mut i = i32::from_str(&args[1]).unwrap();
    for _ in ticks {
        if(i < 0){
            break;
        }
        
        clear();
        // execute code once a second, send results via `tx`
        mvprintw(rows/2, cols/2, &format!("{}s", i));
        refresh();

        i -= 1;
    }

    getch();
    endwin();
}