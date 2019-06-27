use crate::pts;

// Consiste em separar um conjunto de pontos em N grupos, sendo que
// os pontos pertencentes a um mesmo grupo possuem uma distancia euclidiana
// menor ou igual a um limite pre-determinado.
pub fn clustering (points: &Vec<pts::Point>,limit: f64) -> Vec<Vec<&pts::Point>> {
    let mut groups: Vec<Vec<&pts::Point>> = vec![vec![]];
	groups[0].push(&points[0]);

    for i in 1..points.len() {
		let mut leader = true;
		
		for j in 0..groups.len() {
			if pts::euclidean_distance(&points[i],&groups[j][0]) <= limit {
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

// O centroide eh o ponto representativo de um grupo e eh calculado
// como o centro de massa do grupo.
fn centroid (points: &Vec<pts::Point>, group: &Vec<&pts::Point>) -> pts::Point {
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
    return pts::Point{coordenates: coords, index: 0};
}

// Calcula a soma das distancias euclidianas quadradas(SSE) entre os pontos
// pertencentes a cada um dos grupos. Para tanto, eh utilizado o centroide de cada grupo.
pub fn sse (points: &Vec<pts::Point>, groups: &Vec<Vec<&pts::Point>>) -> f64{
    let mut sum = 0 as f64;

    for group in groups {
        let c: pts::Point = centroid(&points,group);
        for point in group {
            sum += pts::euclidean_distance(point,&c).powf(2.0);
        }
    }
    return sum;
}