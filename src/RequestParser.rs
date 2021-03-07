pub fn get_request_info(request: &str) -> (String, String) {

    let lines: Vec<&str> = request.split("\r\n").collect();
    let first_line: Vec<&str> = lines[0].split_whitespace().collect();
    let target_file = first_line[1];

    if target_file == "/" {
        ("HTTP/1.1 200 OK \r\n\r\n".to_string(), "public/index.html".to_string())
    } else {
        let path = "public".to_owned() + target_file;
        ("HTTP/1.1 200 OK \r\n\r\n".to_string(), path)
    }
}