/// I/O Handler Scanner
///
/// Scanner module for I/O Handler.
///
/// Functions:
///    read_lines -> Collect bytes from ANSI document and return std::io::Lines.
///    get_art_para -> Collect std::io::Lines into Vec<String>.

use std::{
    fs::File,
    io::{Result, Lines, BufRead, BufReader},
    path::Path,
};

pub fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

pub fn get_art_para(filename: &str, width: usize) -> Result<Vec<String>> {
    let mut linevec = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(artline) = line {
                if !artline.starts_with("?>=") {
                    let snipline: String = String::from(&artline).chars().into_iter().take(width).collect();

                    linevec.push(snipline+"\n");
                }
            }
        }
    }
    Ok(linevec)
}
