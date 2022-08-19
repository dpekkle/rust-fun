use std::fs::File;


fn openfile_or_panic(path: &str) -> File {
    let greeting_file_result = File::open(path);
	
	let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

	greeting_file
}

fn openfile_or_panic_2(path: &str) -> File {
    let greeting_file_result = File::open(path).expect("{path:?} not in path");
	
	greeting_file_result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
	#[should_panic(expected="Problem opening the file:")]
    fn openfile_panics() {
		openfile_or_panic("hello.txt");
    }

    #[test]
	#[should_panic(expected="hello.txt not in path")]
    fn openfile_panics_again() {
		openfile_or_panic_2("hello.txt");
    }

}
