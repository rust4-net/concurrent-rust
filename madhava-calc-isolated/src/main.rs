use clap::Clap;
use num_format::{SystemLocale, ToFormattedString};
use std::thread;
use std::time::{Instant};
use num_cpus;


fn main() {
    println!("Begin program.");

    let app_timer = Instant::now();

    let opts: Opts = Opts::parse();
    if Ok(()) == is_valid_magnitude(opts.oom) {
        calulate_pi(opts.oom);
    }

    println!("\nGoodbye. End of program.  Total execution time: {} ms", app_timer.elapsed().as_millis());
}    

fn calulate_pi(oom: u8) {
    // Limit to a thread per cpu
    let _thread_count: u64 = num_cpus::get() as u64;
    // Size of each window or segement of the series
    let factors_per_thread: u64 = 10u64.pow(u32::from(oom - 1));
    // Vector for the thread handles to join after all created
    let mut thread_handles = Vec::new();

    // Begin calculation timer
    let beginning = Instant::now();

    // Create threads, each calculating a segment of the entire series
    for i in 0.._thread_count {

        /* 
            pi/4 = sum[0..infinity)( -1^k / (2k+1))
        
        Each thread calculates one of the windows or ranges
            madhava begin position = i * window size
            madhava end (exclusive) position = (i + 1) * windows size
            accum += (-1^k)/(2k+1)

        */

        let range_start = (i * factors_per_thread) as u64;
        let range_end = ((i + 1) * factors_per_thread) as u64;

        // Create thread and capture join handle
        thread_handles.push(thread::spawn(move || {
            // Each thread accumulates using its slice of denoms
            let mut _accum: f64 = 0.0;

            for i in range_start..range_end {
                if i % 2 == 0 {
                    _accum += 1.0 / (2 * i + 1) as f64;
                } else {
                    _accum -= 1.0 / (2 * i + 1) as f64;
                }
            }

            // Each thread returns its accumulated result
            return _accum;
        }));
    }

    // Wait for all the threads to complete
    let mut rv = Vec::new();
    for jh in thread_handles {
        rv.push(jh.join().unwrap());
    }

    // Aggregate results from threads
    let mut _partial: f64 = 0.0;
    for r in rv {
        _partial += r;
    }

    // Finally, calculate value of pi.
    let pi: f64 = 4.0 * (_partial);
    println!("----------------------------------------");
    println!("pi = {:?} which differs from PI constant by {:?} ", pi, diff_of_pi_const(&pi));
    println!("({} factors calculated in {} ms)", 
        (_thread_count * factors_per_thread).to_formatted_string(&SystemLocale::default().unwrap())
        , beginning.elapsed().as_millis());
    println!("----------------------------------------");
}


#[derive(Clap)]
#[clap(version = "0.1", author = "Rust4.Net")]
struct Opts {
    /// Number of Madhava factors to calculate in order of magnitude (10^m)
    #[clap(short = 'm', long = "magnitude", default_value = "6")]
    oom: u8,
}

const MAX_MAGNITUDE:u8 = 10;  // failure to alloc membry may occur over 10 (e.g., 10^10)
fn is_valid_magnitude(val: u8) -> Result<(), String> {
    if val > 0 && val <= MAX_MAGNITUDE {
        Ok(())
    } else {
        // clap automatically adds "error: " to the beginning
        // of the message.
        Err(String::from("magnitude must be within range (0-8]."))
    }
}

fn diff_of_pi_const(calcd_pi:&f64) -> f64 {
    return calcd_pi - std::f64::consts::PI;
}