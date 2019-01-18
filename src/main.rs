extern crate ncurses;
use ncurses::*; // TODO: Don't use globs
use std::char;
use std::fs;
use std::process::Command;
use std::str;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
extern crate rand;
use rand::Rng;

fn main() {
    run_move(
        "./connect4.py".to_string(),
        3,
        "x".to_string(),
        "------------------------------------------".to_string(),
        Duration::from_millis(1500),
    );
    //
    // config();
    // loop {
    // let key = getch();
    // if get_key(key) == 'q' {
    // break;
    // }
    // }
    // endwin();
    // let s = match str::from_utf8(&output.stdout) {
    // Ok(v) => v,
    // Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    // };
    // dbg!(s);
}

fn run_move(c: String, ply: usize, play: String, board: String, t: Duration) -> Option<u8> {
    let n = rand::thread_rng().gen_range(10000, 100000);
    let outputfile = format!("out-{}.txt", n);
    let prefix = "Result:".to_string();
    let (tx, rx) = mpsc::channel();
    let s = thread::spawn(move || {
        let mut cmd = Command::new(c)
            .arg(format!("ply={}", ply))
            .arg(format!("play={}", play))
            .arg(format!("board={}", board))
            .arg(format!("outputfile={}", outputfile))
            .arg(format!("result_prefix={}", prefix))
            .arg(format!("cputime={}", t.as_secs()))
            .arg("action=move")
            .spawn()
            .expect("oops"); // TODO: Actual error handling
        tx.send(&mut cmd).unwrap();
        cmd.wait();
        tx.send(&mut cmd).unwrap();
    });
    let mut child = rx.recv().unwrap();
    rx.recv_timeout(t).unwrap();
    child.kill().unwrap();
    // for j in 0..5 {
        // dbg!(j);
        // thread::sleep(Duration::from_millis(100));
    // }
    // tx.send("hi").unwrap();
    // s.join().unwrap();
    // thread::sleep(Duration::from_millis(5000));
    // match cmd.kill() {
        // Ok(_) => (),
        // Err(_) => (),
    // };
    // let m = read_last(outputfile, prefix);
    // let ls = match m {
        // Some(ref s) => s.lines(),
        // None => return None,
    // };
    // for l in dbg!(ls) {
        // if l.starts_with("move=") {
            // let s = l.trim_start_matches("move=");
            // let r = s.parse::<u8>();
            // return match r {
                // Ok(n) => Some(n),
                // Err(_) => None,
            // };
        // }
    // }
    return None;
}

fn read_last(f: String, prefix: String) -> Option<String> {
    let s = match fs::read_to_string(f) {
        Ok(x) => x,
        Err(_) => return None,
    };
    let ls = s.lines();
    let mut last_output = String::new();
    for l in dbg!(ls) {
        if l == prefix {
            last_output = String::new();
        } else {
            last_output.push_str(l);
            last_output.push('\n');
        }
    }
    return Some(last_output);
}

fn get_key(n: i32) -> char {
    return match char::from_u32(n as u32) {
        Some(x) => x,
        None => '\x00',
    };
}

fn config() {
    // Manage windowing
    let win = initscr();

    let mut max_x: i32 = 0;
    let mut max_y: i32 = 0;

    // Set max_x and max_y
    getmaxyx(win, &mut max_y, &mut max_x);

    // Catch control characters
    raw();

    // Don't echo keystrokes
    noecho();

    // Hide cursor
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    // Mac arrow keys fix
    keypad(stdscr(), true);
}
