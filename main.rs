use std::io::{BufRead, BufReader};
use std::fs::File;
use std::fs;

struct Point {
    coords: Vec<f64>,
    index: i32,
}

fn main() {
    let path = "baterias/bateria1/";
    let reader = BufReader::new(File::open(format!("{}{}",path,"entrada.txt")).expect("Erro na abertura do arquivo"));
    
    let content = fs::read_to_string(format!("{}{}",path,"distancia.txt")).expect("Erro na abertura do arquivo");
    let limit = content.trim().parse::<f64>().unwrap();
    println!("Distancia: {}",limit);

    let mut points : Vec<Point> = Vec::new();
    let mut i = 1;  
    for line in reader.lines() {
        let mut c : Vec<f64> = Vec::new();
        for word in line.unwrap().split_whitespace() {
            c.push(word.parse::<f64>().unwrap());
        }
        let p = Point{coords: c,index: i};
        i += 1;
        points.push(p);
    }
    
    for i in 0..points.len(){
        println!("{:?} {}",points[i].coords,points[i].index);
    }   
}