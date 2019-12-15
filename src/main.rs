extern crate tectonic;

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

  let path = Path::new("uvod.tex");
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

fn show_file_contents(_display: Display, s: String) {
  // print!("{} contains:\n{}", display, s);

  prepare_final_tex(
    s,
    "pr1-ing-10".to_string(),
    "pr2-ing-5".to_string(),
    "pr3-ing-12".to_string(),
  );
}

fn prepare_final_tex(uvod: String, pr1: String, pr2: String, pr3: String) -> std::io::Result<()> {
  // let part1 = format!("\\documentclass[12pt]{article}
  // \\usepackage{{graphics,amssymb,amsmath}}

  // \\usepackage[slovak]{babel}
  // \\usepackage[utf8]{inputenc}
  // \\usepackage[IL2]{fontenc}

  // \\usepackage{multicol}
  // \\usepackage{mathtools}

  // \\pagestyle{empty}

  // \\setlength\\textwidth{170mm}

  // \\setlength\\textheight{265mm}
  // \\addtolength\\oddsidemargin{-20mm}
  // \\addtolength\\topmargin{-20mm}
  // \\setlength{\\parindent}{1pt}
  // \\setlength{\\parskip}{10pt}
  // \\newcount\\pocet
  // \\pocet = 1
  // \\def\\pr{{\\bf \\the \\pocet .\\ \\global\\advance\\pocet by 1}}

  // \\newcommand{\\g}{ \\dots \\dots \\dots \\dots \\dots \\ }
  // \\newcommand{\\gu}{ \\dots \\dots \\ }
  // \\newcommand{\\gr}{\\dotfill \\ }

  // \\begin{document}

  // \\newenvironment{itemize*}
  //  {\\begin{itemize}
  //    \\setlength{\\itemsep}{0pt}
  //    \\setlength{\\parskip}{0pt}}
  //  {\\end{itemize}}

  // \\newenvironment{enumerate*}
  //  {\\begin{enumerate}
  //    \\setlength{\\itemsep}{0pt}
  //    \\setlength{\\parskip}{0pt}}
  //  {\\end{enumerate}}

  // \\phantom{a}

  // \\centerline{\\textbf{\\Large Matematika I}}
  // \\smallskip
  // \\centerline{current_date}
  // \\centerline{9:00}
  // \\vskip0.5cm

  // \\centerline{\\bf  Meno a priezvisko: \\gr Podpis: \\gr}
  // \\vskip0.5cm
  // \\centerline{\\bf  Ročník: \\gr študijný program: \\gr}
  // \\vskip0.5cm

  // \\medskip\n\n", current_date = "05 Január 2020");

  let part2 = format!(
    r#"{}

    \newpage

    {}
    {}"#,
    pr1.to_string(),
    pr2.to_string(),
    pr3.to_string()
  );

  let part3 = "\\end{document}".to_string();

  let result = [uvod, part2, part3].join(""); //format!("{}", fulltext, pr1, pr2, pr3);

  println!("{}", result);

  let mut buffer = File::create("exam_sources/ing_01.tex")?;
  buffer.write_fmt(format_args!("{}", result))?;

  // let source = File::open("exams_sources/ing_01.tex")?;
  let latex = r#"
\documentclass{article}
\begin{document}
Hello, world!
\end{document}
"#;

  /* Run the TeX engine */
  let pdf_data: Vec<u8> = {
    tectonic::latex_to_pdf(result)?
  };
  println!("Output PDF size is {:?} bytes", pdf_data.len());

  /* output the results */
  {
    std::fs::create_dir_all("pdfs")?;
    let mut out_buf_pdf = File::create(format!("pdfs/{}.pdf", "ing_01".to_string()))?;
    out_buf_pdf.write_all(&pdf_data)?
  }

  Ok(())
}
