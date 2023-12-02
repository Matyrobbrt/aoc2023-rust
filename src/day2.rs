struct Cubes {
    red: u32,
    blue: u32,
    green: u32,
}

pub(crate) fn run() {
    let part2 = false;

    let input = include_str!("../inputs/day2.txt");

    let lines = input.split("\n");

    let mut sum = 0;
    let mut part2sum = 0;
    for (i, line) in lines.enumerate() {
        let sets = line.split(":").last().unwrap().trim().split(";");

        let mut minimum = Cubes {
            red: 0,
            blue: 0,
            green: 0,
        };

        let mut valid = true;
        for mut set in sets {
            let mut cubes = Cubes { red: 0, blue: 0, green: 0 };

            set = set.trim();
            let cube_info = set.split(",");
            for mut cube in cube_info {
                cube = cube.trim();
                let num_type = cube.split(" ").collect::<Vec<&str>>();
                let amount = num_type[0].parse::<u32>().unwrap();

                if num_type[1] == "red" {
                    cubes.red += amount;
                } else if num_type[1] == "green" {
                    cubes.green += amount;
                } else {
                    cubes.blue += amount;
                }
            }


            if valid {
                valid = cubes.red <= 12 && cubes.green <= 13 && cubes.blue <= 14;
            }

            if cubes.red > minimum.red { minimum.red = cubes.red; }
            if cubes.blue > minimum.blue { minimum.blue = cubes.blue; }
            if cubes.green > minimum.green { minimum.green = cubes.green; }
        }

        if valid {
            sum += i + 1;
        }

        part2sum += (minimum.red * minimum.blue * minimum.green);
    }

    println!("Part 1: {sum}");
    print!("Part 2: {part2sum}");
}