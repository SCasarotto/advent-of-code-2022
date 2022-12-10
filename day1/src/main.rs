fn main() {
    // Load data from input file
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    // split input by new line character
    let split_by_line = input.split("\n");
    // create a vector to store totals with a starting value of 0
    let mut totals: Vec<i32> = vec![0];
    for line in split_by_line {
        // check if line is empty
        if line.is_empty() {
            // add 0 to totals
            totals.push(0);
        } else {
            // convert to i32
            let num = line.parse::<i32>().unwrap();
            let totals_length = totals.len();
            // add to last element in totals
            totals[totals_length - 1] += num;
        }
    }
    // sort totals descending
    totals.sort_by(|a, b| b.cmp(a));
    // print first element
    println!("Part 1: {}", totals[0]);
    // 67658
    // print sum of first 3 elements
    println!("Part 2: {}", &totals[0..3].iter().sum::<i32>());
    // 200158
}
