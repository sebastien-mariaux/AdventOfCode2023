use crate::utils::read_data;

pub fn solve_puzzle(file_name: &str) -> usize {
    let data = read_data(file_name);

    let mut trenches: Vec<(isize, isize)> = Vec::new();
    trenches.push((0, 0));

    let mut sides: Vec<((isize, isize), (isize, isize))> = Vec::new();

    for line in data.lines() {
        let mut instructions = line.split(" ");
        instructions.next();
        instructions.next();
        let hexa = instructions.next().unwrap();
        let five_first = &hexa[2..7];
        let one_last = &hexa[7..8];
        // println!("hexa: {}, five_first: {}, one_last: {}", hexa, five_first, one_last);
        let direction = match one_last {
            "0" => "R",
            "1" => "D",
            "2" => "L",
            "3" => "U",
            _ => panic!("Unknown direction: {}", one_last),
        };

        let distance = isize::from_str_radix(five_first, 16).unwrap();
        // println!("hexa {} direction: {}, distance: {}", hexa, direction, distance);

        dig_trenches(&mut sides, direction, distance);
    }

    println!("sides: {:?}", sides);
    let vertices = sides.iter().map(|(s, _)| s).collect::<Vec<_>>();
    println!("vertices: {:?}", vertices);

    let xs = vertices.iter().map(|(x, _)| x).collect::<Vec<_>>();
    let ys = vertices.iter().map(|(_, y)| y).collect::<Vec<_>>();
    println!("xs: {:?}", xs);
    println!("ys: {:?}", ys);
    println!("length of sides {}", sides.len());
    println!("length of vertices {}", vertices.len());

    let mut ys_changed = ys.clone();
    let last_element = ys_changed.remove(0);
    ys_changed.push(last_element);

    let mut xs_changed = xs.clone();
    let last_element = xs_changed.remove(0);
    xs_changed.push(last_element);
    println!("ys_changed: {:?}", ys_changed);
    println!("xs_changed: {:?}", xs_changed);

    let first_sum: isize = xs
        .iter()
        .zip(ys_changed.iter())
        .map(|(x, y)| **x * **y)
        .sum();
    let second_sum: isize = ys
        .iter()
        .zip(xs_changed.iter())
        .map(|(y, x)| **y * **x)
        .sum();

    let result = 0.5 * ((first_sum - second_sum).abs() as f64);

    let perimeter = sides
        .iter()
        .map(|(s, e)| (s.0 - e.0).abs() + (s.1 - e.1).abs())
        .sum::<isize>();
    println!("perimeter: {}", perimeter);
    println!("result: {}", result);

    result as usize + (perimeter as f64 / 2.0) as usize + 1
}

fn dig_trenches(
    sides: &mut Vec<((isize, isize), (isize, isize))>,
    direction: &str,
    distance: isize,
) {
    let last_angle;
    if sides.is_empty() {
        last_angle = (0, 0);
    } else {
        last_angle = sides.last().unwrap().clone().1;
    }

    let new_angle;
    match direction {
        "R" => {
            new_angle = (last_angle.0, last_angle.1 + distance);
        }
        "D" => {
            new_angle = (last_angle.0 + distance, last_angle.1);
        }
        "L" => {
            new_angle = (last_angle.0, last_angle.1 - distance);
        }
        "U" => {
            new_angle = (last_angle.0 - distance, last_angle.1);
        }
        _ => panic!("Unknown direction: {}", direction),
    }
    sides.push((last_angle, new_angle));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_data() {
        assert_eq!(952408144115, solve_puzzle("test_data"));
    }

    #[test]
    // #[ignore]
    fn test_solution() {
        assert_eq!(96556251590677, solve_puzzle("input"));
    }
}
