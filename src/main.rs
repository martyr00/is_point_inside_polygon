fn main() {
    let input_figure: Vec<[i32; 2]> = [[1, 1], [6, 2], [6, 3], [4, 5], [2, 4], [1, 1]].to_vec();
    let input_points:[[i32; 2]; 2] = [[5, 5], [5, 2]];

    let point1 = input_points[1];
    let point2 = input_points[0];
    let polygon = input_figure;

    println!("inside: {}", is_point_inside_polygon(point1, &polygon));
    println!("outside: {}", is_point_inside_polygon(point2, &polygon));
}

fn is_point_inside_polygon(point: [i32; 2], polygon: &Vec<[i32; 2]>) -> bool {
    let mut count  = 0;
    let mut direction = 0;

    while count < (polygon.len() - 1) {
        let res = (point[0] - polygon[count][0]) * (polygon[count+1][1] - polygon[count][1]) - (point[1] - polygon[count][1]) * (polygon[count+1][0] - polygon[count][0]);

        if res < 0 {
            if direction > 0 {
                return false;
            }
            direction -= 1;
        } else if res > 0 {
            if direction < 0 {
                return false;
            }
            direction += 1;
        } else {
            return true;
        }

        count += 1;
    }

    return true;
}