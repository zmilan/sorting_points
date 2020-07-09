mod points;
use points::*;

fn main() {
    let mut list: Vec<Point> = vec![Point { x: -2.0, y: 3.0 }, Point { x: 9.0, y: 1.0 }, Point { x: 2.0, y: 2.0 }, Point { x: -2.0, y: -2.0 }, Point { x: -3.0, y: 1.0 }, Point { x: 2.0, y: -1.0 }];
    let center = find_center(&list);
    println!("Not sorted list: {:?}", list);
    println!("Center: {:?}", center);
    list.sort_by(|a, b| less(&center, a, b));
    println!("Sorted list with less: {:?}", list);
    println!("#######################################################################################################");

    let mut list2: Vec<Point> = vec![Point { x: -2.0, y: 3.0 }, Point { x: 7.0, y: 8.0 }, Point { x: 2.0, y: 2.0 }, Point { x: -2.0, y: -2.0 }, Point { x: -3.0, y: 1.0 }, Point { x: 2.0, y: -1.0 }];
    let center2 = find_center(&list2);
    println!("Not sorted list2: {:?}", list2);
    println!("Center2: {:?}", center2);
    list2.sort_by(|a, b| sort_by_atan2(&center2, a, b));
    println!("Sorted list with sort_by_atan2: {:?}", list2);
    println!("#######################################################################################################");

    let mut list3: Vec<Point> = vec![Point { x: 3.0, y: 9.0 }, Point { x: -2.0, y: 3.0 }, Point { x: 7.0, y: 8.0 }, Point { x: 2.0, y: 2.0 }, Point { x: -2.0, y: -2.0 }, Point { x: -3.0, y: 1.0 }, Point { x: 2.0, y: -1.0 }];
    let center3 = Point { x: 1.0, y: 4.0 };
    println!("Not sorted list3: {:?}", list3);
    println!("Center3: {:?}", center3);
    list3.sort_by(|a, b| sort_by_atan2(&center3, a, b));
    println!("Sorted list with sort_by_atan2: {:?}", list3);
    println!("#######################################################################################################");

}
