use std::io::{BufRead, BufReader, BufWriter, Write};
use std::fs::File;
use std::fs;
use crate::pts;

// Dado um arquivo de entrada, le um numero de ponto flutuante.
// Esse valor representa a distancia limite.
pub fn read_limit_from (filename: String) -> f64 {
    let content = fs::read_to_string(filename).expect("Erro na abertura do arquivo");
    return content.trim().parse::<f64>().unwrap();
}

// Dado um arquivo de entrada onde cada linha representa as coordenadas
// de um ponto, a funcao cria pontos associando as coordenadas de cada linha
// a um identificador e insere esses pontos em um vetor.
pub fn read_points_from (filename: String) -> Vec<pts::Point> {
    let reader = BufReader::new(File::open(filename).expect("Erro na abertura do arquivo"));
    let mut points: Vec<pts::Point> = Vec::new();
    let mut i = 1;  

    for line in reader.lines() {
        let mut coords: Vec<f64> = Vec::new();
        for word in line.unwrap().split_whitespace() {
            coords.push(word.parse::<f64>().unwrap());
        }
        points.push(pts::Point{coordenates: coords,index: i});
        i += 1;
    } 
    return points;
}

// Imprime os grupos formados pela funcao clustering() em um arquivo.
pub fn print_groups (groups: &Vec<Vec<&pts::Point>>) {
    let fsaida = File::create(String::from("saida.txt")).expect("Erro na abertura do arquivo");
	let mut saida_buff = BufWriter::new(fsaida);
    let mut result: Vec<String> = Vec::new();

    for group in groups {
        let point_str: Vec<_> = group.iter().map(ToString::to_string).collect();
        result.push(point_str.join(" "));
    }
    write!(saida_buff,"{}",result.join("\n\n")).unwrap();
}

// Imprime o valor calculado pela funcao sse() em um arquivo. 
pub fn print_result (result: f64) {
    let fresult = File::create(String::from("result.txt")).expect("Erro na abertura do arquivo");
	let mut result_buff = BufWriter::new(fresult);
	write!(result_buff, "{:.4}", result).unwrap();
}