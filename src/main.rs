use std::thread;

mod cli;
mod setup;

unsafe fn print_forever(ring: &mut io_uring::IoUring, write_entries: &[io_uring::squeue::Entry]) {
    loop {
        let cap = ring.submission().capacity();
        let len = ring.submission().len();
        let free = cap - len;

        if free > 0 {
            // SAFETY: write_entries are valid
            unsafe {
                ring.submission()
                    .push_multiple(&write_entries[..free])
                    .expect("failed to push write operations");
            }
        }

        if !ring.params().is_setup_sqpoll() {
            ring.submit().expect("failed to submit write operations");
        }

        while ring.completion().next().is_some() {}
    }
}

unsafe fn spawn_threads(args: &cli::Args, buffer: &mut [u8], entries: &[io_uring::squeue::Entry]) {
    use std::os::fd::AsRawFd;

    let threads = setup::compute_thread_count(&args);
    thread::scope(|s| {
        let mut sqpoll_fd = None;
        for _ in 0..threads {
            let ring = match sqpoll_fd {
                Some(fd) => setup::create_ring(&args, Some(fd)),
                None => {
                    let ring = setup::create_ring(&args, None);
                    sqpoll_fd = Some(ring.as_raw_fd());
                    ring
                }
            };
            setup::setup_ring(&ring, buffer);

            s.spawn(|| {
                let mut ring = ring;
                // SAFETY: write_entries are valid
                unsafe {
                    print_forever(&mut ring, &entries);
                }
            });
        }
    });
}

fn main() {
    use clap::Parser;
    let args = cli::Args::parse();

    let mut buffer = setup::make_buffer(&args);
    let entries = setup::create_write_entries(&args, &buffer);

    // SAFETY: write_entries are valid
    unsafe {
        spawn_threads(&args, &mut buffer, &entries);
    }
}
