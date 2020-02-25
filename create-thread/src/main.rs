use std::thread;

fn main() {
    println!("Begin program.");

    let mut waitForThread = 1;
    while waitForThread <= 2 {
        if waitForThread == 1 {
            println!(
                "\nPass {}. Create thread and wait for it to complete...",
                waitForThread
            );
        } else {
            println!("\nPass {}. Create thread and continue...", waitForThread);
        }

        let handle = thread::spawn(|| {
            thread::sleep_ms(100);
            println!("Hello from new thread");
        });
        println!("Created thread, id = {:?}", handle.thread().id());

        if waitForThread == 1 {
            handle.join();
        }

        waitForThread += 1;
    }

    println!("\nGoodbye. End of program.");
}
