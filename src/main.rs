use std::{env, str, process};

fn parse_digits(t_num: &str) -> Vec<u32> {

    t_num.chars().filter_map(|a| a.to_digit(10)).collect()

}

fn join_nums(nums: Vec<u32>) -> u64 {

    let str_nums: Vec<String> = nums.iter().map(|n| n.to_string()).collect();

    str_nums.join("").parse::<u64>().unwrap()

}

fn help_message() {

    println!("The following arguments exist:
    -h, lists arguments
    -b, displays memory info in bytes
    -k, displays memory info in kilobytes (default)
    -m, displays memory info in megabytes
    -g, displays memory info in gigabytes"
    );

    process::exit(0);

}

fn main() {

    let args: Vec<String> = env::args().collect();

    let mut memory_prefix: u64 = 1024;

    if args.len() == 2 {
        match args[1].as_str() {
            "-b" => memory_prefix = 1,
            "-k" => (),
            "-m" => memory_prefix = 1048576,
            "-g" => memory_prefix = 1073741824,
            _ => help_message(), 
        };
    }

    else if args.len() > 2 { help_message(); }
    
    let vm_stat_output: process::Output = process::Command::new("sh")
        .args(["-c", "vm_stat"])
        .output()
        .expect("Something went wrong");
    
    let sysctl_output: Vec<u8> = process::Command::new("sh")
        .args(["-c", "sysctl hw.memsize"])
        .output()
        .expect("Something went wrong")
        .stdout;
    
    let mut lines: str::Lines = str::from_utf8(&vm_stat_output.stdout).unwrap().lines();
    let line_length: u8 = str::from_utf8(&vm_stat_output.stdout).unwrap().lines().count() as u8;

    let mut elements: Vec<u64> = vec![];

    for _x in 0..line_length {

        elements.push(join_nums(parse_digits(lines.next().unwrap())));

    }

    let total_memory: u64 = join_nums(parse_digits(std::str::from_utf8(&sysctl_output).unwrap())) / memory_prefix;
    let active_memory: u64 = (elements[0] * elements[2]) / memory_prefix;
    let wired_memory: u64 = (elements[0] * elements[6]) / memory_prefix;
    let free_memory: u64 = (elements[0] * elements[1]) / memory_prefix;

    println!("{0: <5} {1: >10} {2: >10} {3: >10} {4: >10}", "", "total", "active", "wired", "free");
    println!("{0: <5} {1: >10} {2: >10} {3: >10} {4: >10}", "Mem:", total_memory, active_memory, wired_memory, free_memory);

    process::exit(0);

}
