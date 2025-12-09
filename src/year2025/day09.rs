use crate::util::point::Point;

pub fn parse(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(",").unwrap();
            Point::new(x.parse().unwrap(), y.parse().unwrap())
        })
        .collect()
}

pub fn part_1(coords: &[Point]) -> u64 {
    let mut max_area = 0;
    for p1 in coords {
        for p2 in coords {
            if p1 != p2 {
                max_area = max_area.max(area(p1, p2));
            }
        }
    }

    max_area
}

pub fn part_2(coords: &[Point]) -> u64 {
    todo!()
}

#[inline]
pub fn area(p1: &Point, p2: &Point) -> u64 {
    let x_distance = p1.x.abs_diff(p2.x) as u64;
    let y_distance = p1.y.abs_diff(p2.y) as u64;

    (x_distance + 1) * (y_distance + 1)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_area() {
        assert_eq!(50, area(&Point { x: 2, y: 5 }, &Point { x: 11, y: 1 }))
    }
}
