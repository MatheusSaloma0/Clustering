# Clustering
Trabalhos de Linguagens de Programação (Go, Lua, Haskell, Python e Rust)

Agrupamento de dados multidimensionais é um dos problemas mais comuns na área de
mineração de dados. Esse problema consiste em dividir um conjunto de pontos em um
espaço multidimensional em um determinado número pré-especificado de grupos de
modo que os pontos pertencentes a um mesmo grupo estão mais relacionados entre si e
menos relacionados em relação aos pontos associados aos outros grupos. 

Este trabalho realiza o agrupamento dos pontos obtidos a partir do arquivo *entrada.txt* 
(contem as coordenadas dos pontos) com o auxilio do arquivo *distancia.txt* (contem a 
distância limite que será utilizada para avaliar a relação entre os pontos). O critério 
de qualidade utilizado é a soma das distâncias euclidianas quadradas (SSE) entre os 
pontos pertencentes a cada um dos grupos.

Os resultados obtidos serão armazenados nos arquivos *saida.txt* e *result.txt* que contém 
respectivamente, os grupos obtidos e o calculo da SSE dos grupos.
