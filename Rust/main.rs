mod cluster;
mod io;
mod pts;

fn main() {    
    let limit = io::read_limit_from(String::from("distancia.txt"));
    let points: Vec<pts::Point> = io::read_points_from(String::from("entrada.txt"));
    let groups: Vec<Vec<&pts::Point>> = cluster::clustering(&points,limit);

    io::print_groups(String::from("saida.txt"),&groups);
    io::print_result(String::from("result.txt"),cluster::sse(&points,&groups)); 
}