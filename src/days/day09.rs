use std::cmp::{max, min};

const INPUT: &str = include_str!("../../inputs/input09.txt");

pub fn part1() -> i64 {
    // we optimize it by
    // ordering all tiles by the distance to the farthest corner in the square opened by minx, maxx, miny, maxy
    // then take the one tile at a time (with max distance)
    // if its distance to farthest corner is less than the diagonal of the already found largest square, break the loop
    // if not compare the new tile's distance to all tiles in the set already taken and find the max square
    let tiles: Vec<(i64, i64)> = INPUT
        .lines()
        .map(|line| {
            let mut s = line.split(',');
            (
                s.next().unwrap().parse::<i64>().unwrap(),
                s.next().unwrap().parse::<i64>().unwrap(),
            )
        })
        .collect();
    let xmin = tiles.iter().map(|t| t.0).min().unwrap();
    let xmax = tiles.iter().map(|t| t.0).max().unwrap();
    let ymin = tiles.iter().map(|t| t.1).min().unwrap();
    let ymax = tiles.iter().map(|t| t.1).max().unwrap();
    let mut sorted_tiles: Vec<(&(i64, i64), i64)> = tiles
        .iter()
        .map(|t| {
            (
                t,
                (max(t.0 - xmin + 1, xmax - t.0 + 1) * max(t.1 - ymin + 1, ymax - t.1 + 1)),
            )
        })
        .collect();
    sorted_tiles.sort_unstable_by_key(|k| std::cmp::Reverse(k.1));
    let mut maxarea = 0i64;
    for i in 1..sorted_tiles.len() {
        if sorted_tiles[i].1 <= maxarea {
            break;
        }
        for j in 0..i {
            let a = sorted_tiles[i].0;
            let b = sorted_tiles[j].0;
            let area = ((a.0 - b.0).abs() + 1) * ((a.1 - b.1).abs() + 1);
            if area > maxarea {
                maxarea = area;
            }
        }
    }

    maxarea
}

pub fn point_is_on_vertex(point: &(i64, i64), vertex: &(&(i64, i64), &(i64, i64))) -> bool {
    // we only have vertical or horizontal vertices
    if point.0 == vertex.0.0 && point.0 == vertex.1.0 {
        // vertical vertex
        return point.1 >= min(vertex.0.1, vertex.1.1) && point.1 <= max(vertex.0.1, vertex.1.1);
    }
    if point.1 == vertex.0.1 && point.1 == vertex.1.1 {
        // vertical vertex
        return point.0 >= min(vertex.0.0, vertex.1.0) && point.0 <= max(vertex.0.0, vertex.1.0);
    }
    false
}
//- point is inside polygon
pub fn point_is_inside(point: &(i64, i64), vertices: &Vec<(&(i64, i64), &(i64, i64))>) -> bool {
    let mut inside = false;
    for vertex in vertices {
        if point_is_on_vertex(point, vertex) {
            // println!("      Point {:?} is on vertex {:?}", point, vertex);
            return true;
        }
        // check if ray to the right touches vertex and intersection is to the right of point
        let intersect = ((vertex.0.1 > point.1) != (vertex.1.1 > point.1))
            && (point.0
                < (vertex.0.0 - vertex.1.0) * (point.1 - vertex.0.1) / (vertex.0.1 - vertex.1.1)
                    + vertex.0.0);

        if intersect {
            inside = !inside;
        }
    }
    inside
}

pub fn part2() -> i64 {
    let tiles: Vec<(i64, i64)> = INPUT
        .lines()
        .map(|line| {
            let mut s = line.split(',');
            (
                s.next().unwrap().parse::<i64>().unwrap(),
                s.next().unwrap().parse::<i64>().unwrap(),
            )
        })
        .collect();
    let mut vertices: Vec<(&(i64, i64), &(i64, i64))> = Vec::with_capacity(tiles.len());
    for i in 0..tiles.len() - 1 {
        vertices.push((&tiles[i], &tiles[i + 1]));
    }
    vertices.push((tiles.last().unwrap(), tiles.first().unwrap()));
    let mut maxarea = 0i64;

    // for each rectangle (unordered combination of pairs):
    // check if all 4 corners (2 remaining only) are inside the polygon built by the point connections
    // then check if all sides of the rectangle do not intersect with any of the polygon edges
    // For checking if a point is inside polygon:
    // Point in Polygon ray-casting algorithm? Or is there a simpler approach because our polygon's vertices all
    // have 90 degree angles?

    // for all rectangles
    // let mut k = 0usize;
    for i in 0..tiles.len() - 1 {
        'point2: for j in i + 1..tiles.len() {
            // k += 1;
            // println!("{}", k);
            let a = tiles[i];
            let b = tiles[j];
            let area = ((a.0 - b.0).abs() + 1) * ((a.1 - b.1).abs() + 1);
            // println!("Checking tiles {:?} and {:?} with area {}", a, b, area);
            // if area is smaller than existing largest continue
            if area <= maxarea {
                continue;
            }
            // if all 4 corners are in polygon
            if point_is_inside(&(a.0, b.1), &vertices) && point_is_inside(&(b.0, a.1), &vertices) {
                // println!(" all 4 corners are inside");
                // if no side of the rectangle intersects with polygon edges
                let xmin = min(a.0, b.0);
                let xmax = max(a.0, b.0);
                let ymin = min(a.1, b.1);
                let ymax = max(a.1, b.1);

                for vertex in &vertices {
                    let minxv = min(vertex.0.0, vertex.1.0);
                    let maxxv = max(vertex.0.0, vertex.1.0);
                    let minyv = min(vertex.0.1, vertex.1.1);
                    let maxyv = max(vertex.0.1, vertex.1.1);
                    // left side // (xmin, ymin) -> (xmin, ymax)
                    // only interested in horizontal vertices
                    if vertex.0.0 != vertex.1.0 {
                        if xmin > minxv && xmin < maxxv && ymin < vertex.0.1 && ymax > vertex.0.1 {
                            // println!(
                            //     " left side of rectangle {:?}{:?} intercepts vertex {:?}",
                            //     a, b, vertex
                            // );
                            continue 'point2;
                        }
                        // right side // (xmax, ymin) -> (xmax, ymax)
                        if xmax > minxv && xmax < maxxv && ymin < vertex.0.1 && ymax > vertex.0.1 {
                            // println!(
                            //     " right side of rectangle {:?}{:?} intercepts vertex {:?}",
                            //     a, b, vertex
                            // );
                            continue 'point2;
                        }
                    }
                    // only interested in horizontal vertices
                    if vertex.0.1 != vertex.1.1 {
                        if ymin > minyv && ymin < maxyv && xmin < vertex.0.0 && xmax > vertex.0.0 {
                            // println!(
                            //     " bottom side of rectangle {:?}{:?} intercepts vertex {:?}",
                            //     a, b, vertex
                            // );
                            continue 'point2;
                        }
                        if ymax > minyv && ymax < maxyv && xmin < vertex.0.0 && xmax > vertex.0.0 {
                            // println!(
                            //     " top side of rectangle {:?}{:?} intercepts vertex {:?}",
                            //     a, b, vertex
                            // );
                            continue 'point2;
                        }
                    }
                }

                // now complete rectangle is inside polygon and we have a new max
                maxarea = area;
            } else {
                continue;
            }
        }
    }
    maxarea
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(part1(), 4767418746);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(), 1461987144);
    }
}
