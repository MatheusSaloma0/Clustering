// Estrutura representando um ponto de d coordenadas(dimensoes).
// Cada ponto apresenta um identificador(index) que corresponde a ordem deste
// durante o processo de leitura do arquivo contendo todos os pontos.
pub struct Point {
    pub coordenates: Vec<f64>,
    pub index: usize,
}

// Implementacao do ToString de um ponto.
impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"{}",self.index)
    }
}

// Inicializa um ponto a partir de um vetor de ponto flutuante e um inteiro.
// pub fn init (c: Vec<f64>,i: usize)->Point{
//     return Point{coordenates: c, index:i};
// }

// Calcula a distancia euclidiana entre dois pontos.
pub fn euclidean_distance (p1: &Point, p2: &Point) ->  f64 {
    let mut sum = 0.0 as f64;
    let n = p1.coordenates.len();

    for i in 0..n {
        sum += (p1.coordenates[i] - p2.coordenates[i]).powf(2.0);
    }
    return sum.sqrt();
}