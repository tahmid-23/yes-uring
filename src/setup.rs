use crate::cli;
use io_uring::{cqueue, opcode, squeue, types};

pub fn make_buffer(args: &cli::Args) -> Vec<u8> {
    if args.strings.is_empty() {
        return Vec::from(b"y\n");
    }

    let mut joined = args.strings.join(" ");
    joined.push('\n');

    Vec::from(joined.as_bytes())
}

pub fn create_ring(args: &cli::Args, fd_opt: Option<std::os::fd::RawFd>) -> io_uring::IoUring {
    let mut ring_builder: io_uring::Builder<squeue::Entry, cqueue::Entry> =
        io_uring::IoUring::builder();
    if let Some(idle) = args.sqpoll {
        ring_builder.setup_sqpoll(idle);
        if let Some(fd) = fd_opt {
            ring_builder.setup_attach_wq(fd);
        }
    }

    ring_builder
        .build(args.ring_capacity)
        .expect("failed to create io_uring instance")
}

pub fn setup_ring<'a, 'b>(ring: &'a io_uring::IoUring, buffer: &'b mut [u8])
where
    'b: 'a,
{
    use std::os::fd::AsRawFd;

    ring.submitter()
        .register_files(&[std::io::stdout().as_raw_fd()])
        .expect("failed to register stdout with io_uring");

    let bufs = &[libc::iovec {
        iov_base: buffer.as_mut_ptr() as *mut _,
        iov_len: buffer.len(),
    }];

    // SAFETY: `buffer` lives as long as `ring`
    unsafe {
        ring.submitter()
            .register_buffers(bufs)
            .expect("failed to register buffer");
    }

    if ring.params().is_setup_sqpoll() {
        ring.submit().expect("failed to submit empty operations");
    }
}

pub fn create_write_entries(args: &cli::Args, buffer: &Vec<u8>) -> Vec<squeue::Entry> {
    vec![
        opcode::WriteFixed::new(types::Fixed(0), buffer.as_ptr(), buffer.len() as u32, 0).build();
        args.ring_capacity as usize
    ]
}

pub fn compute_thread_count(args: &cli::Args) -> usize {
    if args.cpu_threads {
        num_cpus::get()
    } else {
        args.threads
    }
}
