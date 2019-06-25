use std::io::{BufRead, BufReader};
use std::fs::File;
use std::fs;
use std::f64;

struct Point {
    coordenates: Vec<f64>,
    index: i32,
}

fn read_limit_from(filename: String) -> f64 {
    let content = fs::read_to_string(filename).expect("Erro na abertura do arquivo");
    return content.trim().parse::<f64>().unwrap();
}

fn read_points_from(filename: String) -> Vec<Point> {
    let reader = BufReader::new(File::open(filename).expect("Erro na abertura do arquivo"));

    let mut points: Vec<Point> = Vec::new();
    let mut i = 1;  
    for line in reader.lines() {
        let mut coords: Vec<f64> = Vec::new();
        for word in line.unwrap().split_whitespace() {
            coords.push(word.parse::<f64>().unwrap());
        }
        points.push(Point{coordenates: coords,index: i});
        i += 1;
    } 
    return points;
}

fn euclidean_distance(p1: &Point, p2: &Point) ->  f64 {
    let mut sum = 0.0_f64;
    let n = p1.coordenates.len();
    for i in 0..n {
        sum += (p1.coordenates[i] - p2.coordenates[i]).powf(2.0);
    }
    return sum.sqrt();
}

fn clustering(points: Vec<&Point>,limit: f64) -> Vec<Vec<&Point>> {
    let mut groups: Vec<Vec<&Point>> = Vec::new();
    groups[0][0] = points[0];

    for i in 1..points.len() {
		let mut leader = true;
		
		for j in 0..groups.len() {
			if euclidean_distance(points[i],groups[j][0]) <= limit {
				groups[j].push(points[i]);
				leader = false;
				break;
			}
		}
		if leader {
			let mut new_g: Vec<&Point> = Vec::new();
			new_g[0] = points[i];
			groups.push(new_g);
		}
	}
    return groups;
}

// fn centroid (group: Vec<&Point>) -> Point {
// }

// fn sse (groups: Vec<Vec<&Point>>) -> f64{
// }

fn main() {    
    let limit = read_limit_from(String::from("baterias/bateria1/distancia.txt"));
    let points: Vec<Point> = read_points_from(String::from("baterias/bateria1/entrada.txt"));
    
    //Teste:
    println!("{}",limit);
    
    //Teste:
    for i in 0..points.len(){
        println!("{:?} {}",points[i].coordenates,points[i].index);
    }
    // let groups: Vec<Vec<&Point>> = clustering(points,limit);  
}