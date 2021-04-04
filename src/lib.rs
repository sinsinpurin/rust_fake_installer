use indicatif::{ProgressBar, ProgressStyle};
use std::{thread, time};
use std::fs::File;
use std::io::prelude::*;

// logging用の時間に関するパッケージ
use chrono::{Utc, DateTime};

use rand::Rng;

pub struct Installer{
    pub mode: Mode
}

pub enum Mode {
    InstallFile,
    Hacking
}

impl Installer{
    pub fn new( mode: Mode) -> Result<Installer, &'static str>{
        Ok(Installer{mode})
    }
    pub fn run(&self){
        match &self.mode{
            Mode::InstallFile => {
                &self.run_installfile_mode();
            }
            Mode::Hacking => {
                &self.run_hacking_mode();
            }
        }
    }

    // 具体的なinstallfileモードの処理
    fn run_installfile_mode(&self){
        let filenames = get_file_strings(&self.mode);
        let vec_filename: Vec<&str> = filenames.split_whitespace().collect();

        for filename in vec_filename{
            fake_progressbar(filename);
        }
    }

    // 具体的なハッキングモードの処理
    fn run_hacking_mode(&self){
        let filenames = get_file_strings(&self.mode);
        let vec_filename: Vec<&str> = filenames.split_whitespace().collect();

        for filename in vec_filename{
            fake_log(filename);
        }
        
    }
}

// 偽のシーケンスバーを表示
pub fn fake_progressbar(filename : &str){
    let mut rng = rand::thread_rng();
    let total_size = 100;
    
    let bar = ProgressBar::new(total_size);
    let sty = ProgressStyle::default_bar()
    .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
    .progress_chars("##-");
    bar.set_style(sty);
    
    for _ in 0..total_size {
        // リアル感のためのスリープ
        let sleep_time = rng.gen_range(10..50);
        bar.set_message(&format!("download {}", filename));
        bar.inc(1);
        thread::sleep(time::Duration::from_millis(sleep_time));
    }
    bar.finish_with_message("downloaded");
}

pub fn fake_log(file_name: &str){
    let mut rng = rand::thread_rng();
    let sleep_time = rng.gen_range(10..50);
    logger(file_name, "start");
    thread::sleep(time::Duration::from_millis(sleep_time));
    fake_progressbar(file_name);
    logger(file_name, "end");
}

// logger
pub fn logger(process_name : &str, message: &str){
    let utc_datetime: DateTime<Utc> = Utc::now();
    println!("[{}   :{}] {}", utc_datetime, process_name, message);
}

// file名などに使用する辞書 txtフォルダ
pub fn get_file_strings(mode: &Mode) -> String{
    let mut file: File;
    let mut content = String::new();
    match mode{
        Mode::InstallFile=>{
            file = File::open("inst.txt").expect("error: inst.txt couldn't import.");
            file.read_to_string(&mut content).expect("error: inst.txt couldn't read.");
        },
        Mode::Hacking=>{
            file = File::open("hk.txt").expect("error: inst.txt couldn't import.");
            file.read_to_string(&mut content).expect("error: inst.txt couldn't read.");
        }
    }
    content
}
