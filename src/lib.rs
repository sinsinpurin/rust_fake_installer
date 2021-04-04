use indicatif::{ProgressBar, ProgressStyle};
use std::{thread, time};
use std::fs::File;
use std::io::prelude::*;

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
                // 
                &self.run_installfile_mode();
            }
            Mode::Hacking => {
                println!("hacking mode");
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

    }
}

// 偽のシーケンスバーを表示
pub fn fake_progressbar(filename : &str){
    let ten_millis = time::Duration::from_millis(100);
    let total_size = 100;
    
    let bar = ProgressBar::new(total_size);
    let sty = ProgressStyle::default_bar()
    .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
    .progress_chars("##-");
    bar.set_style(sty);
    
    for _ in 0..total_size {
        bar.set_message(&format!("dounload {}", filename));
        bar.inc(1);
        thread::sleep(ten_millis);
    }
    bar.finish_with_message("downloaded");
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
