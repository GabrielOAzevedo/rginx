use std::fs;

pub fn parse_markdown(contents: &str) -> String {
    let lines: Vec<&str> = contents.split("\n").collect();

    let mut total_string: String = String::new();

    for line in lines.iter() {
        if line == &"" {
            continue;
        }
        let first_char = line.chars().nth(0).unwrap();
        let formatted_line = match first_char {
            '#' => format!("<h1>{}</h1>", parse_line(line, &'#')),
            _ => format!("<p>{}</p>", line.to_string())
        }.to_string();

        total_string = format!("{}{}", total_string, formatted_line);
    }

    let html_template = fs::read_to_string("public/md_template.html")
        .unwrap()
        .replace("{{MARKDOWN}}", &total_string);

        println!("{}", html_template);

    html_template.to_string()
}

fn parse_line(line: &str, token: &char) -> String {

    let mut cloned_line: String = line.to_string();

    cloned_line = cloned_line.strip_prefix("#").unwrap_or("").to_string();
    cloned_line = cloned_line.strip_prefix(" ").unwrap_or("").to_string();

    cloned_line
}