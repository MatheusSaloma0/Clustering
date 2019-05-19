package main

import (
	"strings"
	"strconv"
)

/**
 * Estrutura representando um ponto de d coordenadas(dimensoes).
 * Cada ponto apresenta um identificador(id) que corresponde a ordem deste 
 * durante o processo de leitura do arquivo contendo todos os pontos. 
 */
type Point struct {
	id int
	coordenates []float64
}

/**
 * Esta funcao eh responsavel por determinar todas as coordenadas de um ponto a partir 
 * de uma string. 
 * inputs: um string contendo as coordenadas de um ponto.
 * output: um vetor de numeros em ponto flutuante contendo as coordenadas 
 * de um ponto.
 */
func getCoordenates (line string) ([]float64) {
	floats := make([]float64, 0)

	// A string eh convertida em um conjunto de substrings, considerando espaços 
	// em branco como delimitador.
	aux := strings.Fields(line)

	for _,v := range aux {
		// Cada substring eh convertida para ponto flutuante.
		value,err := strconv.ParseFloat(v,64)

		if err != nil {
			panic(err)
		} else {
			floats = append(floats,value)
		}
	}
	return floats
}

/**
 * Dado um vetor de linhas, de cada linha sao retiradas coordenadas que serao
 * associadas a um ponto. Cada ponto recebera um identificador relacionado a sua 
 * posicao no vetor de linhas e sera adicionado ao vetor de pontos, que sera 
 * retornado ao final da funcao.
 * inputs: Um vetor de pontos(vazio) e um vetor de linhas(strings).
 * output: Um vetor de pontos modificado.
 */
func fillPointArray (points []Point,lines []string) ([]Point){
	for i := range lines {
		new := Point{}
		new.coordenates = getCoordenates(lines[i])
		new.id = i+1
		points = append(points,new)
	}
	return points
}
