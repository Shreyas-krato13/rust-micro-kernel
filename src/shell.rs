use spin::Mutex;

const INPUT_SIZE: usize = 128;

// SHELL INPUT BUFFER

struct Shell {
    buffer: [u8; INPUT_SIZE],
    len: usize,
}

impl Shell {
    const fn new() -> Self {
        Self {
            buffer: [0; INPUT_SIZE],
            len: 0,
        }
    }
}

// GLOBAL SHELL

static SHELL: Mutex<Shell> = Mutex::new(Shell::new());

// START SHELL

pub fn start() {
    crate::kprintln!("");
    crate::kprint!("Micro-OS> ");
}

// HANDLE CHARACTER INPUT

pub fn handle_character(character: u8) {
    let mut shell = SHELL.lock();

    if shell.len < INPUT_SIZE {
        let index = shell.len;

        shell.buffer[index] = character;
        shell.len += 1;

        crate::kprint!("{}", character as char);
    }
}

// HANDLE BACKSPACE

pub fn handle_backspace() {
    let mut shell = SHELL.lock();

    if shell.len > 0 {
        shell.len -= 1;

        crate::vga_buffer::WRITER.lock().delete_char();
    }
}

// HANDLE ENTER

pub fn handle_enter() {
    let mut command_buffer = [0u8; INPUT_SIZE];

    let len;

    {
        let mut shell = SHELL.lock();

        len = shell.len;

        command_buffer[..len]
            .copy_from_slice(&shell.buffer[..len]);

        shell.len = 0;
    }

    crate::kprintln!("");

    let command = core::str::from_utf8(
        &command_buffer[..len]
    ).unwrap_or("");

    execute_command(command);

    crate::kprint!("Micro-OS> ");
}

// COMMAND PROCESSOR

fn execute_command(command: &str) {

    match command {

        "help" => {
            crate::kprintln!("Available commands:");
            crate::kprintln!("help    - Show commands");
            crate::kprintln!("about   - About Micro-OS");
            crate::kprintln!("version - OS version");
            crate::kprintln!("echo    - Display message");
            crate::kprintln!("mem     - Show memory information");
            crate::kprintln!("alloc N - Allocate N bytes");
        }

        "about" => {
            crate::kprintln!("Micro-OS");
            crate::kprintln!("A minimal operating system");
            crate::kprintln!("Developed using Rust.");
        }

        "version" => {
            crate::kprintln!("Micro-OS Version 0.1.0");
        }

        "mem" => {
            let (used, free) = crate::memory::get_stats();

            crate::kprintln!("========== MEMORY MANAGER ==========");
            crate::kprintln!("Total Memory Pool : {} KB",
                crate::memory::POOL_SIZE / 1024);
            crate::kprintln!("Used Memory       : {} bytes", used);
            crate::kprintln!("Free Memory       : {} bytes", free);
            crate::kprintln!("====================================");
        }
        
        "tasks" => {
    crate::scheduler::run_demo();
}

        _ => {

            if let Some(message) = command.strip_prefix("echo ") {

                crate::kprintln!("{}", message);

            } else if let Some(size_text) = command.strip_prefix("alloc ") {

                match size_text.parse::<usize>() {

                    Ok(size) => {
                        match crate::memory::allocate(size) {

                            Some(offset) => {
                                crate::kprintln!(
                                    "Allocated {} bytes at offset {}",
                                    size, offset
                                );
                            }

                            None => {
                                crate::kprintln!("Memory allocation failed!");
                            }
                        }
                    }

                    Err(_) => {
                        crate::kprintln!("Invalid memory size!");
                    }
                }

            } else if !command.is_empty() {

                crate::kprintln!("Unknown command: {}", command);

            }
        }
    }
}
