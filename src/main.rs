use std::process::Command;
use std::env;

fn parse_digits(t_num: &str) -> Vec<u32> {

    t_num
        .chars()
        .filter_map(|a| a.to_digit(10))
        .collect()

}


fn join_nums(nums: Vec<u32>, sep: &str) -> u64 {

    let str_nums: Vec<String> = nums.iter() 
        .map(|n| n.to_string())
        .collect();

    let joined_integer: u64 = str_nums.join(sep).parse::<u64>().unwrap();

    return joined_integer;
    
}


fn main() {

    let args: Vec<String> = env::args().collect();

    let memory_prefix: u64;

    if args.len() > 1 {
        memory_prefix = match args[1].to_string().as_ref() {
            "-k" => 1024,
            "-m" => 1048576,
            "-g" => 1073741824,
            _ => 1024
        };
    }
    else {
        memory_prefix = 1024;
    }
    
    let vm_stat_output = Command::new("sh")
        .arg("-c")
        .arg("vm_stat")
        .output()
        .expect("no work");
    
    let sysctl_output = Command::new("sh")
        .arg("-c")
        .arg("sysctl hw.memsize")
        .output()
        .expect("no work")
        .stdout;
    
    let mut lines = std::str::from_utf8(&vm_stat_output.stdout).unwrap().lines();
    let line_length = std::str::from_utf8(&vm_stat_output.stdout).unwrap().lines().count();

    let mut elements: Vec<u64> = vec![];

    for _x in 0..line_length {

        elements.push(join_nums(parse_digits(lines.next().unwrap()), "").try_into().unwrap());

    }

    let total_memory = join_nums(parse_digits(std::str::from_utf8(&sysctl_output).unwrap()), "") / memory_prefix;
    let active_memory = (elements[0] * elements[2]) / memory_prefix;
    let wired_memory = (elements[0] * elements[6]) / memory_prefix;
    let free_memory = (elements[0] * elements[1]) / memory_prefix;

    println!("{0: <5} {1: >10} {2: >10} {3: >10} {4: >10}", "", "total", "active", "wired", "free");
    println!("{0: <5} {1: >10} {2: >10} {3: >10} {4: >10}", "Mem:", total_memory, active_memory, wired_memory, free_memory);

}