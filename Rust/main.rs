use std::io::{BufRead, BufReader, BufWriter, Write};
use std::fs::File;
use std::fs;

struct Point {
    coordenates: Vec<f64>,
    index: usize,
}

fn read_limit_from (filename: String) -> f64 {
    let content = fs::read_to_string(filename).expect("Erro na abertura do arquivo");
    return content.trim().parse::<f64>().unwrap();
}

fn read_points_from (filename: String) -> Vec<Point> {
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

fn euclidean_distance (p1: &Point, p2: &Point) ->  f64 {
    let mut sum = 0.0 as f64;
    let n = p1.coordenates.len();
    for i in 0..n {
        sum += (p1.coordenates[i] - p2.coordenates[i]).powf(2.0);
    }
    return sum.sqrt();
}

fn clustering (points: &Vec<Point>,limit: f64) -> Vec<Vec<&Point>> {
    let mut groups: Vec<Vec<&Point>> = vec![vec![]];
	groups[0].push(&points[0]);

    for i in 1..points.len() {
		let mut leader = true;
		
		for j in 0..groups.len() {
			if euclidean_distance(&points[i],&groups[j][0]) <= limit {
				groups[j].push(&points[i]);
				leader = false;
				break;
			}
		}
		if leader {
			groups.push(vec![ &points[i] ]);
		}
	}
    return groups;
}

fn centroid (points: &Vec<Point>, group: &Vec<&Point>) -> Point {
    let mut coords: Vec<f64> = Vec::new(); 
    let len_coords = points[0].coordenates.len();
    let len_group = group.len();
   
    for i in 0..len_coords {
        coords.push(0.0);
        for j in 0..len_group {
            coords[i] += group[j].coordenates[i];
        }
        coords[i] /= len_group as f64;
    }
    return Point{coordenates: coords, index: 0};
}

fn sse (points: &Vec<Point>, groups: &Vec<Vec<&Point>>) -> f64{
    let mut sum = 0 as f64;

    for group in groups {
        let c: Point = centroid(&points,group);
        for point in group {
            sum += euclidean_distance(point,&c).powf(2.0);
        }
    }
    return sum;
}

fn results (points: &Vec<Point>, groups: &Vec<Vec<&Point>>) {
    let fsaida = File::create(String::from("saida.txt")).expect("Erro na abertura do arquivo");
	let mut saida_buff = BufWriter::new(fsaida);
    
    for group in groups {
        for point in group {
            write!(saida_buff,"{} ",point.index).unwrap();
        }
        write!(saida_buff,"\n\n").unwrap();
    }

    let fresult = File::create(String::from("result.txt")).expect("Erro na abertura do arquivo");
	let mut result_buff = BufWriter::new(fresult);
	write!(result_buff, "{:.4}", sse(&points,&groups)).unwrap();
}

fn main() {    
    let limit = read_limit_from(String::from("baterias/bateria5/distancia.txt"));
    let points: Vec<Point> = read_points_from(String::from("baterias/bateria5/entrada.txt"));

    let groups: Vec<Vec<&Point>> = clustering(&points,limit);
    results(&points,&groups); 
}