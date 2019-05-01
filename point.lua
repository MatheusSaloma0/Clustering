p = {}

--[[
 - Os pontos possuem d coordenadas(dimensoes).
 - Cada ponto sera representado por uma tabela contendo dois campos:
    - id: Um numero como identificador, que corresponde a ordem deste 
    - durante o processo de leitura do arquivo.
    - coordenates: Uma tabela de numeros contendo as coordenadas do ponto.   
 - Isso eh realizado na funcao createPointsTable.
--]]

--[[
 - Esta funcao eh responsavel por determinar todas as coordenadas de um ponto a partir 
 - de uma string. 
 - inputs: um string contendo as coordenadas de um ponto.
 - output: tabela de numeros contendo as coordenadas de um ponto.
--]]
function p.getCoordenates (line)
    coord = {}

    for value in string.gmatch(line,"[^ ]+") do 
        table.insert(coord,tonumber(value)) 
    end
    
    return coord
end

--[[
 - Dada uma tabela de linhas, de cada linha sao retiradas coordenadas que serao
 - associadas a um ponto. Cada ponto recebera um identificador relacionado a sua 
 - posicao na tabela de linhas e sera adicionado a uma tabela de pontos.
 - inputs: Uma tabela de pontos(vazia) e uma tabela de linhas(strings).
 - output: Uma tabela de pontos modificada.
--]]
function p.createPointsTable (lines)
    points = {}

    for i = 1,#lines do
        point = {id = i,coordenates = p.getCoordenates(lines[i])}
        table.insert(points,point)
    end

    return points
end

return p