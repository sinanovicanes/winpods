// Prevents an additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // The guard flushes the log file's background writer; holding it until `run` returns keeps
    // the last lines before a shutdown from being lost.
    let _logging = winpods_lib::logging::init();

    winpods_lib::run();
}
