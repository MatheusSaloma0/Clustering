package main

import ("math")

/**
 * Calcula a distancia euclidiana entre dois pontos.
 * inputs: dois pontos.
 * output: um ponto flutuante. 
 */
func euclideanDistance (p1 Point,p2 Point) (float64){
	sum := 0.0

	for i := range p1.coordenates {
		sum += math.Pow(p1.coordenates[i] - p2.coordenates[i],2)
	} 
	return math.Sqrt(sum)
}

/**
 * A funcao consiste em dividir um conjunto de pontos em N grupos, sendo que 
 * os pontos pertencentes a um mesmo grupo possuem uma distancia euclidiana
 * menor ou igual a um limite pre-determinado.
 * inputs: vetor de pontos e uma distancia limite.
 * output: um vetor de vetores de pontos. 
 */
func clustering (points []Point, limit float64) ([][]Point) {
	groups := make([][]Point,1)
	groups[0] = make([]Point,1);
	groups[0][0] = points[0]

	for i := 1; i < len(points); i++ {
		leader := true
		
		for j := range groups {
			if euclideanDistance(points[i],groups[j][0]) <= limit {
				groups[j] = append(groups[j],points[i])
				leader = false
				break
			}
		}
		if leader {
			newGroup := make([]Point,1);
			newGroup[0] = points[i]
			groups = append(groups,newGroup)
		}
	}
	return groups
}

/**
 * O centroide de um grupo eh o ponto representativo de um determinado grupo de 
 * pontos e eh calculado como o centro de massa do grupo.
 * inputs: um grupo de pontos. 
 * output: um ponto que representa o centroide do grupo.
 */
func centroid (group []Point) (Point){
	cm_coords := make([]float64,len(group[0].coordenates))

	for i := range cm_coords {
		for j := range group {
			cm_coords[i] += group[j].coordenates[i]
		}
		cm_coords[i] /= (float64)(len(group))
	}

	cm := Point{}
	cm.coordenates = cm_coords
	
	return cm
}

/**
 * Essa funcao calcula a soma das distancias euclidianas quadradas (SSE) entre os pontos 
 * pertencentes a cada um dos grupos. Para tanto, eh utilizado o centroide de cada grupo.
 * inputs: um vetor de vetores de pontos.
 * output: um numero de ponto flutuante que corresponde ao SSE entre os grupos.
 */
func sse (groups [][]Point) (float64){
	sum := 0.0

	for i := range groups {
		c := centroid(groups[i])
		for j := range groups[i] {
			sum += math.Pow(euclideanDistance(groups[i][j],c),2)
		}
	}
	return sum
}
