use std::io::Read;
use std::io::Write;
use std::fs::File;

const unit_length:i32 = 5800;
const cut_waste:i32 = 2;

fn cut(remain_length:i32, cut_length:i32) -> i32 {
    return remain_length - (cut_length + cut_waste);
}

fn print_array(array:Vec<i32>) {
    for i in 0..array.len() {
        print!("{} ", array[i]);
    }

    print!("\n");
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
    let mut flag:bool = false; // true = even, false = odd
    let mut parsed_nums:Vec<i32> = Vec::new();
    for i in &v {
        if flag {
            count = i.parse::<i32>().unwrap();
            flag = false;
        } else {
            for j in 0..count {
                parsed_nums.push(i.parse::<i32>().unwrap());
            }
            flag = true;
        }
    }

    // sort
    nums.sort();

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

        println!("This time waste {} mm of aluninum.", remain_length);
        waste += remain_length;
        aluminum += 1;
        print!("ans = ");
        print_array(ans);
        println!("");
        if index == 0 {
            break;
        }
    }

    println!("Need to use {} *5800mm aluminum materials.", aluminum);
    // println!("Wasted mm of aluminum.", waste);

    output_file.write(b"hi").expect("write failed");
}
