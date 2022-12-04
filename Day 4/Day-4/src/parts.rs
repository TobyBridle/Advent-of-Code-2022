pub fn part_one(lines: Vec<&str>) -> u32 {
    let mut contained = 0;
    for line in lines {
        // Split on the comma
        let mut split = line.split(",");
        // Convert part to a range
        // We can say that it is contained if the lower value and upper values of one
        // part are greater than the lower and upper values of the other part
        let part_one = split.next().unwrap();
        let part_two = split.next().unwrap();
        let part_one_bounds = part_one
            .split('-')
            .map(|bound| return bound.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        let part_two_bounds = part_two
            .split('-')
            .map(|bound| return bound.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        // If the first part is contained in the second part
        if part_one_bounds[0] >= part_two_bounds[0] && part_one_bounds[1] <= part_two_bounds[1] {
            contained += 1;
        } else if part_two_bounds[0] >= part_one_bounds[0]
            && part_two_bounds[1] <= part_one_bounds[1]
        {
            contained += 1;
        }
    }
    contained
}

pub fn part_two(lines: Vec<&str>) -> u32 {
    let mut pairs = 0;
    for line in lines {
        // Split on the comma
        let mut split = line.split(",");
        // Convert part to a range
        // We can say that it is contained if the lower value and upper values of one
        // part are greater than the lower and upper values of the other part
        let part_one = split.next().unwrap().split('-').collect::<Vec<&str>>();
        let part_two = split.next().unwrap().split('-').collect::<Vec<&str>>();

        let mut part_one_iter =
            part_one[0].parse::<u32>().unwrap()..part_one[1].parse::<u32>().unwrap() + 1;
        let mut part_two_iter =
            part_two[0].parse::<u32>().unwrap()..part_two[1].parse::<u32>().unwrap() + 1;

        // We want to check if there are pairs of numbers that are the same
        // We can do this by checking if the intersection of the two ranges is not empty
        // If it is not empty, then we have a pair
        if part_one_iter
            .clone()
            .any(|x| part_two_iter.clone().any(|y| x == y))
        {
            pairs += 1;
        }
    }
    pairs as u32
}
