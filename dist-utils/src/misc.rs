use sysinfo::{ProcessExt, Signal, System, SystemExt};

pub fn stop_process_by_name(name: &str) {
    let mut system = System::new();
    system.refresh_all();
    // Name is truncated here and I don't feel like trying to snag it from the command path
    let name = if name.len() < 15 { name } else { &name[..15] };
    let processes = system.get_process_by_name(name);

    for process in processes {
        process.kill(Signal::Interrupt);
    }
}
