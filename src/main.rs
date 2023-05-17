use std::io::Read;
use std::io::Write;
use std::fs::File;

const unit_length:i32 = 5800;
const cut_waste:i32 = 2;

fn cut(remain_length:i32, cut_length:i32) -> i32 {
    return remain_length - (cut_length + cut_waste);
}

fn min_element(array:Vec<i32>) -> i32 {
    let mut min:i32 = array[0];
    for i in 0..array.len() {
        if array[i] < min {
            min = array[i];
        }
    }

    return min;
}

fn main() {
    let mut nums:Vec<i32> = vec![];
    let mut aluminum:i32 = 0;
    let mut waste:i32 = 0;

    // get input
    let mut input_file = File::open("input.txt").expect("file not found");
    let mut content:String = String::new();
    input_file.read_to_string(&mut content).expect("something went wrong reading the file");

    // debug output file content
    /*
    println!("debug output file content");
    println!("With text:\n{}", content);
    print!("\n");
    */

    // set output file
    let mut output_file = File::create("output.txt").expect("file not found");

    // parse input to vector
    let mut v: Vec<String> = content.split_whitespace().map(|s| s.to_string()).collect();
    for i in (0..v.len()).step_by(2) {
        for j in 0..v[i].parse::<i32>().unwrap() {
            nums.push(v[i+1].parse::<i32>().unwrap());
            // print!("{} {} ", v[i].parse::<i32>().unwrap(), v[i+1].parse::<i32>().unwrap());
        }
    }

    // debug output unsorted vector
    /*
    println!("debug output unsorted vector");
    for i in &mut nums {
        print!("{i} ");
    }
    print!("\n\n");
    */

    let mut count:i32 = 0;

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

    // debug output sorted vector
    /*
    println!("debug output sorted vector");
    for i in &mut nums {
        print!("{i} ");
    }
    print!("\n\n");
    */

    // calculate
    let mut index = nums.len().try_into().unwrap();
    index -= 1;

    loop {
        let mut remain_length = unit_length;
        let mut ans:Vec<i32> = Vec::new();

        while remain_length >= min_element(nums.clone()) + cut_waste {
            index = nums.len() - 1;
            while index > 0 && nums[index] > remain_length + cut_waste {
                index -= 1;
            }

            if index > 0 {
                remain_length = cut(remain_length, nums[index]);
                ans.push(nums[index]);
                nums.remove(index);
            } else if index == 0 {
                remain_length = cut(remain_length, nums[index]);
                ans.push(nums[index]);
                break;
            } else {
                break;
            }
        }

        output_file.write(b"\nThis time waste ").expect("write failed");
        output_file.write(remain_length.to_string().as_bytes()).expect("write failed");
        output_file.write(b" mm of aluninum.\n").expect("write failed");
        waste += remain_length;
        aluminum += 1;
        output_file.write(b"ans = ").expect("write failed");
        for i in 0..ans.len() {
            output_file.write(ans[i].to_string().as_bytes()).expect("write failed");
            output_file.write(b" ").expect("write failed");
        }

        output_file.write(b"\n").expect("write failed");
        if index == 0 {
            break;
        }
    }

    output_file.write(b"\nNeed to use ").expect("write failed");
    output_file.write(aluminum.to_string().as_bytes()).expect("write failed");
    output_file.write(b" * 5800mm aluminum materials.\n").expect("write failed");
    output_file.write(b"Wasted ").expect("write failed.");
    output_file.write(waste.to_string().as_bytes()).expect("write failed.");
    output_file.write(b" mm of aluminum.").expect("write failed.");
}