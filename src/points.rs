use std::cmp::Ordering;

#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

pub fn less (center: &Point, a: &Point, b: &Point) -> Ordering {
    if a.x - center.x >= 0.0 && b.x - center.x < 0.0 {
        return Ordering::Less;
    }
    if a.x - center.x < 0.0 && b.x - center.x >= 0.0 {
        return Ordering::Greater;
    }
    if a.x - center.x == 0.0 && b.x - center.x == 0.0 {
        if a.y - center.y >= 0.0 || b.y - center.y >= 0.0 {
            if a.y > b.y {
                return Ordering::Less;
            }
            return Ordering::Greater;
        }
        if b.y > a.y {
            return Ordering::Less;
        }
        return Ordering::Greater;
    }

    // compute the cross product of vectors (center -> a) x (center -> b)
    let det = (a.x - center.x) * (b.y - center.y) - (b.x - center.x) * (a.y - center.y);
    if det < 0.0 {
        return Ordering::Less;
    }
    if det > 0.0 {
        return Ordering::Greater;
    }

    // points a and b are on same lane from the center, check which point is closer to the center
    let d1 = (a.x - center.x) * (a.x - center.x) + (a.y - center.y) * (a.y - center.y);
    let d2 = (b.x - center.x) * (b.x - center.x) + (b.y - center.y) * (b.y - center.y);

    if d1 > d2 {
        return Ordering::Less;
    }

    Ordering::Greater
}

pub fn sort_by_atan2(center: &Point, a: &Point, b: &Point) -> Ordering {
    let a_atan2 = (a.y - center.y).atan2(a.x - center.x);
    let b_atan2 = (b.y - center.y).atan2(b.x - center.x);
    if a_atan2 > b_atan2 {
        Ordering::Less
    } else if b_atan2 > a_atan2 {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

pub fn find_center(list: &[Point]) -> Point {
    let mut sum_x = 0.0;
    let mut sum_y= 0.0;
    let len : f64 = list.len() as f64;
    for i in list {
        sum_x += i.x;
        sum_y += i.y;
    }

    Point {
        x: sum_x / len,
        y: sum_y / len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_by_atan2() {
        let mut list = vec![Point { x: -2.0, y: 3.0 }, Point { x: 14.0, y: 9.0 }, Point { x: 7.0, y: 8.0 }, Point { x: 2.0, y: 2.0 }, Point { x: -2.0, y: -2.0 }, Point { x: -3.0, y: 1.0 }, Point { x: 2.0, y: -1.0 }];
        let center = find_center(&list);
        list.sort_by(|a, b| sort_by_atan2(&center, a, b));
        let result = vec![Point { x: -2.0, y: 3.0 }, Point { x: 7.0, y: 8.0 }, Point { x: 14.0, y: 9.0 }, Point { x: 2.0, y: -1.0 }, Point { x: 2.0, y: 2.0 }, Point { x: -2.0, y: -2.0 }, Point { x: -3.0, y: 1.0 }];
        assert_eq!(list == result, true);
    }

    #[test]
    fn test_sort_by_atan2_fail() {
        let mut list = vec![Point { x: -2.0, y: 3.0 }, Point { x: 14.0, y: 9.0 }, Point { x: 4.0, y: 6.0 }, Point { x: 7.0, y: 8.0 }, Point { x: 2.0, y: 2.0 }, Point { x: -2.0, y: -2.0 }, Point { x: -3.0, y: 1.0 }, Point { x: 2.0, y: -1.0 }];
        let center = find_center(&list);
        list.sort_by(|a, b| sort_by_atan2(&center, a, b));
        let result = vec![Point { x: 14.0, y: 9.0 }, Point { x: -2.0, y: 3.0 }, Point { x: 4.0, y: 6.0 }, Point { x: 7.0, y: 8.0 }, Point { x: 2.0, y: 2.0 }, Point { x: -2.0, y: -2.0 }, Point { x: -3.0, y: 1.0 }, Point { x: 2.0, y: -1.0 }];
        assert_ne!(list == result, true);
    }

    #[test]
    fn test_sort_with_less() {
        let mut list = vec![Point { x: 6.0, y: 10.0 }, Point { x: -2.0, y: 3.0 }, Point { x: 9.0, y: 1.0 }, Point { x: 2.0, y: 2.0 }, Point { x: -2.0, y: -2.0 }, Point { x: -3.0, y: 1.0 }, Point { x: 2.0, y: -1.0 }];
        let center = find_center(&list);
        list.sort_by(|a, b| less(&center, a, b));
        let result = vec![Point { x: 6.0, y: 10.0 }, Point { x: 2.0, y: 2.0 }, Point { x: 9.0, y: 1.0 }, Point { x: 2.0, y: -1.0 }, Point { x: -2.0, y: -2.0 }, Point { x: -3.0, y: 1.0 }, Point { x: -2.0, y: 3.0 }];
        assert_eq!(list == result, true);
    }

    #[test]
    fn test_sort_with_less_fail() {
        let mut list = vec![Point { x: 6.0, y: 10.0 }, Point { x: -2.0, y: 3.0 }, Point { x: 10.0, y: 4.0 }, Point { x: 9.0, y: 1.0 }, Point { x: 2.0, y: 2.0 }, Point { x: -2.0, y: -2.0 }, Point { x: -3.0, y: 1.0 }, Point { x: 2.0, y: -1.0 }];
        let center = find_center(&list);
        list.sort_by(|a, b| less(&center, a, b));
        let result = vec![Point { x: -2.0, y: 3.0 }, Point { x: 6.0, y: 10.0 }, Point { x: 10.0, y: 4.0 }, Point { x: 9.0, y: 1.0 }, Point { x: 2.0, y: 2.0 }, Point { x: -2.0, y: -2.0 }, Point { x: -3.0, y: 1.0 }, Point { x: 2.0, y: -1.0 }];
        assert_ne!(list == result, true);
    }

    #[test]
    fn test_sort_by_atan2_custom_center() {
        let mut list = vec![Point { x: 3.0, y: 9.0 }, Point { x: 11.0, y: 5.0 }, Point { x: -2.0, y: 3.0 }, Point { x: 7.0, y: 8.0 }, Point { x: 2.0, y: 2.0 }, Point { x: -2.0, y: -2.0 }, Point { x: -3.0, y: 1.0 }, Point { x: 2.0, y: -1.0 }];
        let center = Point { x: 1.0, y: 4.0 };
        list.sort_by(|a, b| sort_by_atan2(&center, a, b));
        let result = vec![Point { x: 3.0, y: 9.0 }, Point { x: 7.0, y: 8.0 }, Point { x: 11.0, y: 5.0 }, Point { x: 2.0, y: 2.0 }, Point { x: 2.0, y: -1.0 }, Point { x: -2.0, y: -2.0 }, Point { x: -3.0, y: 1.0 }, Point { x: -2.0, y: 3.0 }];
        assert_eq!(list == result, true);
    }

    #[test]
    fn test_sort_by_atan2_custom_center_fail() {
        let mut list = vec![Point { x: 3.0, y: 9.0 }, Point { x: 11.0, y: 5.0 }, Point { x: 7.0, y: 8.0 }, Point { x: -2.0, y: 3.0 }, Point { x: 7.0, y: 8.0 }, Point { x: 2.0, y: 2.0 }, Point { x: -2.0, y: -2.0 }, Point { x: -3.0, y: 1.0 }, Point { x: 2.0, y: -1.0 }];
        let center = Point { x: 1.0, y: 4.0 };
        list.sort_by(|a, b| sort_by_atan2(&center, a, b));
        let result = vec![Point { x: 11.0, y: 5.0 }, Point { x: 3.0, y: 9.0 }, Point { x: 7.0, y: 8.0 }, Point { x: -2.0, y: 3.0 }, Point { x: 7.0, y: 8.0 }, Point { x: 2.0, y: 2.0 }, Point { x: -2.0, y: -2.0 }, Point { x: -3.0, y: 1.0 }, Point { x: 2.0, y: -1.0 }];
        assert_ne!(list == result, true);
    }
}