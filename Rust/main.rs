mod cluster;
mod io;
mod pts;

fn main() {    
    let limit = io::read_limit_from(String::from("baterias/bateria1/distancia.txt"));
    let points: Vec<pts::Point> = io::read_points_from(String::from("baterias/bateria1/entrada.txt"));
    let groups: Vec<Vec<&pts::Point>> = cluster::clustering(&points,limit);

    io::print_groups(&groups);
    io::print_result(cluster::sse(&points,&groups)); 
}