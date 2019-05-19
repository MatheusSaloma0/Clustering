package main

import (
	"bufio"
	"os"
	"strconv"
	"fmt"
)

/**
 * Cada linha do arquivo eh adicionada a um vetor. 
 * inputs: o nome de um arquivo.
 * output: um vetor contendo as linhas do arquivo.
 */
func readFileLines (filename string) ([]string) {
	file, err := os.Open(filename)

	if err != nil {
		panic(err)
	}
	defer file.Close()

	var line []string
	scanner := bufio.NewScanner(file)
	
	for scanner.Scan() {
		line = append(line, scanner.Text())
	}

	if scanner.Err() != nil {
		panic(scanner.Err())
	}

	return line
}

/**
 * Dado um arquivo de entrada, le e armazena a distancia limite.
 * inputs: o nome de um arquivo.
 * output: um numero de ponto flutuante.
 */
func readLimit (filename string) (float64) {
	file, err := os.Open(filename)

	if err != nil {
		panic(err)
	}
	defer file.Close()

	var limit float64
	scanner := bufio.NewScanner(file)

	scanner.Scan() 
	limit,err = strconv.ParseFloat(scanner.Text(),64)

	if(err != nil){
		panic(err)
	}

	if scanner.Err() != nil {
		panic(scanner.Err())
	}

	return limit
}

/**
 * Funcao responsavel por salvar os resultados em um arquivo. A forma como os 
 * resultados serao escritos no arquivo dependem do tipo da variavrel recebida
 * pela interface "i". 
 * inputs: o nome de um arquivo e uma variavel generica.
 * output: nenhum.
 */
func writeResults (filename string, i interface{}) {
	file, err := os.Create(filename)
	
	if err != nil {
		panic(err)
	}
	defer file.Close()

	switch v := i.(type) {
		// Caso criado para imprimir o valor calculado pelo SSE dos grupos de pontos.
		case float64:
			_, err = file.WriteString(fmt.Sprintf("%.4f",v))

        		if err != nil {
				panic(err)
        		}
		
		// Caso criado para imprimir como os pontos foram agrupados.
		case [][]Point:
			for i := range v {
				for j := range v[i] {
					if j != len(v[i])-1 {
						_, err = file.WriteString(fmt.Sprintf("%d ",v[i][j].id))
					} else {
						_,err = file.WriteString(fmt.Sprintf("%d",v[i][j].id))
					}
					
					if err != nil {
						panic(err)
					}
				}
				if i != len(v)-1 {
					file.WriteString("\n\n")
				}	
			}
	}
}
