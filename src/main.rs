use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Display;
use std::path::Path;

#[derive(Debug)]
struct Greeting {
  status: String,
  content: String,
}

fn main() {
  let greeting = Greeting {
    status: "test".to_string(),
    content: "another test".to_string(),
  };
  let status = greeting.status;
  println!("Hello, world! {}", status);

  let path = Path::new("pisomka.tex");
  let display = path.display();

  // Open the path in read-only mode, returns `io::Result<File>`
  let mut file = match File::open(&path) {
    // The `description` method of `io::Error` returns a string that
    // describes the error
    Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
    Ok(file) => file,
  };

  // Read the file contents into a string, returns `io::Result<usize>`
  let mut s = String::new();
  match file.read_to_string(&mut s) {
    Err(why) => panic!("couldn't read {}: {}", display, Error::description(&why)),
    Ok(_) => show_file_contents(display, s),
  }
}

fn show_file_contents(display: Display, s: String) {
  // print!("{} contains:\n{}", display, s);

  prepare_final_tex(
    s,
    "{pr1_ing_10}".to_string(),
    "{pr2_ing_5}".to_string(),
    "{pr3_ing_12}".to_string(),
  );
}

fn prepare_final_tex(
  fulltext: String,
  pr1: String,
  pr2: String,
  pr3: String,
) -> std::io::Result<()> {
  let part1 = "\\documentclass{article}\n
  \\usepackage[utf8]{inputenc}\n\n

  \\title{Matematika I}\n\n

  \\begin{document}\n
    \\maketitle\n\n";

  let part2 = format!(
    "\\include{}\n
  \\include{}\n
  \\include{}\n\n",
    pr1.to_string(),
    pr2.to_string(),
    pr3.to_string()
  );

  let part3 = "\\end{document}";

  let result = [part1, &part2, part3].join(""); //format!("{}", fulltext, pr1, pr2, pr3);

  println!("{}", result);

  let mut buffer = File::create("exam_sources/ing_01.tex")?;
  buffer.write_fmt(format_args!("{}", result))?;
  Ok(())
}
