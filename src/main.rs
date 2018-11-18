extern crate curl;
extern crate md5;

#[allow(unused_imports)]
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::str;

use curl::easy::Easy;
use md5::{compute, Digest};

const HASH_URL: &str = "https://www.deltaconnected.com/arcdps/x64/d3d9.dll.md5sum";
const DLL_URL: &str = "https://www.deltaconnected.com/arcdps/x64/d3d9.dll";

fn read_hash() -> String {
  let mut v = Vec::new();

  let mut handle = Easy::new();
  handle.url(HASH_URL).unwrap();
  {
    let mut transfer = handle.transfer();
    transfer
      .write_function(|data| {
        // Example `data` between pipes:
        // |d41d8cd98f00b204e9800998ecf8427e some-file.dll|

        v.extend_from_slice(data);

        Ok(data.len())
      }).unwrap();

    transfer.perform().unwrap();
  }

  let str_data = String::from_utf8(v).unwrap();
  let split: Vec<&str> = str_data.split(" ").collect();
  String::from(split[0])
}

fn backup_old() {
  // Copy to d3d9.dll.bak
  print!("Backing up... ");

  fs::copy("./d3d9.dll", "./d3d9.dll.bak");

  println!("Done");
}

fn download_new() {
  print!("Downloading... ");

  let mut handle = Easy::new();
  handle.url(DLL_URL).unwrap();
  {
    let mut transfer = handle.transfer();
    let mut f = File::create("./d3d9.dll").unwrap();

    transfer
      .write_function(move |data| Ok(f.write(&data).unwrap()))
      .unwrap();

    transfer.perform().unwrap();
  }

  println!("Done");
}

fn calculate_md5(file: &mut File) -> String {
  let mut buffer = Vec::new();
  file.read_to_end(&mut buffer).unwrap();
  format!("{:x}", md5::compute(buffer))
}

fn main() {
  let hash = read_hash();

  match File::open("./d3d9.dll") {
    Err(_e) => {
      // Download the DLL, the old one doesn't exist
      download_new();
    }
    Ok(mut file) => {
      let file_hash = calculate_md5(&mut file);

      if file_hash.eq(&hash) {
        println!("ArcDPS is up to date");
        return;
      }

      println!("New version found");

      backup_old();
      download_new();
    }
  }
}
