use std::{fs::{self}, path::{Path, PathBuf}};
use chrono::{DateTime, Utc};
use clap::{Parser};
use owo_colors::{self,OwoColorize};
use strum::Display;
use tabled::{settings::{self, object::Columns, Color}, Table, Tabled};

#[derive(Debug, Parser)]
#[command(version="0.0.1", about, long_about="Improved ls command")]
struct CLI {
    path: Option<PathBuf>,
}

#[derive(Debug, Tabled)]
struct File {
    #[tabled{rename="Name"}]
    file_name: String,

    #[tabled{rename="Size (B)"}]
    len_byte:   u64,

    modified:   String,
    
    #[tabled{rename="Type"}]
    e_type: EntryType,
}

#[derive(Debug, Display)]
enum EntryType {
    File,
    Dir,
}

fn main() {
    let cli = CLI::parse();
    let path = cli.path.unwrap_or(PathBuf::from("."));

    if let Ok(does_exists) = fs::exists(&path){
        if does_exists{
            //  Table settings
            let mut table = Table::new(get_files(&path));
            
            table.with(settings::Style::empty());

            table.modify(Columns::first(), Color::FG_BRIGHT_CYAN);
            table.modify(Columns::one(1), Color::FG_BRIGHT_MAGENTA);
            table.modify(Columns::one(2), Color::FG_BRIGHT_YELLOW);
            table.modify(Columns::one(3), Color::FG_BRIGHT_RED);

            println!("{}", table);

        } else{
            println!("{}", "Path doesn't exist.".red());
        }
    } else {
        println!("{}", "Error occured while reading the path".red());
    }
}

fn get_files(path:&Path) -> Vec<File> {
    let mut data = Vec::default();

    if let Ok(read_dir) = fs::read_dir(path){

        //  Reading the directiry and getting metadata is succed
        for entry in read_dir{

            if let Ok(entry_file) = entry {
                get_meta(entry_file, &mut data);
            } else {
                println!("{}","Error while reading contents of directory".red());
            }
        }
    }
    data
}

fn get_meta(file:fs::DirEntry, data: &mut Vec<File>){
                if let Ok(meta) = fs::metadata(file.path()){
                    data.push(File { file_name: file
                        .file_name()
                        .into_string()
                        .unwrap_or("unknown".into()),
                        e_type: if meta.is_dir(){
                            EntryType::Dir
                        }else{ EntryType::File
                        },

                        len_byte:   meta.len(),
                        modified:   if let Ok(modi) = meta.modified(){
                            let date:DateTime<Utc> = modi.into();
                            format!("{}", date.format("%a %b %e %Y"))
                        } else {String::default()}
                        });
                    }
}
