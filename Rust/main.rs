use std::io::{BufRead, BufReader};
use std::fs::File;
use std::fs;

struct Point {
    coordenates: Vec<f64>,
    index: usize,
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
    let mut sum = 0.0 as f64;
    let n = p1.coordenates.len();
    for i in 0..n {
        sum += (p1.coordenates[i] - p2.coordenates[i]).powf(2.0);
    }
    return sum.sqrt();
}

fn clustering(points: &Vec<Point>,limit: f64) -> Vec<Vec<usize>> {
    let mut groups: Vec<Vec<usize>> = vec![vec![]];
	groups[0].push(points[0].index);

    for i in 1..points.len() {
		let mut leader = true;
		
		for j in 0..groups.len() {
			if euclidean_distance(&points[i],&points[groups[j][0]-1]) <= limit {
				groups[j].push(points[i].index);
				leader = false;
				break;
			}
		}
		if leader {
			groups.push(vec![points[i].index]);
		}
	}
    return groups;
}

fn centroid (points: &Vec<Point>, group: &Vec<usize>) -> Point {
    let mut coords: Vec<f64> = Vec::new(); 
    let len_coords = points[0].coordenates.len();
    let len_group = group.len();
   
    for i in 0..len_coords {
        coords.push(0.0);
        for j in 0..len_group {
            coords[i] += points[group[j]-1].coordenates[i];
        }
        coords[i] /= len_group as f64;
    }
    return Point{coordenates: coords, index: 0};
}

fn sse (points: &Vec<Point>, groups: &Vec<Vec<usize>>) -> f64{
    let mut sum = 0 as f64;

    for group in groups {
        let c: Point = centroid(&points,&group);
        for point in group {
            sum += euclidean_distance(&points[point-1],&c).powf(2.0);
        }
    }
    return sum;
}

fn main() {    
    let limit = read_limit_from(String::from("baterias/bateria1/distancia.txt"));
    let points: Vec<Point> = read_points_from(String::from("baterias/bateria1/entrada.txt"));
    
    //Teste:
    println!("{}",limit);
    
    //Teste:
    for i in 0..points.len(){
        println!("{:?} {}",points[i].coordenates,points[i].index);
    }
    let groups: Vec<Vec<usize>> = clustering(&points,limit);
    
    for i in &groups {
        for j in i {
            print!("{} ",j);
        }
        print!("\n");
    }
    println!("{}",sse(&points,&groups));   
}