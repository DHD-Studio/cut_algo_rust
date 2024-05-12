use std::io::Read;
use std::io::Write;
use std::fs::File;

const UNIT_LENGTH:i32 = 5800;
const CUT_WASTE:i32 = 2;


fn main() {
    let mut input_file = File::open("input.txt").expect("File not found");
    let mut content = String::new();
    input_file.read_to_string(&mut content).expect("something went wrong reading the file");

    // set output file
    let mut output_file = File::create("output.txt").expect("file not found");

    // parse input to vector
    let v: Vec<String> = content.split_whitespace().map(|s| s.to_string()).collect();
    let mut nums:Vec<i32> = vec![];
    for i in (0..v.len()).step_by(2) {
        for _j in 0..v[i].parse::<i32>().unwrap() {
            nums.push(v[i+1].parse::<i32>().unwrap());
        }
    }

    // sort
    nums.sort();

    // output information of input data
    output_file.write(b"There are ").expect("write failed.");
    output_file.write(nums.len().to_string().as_bytes()).expect("write failed.");
    output_file.write(b" datas.\n\n").expect("write failed.");

    output_file.write(b"There are all of the data content:\n").expect("write failed.");

    for i in nums.clone() {
        output_file.write(i.to_string().as_bytes()).expect("write failed.");
        output_file.write(b" ").expect("write failed.");
    }

    output_file.write(b"\n").expect("write failed.");

    // calculate
    let mut waste = 0;
    let mut aluminum = 0;

    let mut aluminum_list:Vec<i32> = vec![];

    // output result
    output_file.write(b"\nNeed to use ").expect("write failed");
    output_file.write(aluminum.to_string().as_bytes()).expect("write failed");
    output_file.write(b" * 5800mm aluminum materials.\n").expect("write failed");
    output_file.write(b"Wasted ").expect("write failed.");
    output_file.write(waste.to_string().as_bytes()).expect("write failed.");
    output_file.write(b" mm of aluminum.").expect("write failed.");
}
