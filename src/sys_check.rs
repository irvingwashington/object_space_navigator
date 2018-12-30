use std::process;
use std::process::Command;
use std::str;
use std::fs;
pub struct SysCheck {}

impl SysCheck {
  pub fn rss() -> Option<usize> {
    let pid_str = format!("{}", process::id());
    let rss_output = Command::new("ps").args(&["-p", &pid_str, "-o", "rss"]).output();

    match rss_output {
      Ok(output) => {
        let str_value = str::from_utf8(&output.stdout).unwrap();
        let rss_lines: Vec<&str> = str_value.split("\n").collect();
        let rss_line;
        match rss_lines.get(1) {
          Some(line) => rss_line = line.trim(),
          None => { return None }
        }

        match usize::from_str_radix(rss_line, 10) {
          Ok(value) => Some(value),
          Err(_) => None
        }
      }
      Err(_) => None
    }
  }
}

pub struct FileCheck {}
impl FileCheck {
  pub fn size_kb(filename: &str) -> usize {
    let metadata = fs::metadata(filename);

    match metadata {
      Ok(mtd) => (mtd.len() / 1024) as usize,
      Err(_) => 0 as usize
    }
  }
}
