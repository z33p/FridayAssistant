use std::fs::File;
use std::io::prelude::*;
use std::thread;
use std::time::Duration;

fn main() {
    // Run the daemon in a separate thread so that the main thread can exit
    thread::spawn(|| {
        loop {
            // Do the work of the daemon here
            println!("Hello from the daemon!");
            write_file();

            // Sleep for a short period of time to prevent excessive resource usage
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Keep the main thread alive to prevent the program from exiting immediately
    loop {}
}

fn write_file() {
    // Create a new file and open it for writing
    let mut file = match File::create("example.txt") {
        Ok(file) => file,
        Err(error) => panic!("Failed to create file: {}", error),
    };

    // Write a string to the file
    let data = "Hello from the daemon!";
    match file.write(data.as_bytes()) {
        Ok(_) => println!("Data written to file successfully."),
        Err(error) => panic!("Failed to write data to file: {}", error),
    }
}
