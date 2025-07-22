use rand::Rng;

pub fn read_file(file_path: &str) -> String {
    match std::fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(error) => {
            println!("Error reading file: {}", error);
            match std::fs::write(file_path, "") {
                Ok(_) => String::new(),
                Err(error) => {
                    println!("Error creating file: {}", error);
                    String::new()
                }
            }
        }
    }
}

pub fn parse_file(file_content: String) -> Vec<String> {
    file_content
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.to_string())
        .collect()
}

pub fn get_random_sentence(sentences: &Vec<String>) -> String {
    let random_index = rand::rng().random_range(0..sentences.len());
    sentences.get(random_index).unwrap().to_string()
}
