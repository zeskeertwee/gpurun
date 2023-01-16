use std::env;
use std::process::Command;

const ENV_VARIABLES: [(&'static str, &'static str); 2] = [
    ("__NV_PRIME_RENDER_OFFLOAD", "1"),
    ("__GLX_VENDOR_LIBRARY_NAME", "nvidia"),
];

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        println!("[gpurun] No argument provided!");
        println!("[gpurun] Usage: {} [program]", env::args().next().unwrap());
        return;
    }

    let mut args_iter = args.into_iter();

    let status = Command::new(args_iter.next().unwrap())
        .args(args_iter)
        .envs(ENV_VARIABLES)
        .status();

    match status {
        Ok(code) => println!("[gpurun] Process exited with {}", code),
        Err(e) => println!("[gpurun] Failed to execute process: {}", e),
    }
}
