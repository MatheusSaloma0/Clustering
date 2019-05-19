package main

import ("fmt")

func main() {
	lines := readFileLines("entrada.txt")
	limit := readLimit("distancia.txt")

	if  limit < 0 {
		fmt.Println("O limite deve ser um ponto flutuante nao negativo")
	}else {
		points := make([]Point,0)
		points = fillPointArray(points,lines)

		groups := clustering(points,limit)
	
		writeResults("saida.txt",groups)
		writeResults("result.txt",SSE(groups))
	}
}
