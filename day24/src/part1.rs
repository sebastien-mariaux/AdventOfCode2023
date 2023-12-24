use crate::utils::read_data;
use itertools::Itertools;

pub fn solve_puzzle(file_name: &str, min_coord: isize, max_coord: isize) -> u32 {
    let data = read_data(file_name);

    let equations = data
        .lines()
        .map(|line| {
            let (coords, velocities) = line.split_once("@ ").unwrap();
            let mut split_coords = coords.split(", ");
            let x = split_coords.next().unwrap().parse::<f32>().unwrap();
            let y = split_coords.next().unwrap().parse::<f32>().unwrap();
            let mut split_velocities = velocities.split(", ");
            let vx = split_velocities
                .next()
                .unwrap()
                .trim()
                .parse::<f32>()
                .unwrap();
            let vy = split_velocities
                .next()
                .unwrap()
                .trim()
                .parse::<f32>()
                .unwrap();

            let coeff_directeur = vy / vx;
            let ordonne_origine = y - coeff_directeur * x;
            (x, y, vx, vy, coeff_directeur, ordonne_origine)
        })
        .collect::<Vec<(f32, f32, f32, f32, f32, f32)>>();

    equations
        .into_iter()
        .combinations(2)
        .map(|equations| {
            let equation1 = equations[0];
            let equation2 = equations[1];
            intersection_in_range(&equation1, &equation2, min_coord, max_coord)
        })
        .filter(|x| *x)
        .count() as u32
}

fn intersection_in_range(
    equation1: &(f32, f32, f32, f32, f32, f32),
    equation2: &(f32, f32, f32, f32, f32, f32),
    min_coord: isize,
    max_coord: isize,
) -> bool {
    let (x1, y1, vx1, vy1, coeff_directeur1, ordonne_origine1) = equation1;
    let (x2, y2, vx2, vy2, coeff_directeur2, ordonne_origine2) = equation2;
    if coeff_directeur1 == coeff_directeur2 {
        if ordonne_origine1 == ordonne_origine2 {
            return true;
        } else {
            return false;
        }
    }

    let x = (ordonne_origine2 - ordonne_origine1) / (coeff_directeur1 - coeff_directeur2);
    let y = coeff_directeur1 * x + ordonne_origine1;
    if x >= min_coord as f32
        && x <= max_coord as f32
        && y >= min_coord as f32
        && y <= max_coord as f32
    {
        let already_crossed = vx1 < &00.0 && x1 < &x
            || vx1 > &0.0 && x1 > &x
            || vx2 < &00.0 && x2 < &x
            || vx2 > &0.0 && x2 > &x
            || vy1 < &00.0 && y1 < &y
            || vy1 > &0.0 && y1 > &y
            || vy2 < &00.0 && y2 < &y
            || vy2 > &0.0 && y2 > &y;
        if !already_crossed {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_data() {
        assert_eq!(2, solve_puzzle("test_data", 7, 27));
    }

    #[test]
    // #[ignore]
    fn test_solution() {
        assert_eq!(
            18098,
            solve_puzzle("input", 200000000000000, 400000000000000)
        );
    }
}
