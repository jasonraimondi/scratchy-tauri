use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::path::Path;

static BLOCKLIST_START: &str = "#### Jason";
static BLOCKLIST_END: &str = "#### JasonEnd";

pub fn add_to_blocklist(_site: &str) {
  // Create a path to the desired file
  let path = Path::new("/etc/hosts");
  let display = path.display();

  // Open the path in read-only mode, returns `io::Result<File>`
  let mut file = match File::open(&path) {
    Err(why) => panic!("couldn't open {}: {}", display, why),
    Ok(file) => file,
  };

  // Read the file contents into a string, returns `io::Result<usize>`
  let mut s = String::new();
  match file.read_to_string(&mut s) {
    Err(why) => panic!("couldn't read {}: {}", display, why),
    Ok(s) => print!("{} contains:\n{}", display, s),
  }

  // CREATE FILE

  let path = Path::new("lorem_ipsum.txt");
  let display = path.display();

  // Open a file in write-only mode, returns `io::Result<File>`
  let mut new_file = match File::create(&path) {
    Err(why) => panic!("couldn't create {}: {}", display, why),
    Ok(file) => file,
  };

  // s = s.replace()

  match new_file.write_all(s.as_bytes()) {
    Err(why) => panic!("couldn't write to {}: {}", display, why),
    Ok(_) => println!("successfully wrote to {}", display),
  }

  // let mut f = File::open("/etc/hosts").expect("Unable to open file");
  // // let f = BufReader::new(f);
  //
  // // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
  // let mut s = String::new();
  // let foo = f.read_to_string(&mut s);
  //
  // match file.read_to_string(&mut s) {
  //   Err(why) => panic!("couldn't read {}: {}", display, why),
  //   Ok(_) => print!("{} contains:\n{}", display, s),
  // }
  //
  // match file.write_all(foo.as_bytes()) {
  //   Err(why) => panic!("couldn't write to {}: {}", display, why),
  //   Ok(_) => println!("successfully wrote to {}", display),
  // }
}

pub fn fetch_blocklist() -> Vec<String> {
  let mut result: Vec<String> = vec![];

  let mut has_started = false;
  let mut has_ended = false;

  if let Ok(lines) = read_lines("/etc/hosts") {
    // Consumes the iterator, returns an (Optional) String
    for line in lines {
      if let Ok(l) = line {
        if l.contains(BLOCKLIST_START) {
          has_started = true;
          continue;
        } else if l.contains(BLOCKLIST_END) {
          has_ended = true;
          continue;
        }

        if has_started && !has_ended && l != "" {
          // check if empty or not
          result.push(l);
        }
      }
    }
  }

  result
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
  P: AsRef<Path>,
{
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}

// pub fn add_to_blocklist1(site: &mut String) {
//   let f = File::open("/etc/hosts").expect("Unable to open file");
//   let f = BufReader::new(f);
//   println!("i qm the add to blocklist {:?}", f.buffer());
//
//   // let output = String::new();
//   let mut ofile = File::create("hello_world.txt").expect("unable to create file");
//   ofile.write_all(f.buffer()).expect("unable to write");
//
//   // let mut input = String::new();
//   // let mut ifile = File::open("hello_world.txt").expect("unable to open file");
//   // ifile.read_to_string(&mut input).expect("unable to read"); // compare input with output
//   println!("i am the site: {}", site);
//
//   // let f = File::open("/etc/hosts").expect("Unable to open file");
//   // let f = BufReader::new(f);
// }
