use std::thread;

fn main() {
    println!("Begin program.");

    const THREAD_COUNT: usize = 5;
    let factors_per_thread: u32 = 99999;

    // Create vectors of positive and negative denominators
    let (pos_denoms, neg_denoms) = create_factor_vectors(THREAD_COUNT as u32 * factors_per_thread);
    // Calculate the number of denominators each thread will process. Up to THREAD_COUNT-1 denominators
    //  might be abandoned for each.
    let _pos_denoms_window_size = (pos_denoms.len() / THREAD_COUNT) as u32;
    let _neg_denoms_window_size = (neg_denoms.len() / THREAD_COUNT) as u32;

    // Create threads and pass slices of each set of denoms
    let mut hv = Vec::new();
    for i in 0..THREAD_COUNT {
        let range_start = i * _neg_denoms_window_size as usize;
        let range_end = (i + 1) * _neg_denoms_window_size as usize;

        let pos_slice = Vec::from(&pos_denoms[range_start..range_end]);
        let neg_slice = Vec::from(&neg_denoms[range_start..range_end]);

        // Create thread and capture join handle
        hv.push(thread::spawn(move || {
            // Each thread accumulates using its slice of denoms
            let mut _accum: f64 = 0.0;

            for i in 0..pos_slice.len() {
                _accum += 1.0 / pos_slice[i] as f64;
            }
            for i in 0..neg_slice.len() {
                _accum -= 1.0 / neg_slice[i] as f64;
            }

            // Each thread returns its accumulated result
            return _accum;
        }));
    }

    // Wait for all the threads to complete
    let mut rv = Vec::new();
    for jh in hv {
        rv.push(jh.join().unwrap());
    }

    // Aggregate results from threads
    let mut _partial: f64 = 0.0;
    for r in rv {
        _partial += r;
    }

    // Finally, calculate value of pi.
    let pi: f64 = 4.0 * (1.0 - _partial);
    println!("pi = {:?}", pi);

    println!("\nGoodbye. End of program.");
}

fn create_factor_vectors(factors: u32) -> (Vec<u64>, Vec<u64>) {
    let last_factor = 3 + 2 * factors;
    let pos: Vec<u64> = (3..last_factor).step_by(4).map(u64::from).collect();
    let neg: Vec<u64> = (5..last_factor).step_by(4).map(u64::from).collect();
    (pos, neg)
}
