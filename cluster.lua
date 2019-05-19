cluster = {}

--[[
 - Calcula a distancia euclidiana entre dois pontos.
 - inputs: dois pontos.
 - output: um numero que representa a distancia entre os pontos. 
--]]
function cluster.euclideanDistance (p1, p2)
    sum = 0

    for i = 1,#(p1.coordenates) do
        sum = sum + ((p1.coordenates[i] - p2.coordenates[i])^2)
    end

    return math.sqrt(sum)
end

--[[
 - A funcao consiste em dividir um conjunto de pontos em N grupos, sendo que 
 - os pontos pertencentes a um mesmo grupo possuem uma distancia euclidiana
 - menor ou igual a um limite pre-determinado.
 - inputs: tabela de pontos e uma distancia limite.
 - output: uma tabela de tabelas(grupos) de pontos. 
--]]
function cluster.clustering (points,limit)
    groups = {{points[1]}}

    for i = 2,#points do
		leader = true
		
        for j = 1,#groups do 
            if cluster.euclideanDistance(points[i],groups[j][1]) <= limit then
                table.insert(groups[j],points[i])
				leader = false
                break
            end   
        end 
        if leader then
            newGroup = {points[i]}
            table.insert(groups,newGroup)
        end
    end

	return groups
end

--[[
 - O centroide de um grupo eh o ponto representativo de um determinado grupo de 
 - pontos e eh calculado como o centro de massa do grupo.
 - inputs: um grupo de pontos. 
 - output: um ponto que representa o centroide do grupo.
--]]
function cluster.centroid (group)
    coord = {}

    for i = 1,#group[1].coordenates do
        coord[i] = 0
    end    
    
    for i = 1,#coord do
        for j = 1,#group do
            coord[i] = coord[i] + group[j].coordenates[i]
        end
        coord[i] = coord[i]/ #group
    end
    cm = {coordenates = coord}
        
    return cm
end

--[[
 - Essa funcao calcula a soma das distancias euclidianas quadradas (SSE) entre os pontos 
 - pertencentes a cada um dos grupos. Para tanto, eh utilizado o centroide de cada grupo.
 - inputs: uma tabela de tabelas(grupos) de pontos.
 - output: um numero que corresponde ao SSE entre os grupos.
--]]
function cluster.sse (groups)
    sum = 0
    for i = 1,#groups do
	c = cluster.centroid(groups[i])
        for j = 1,#groups[i] do
        sum = sum + cluster.euclideanDistance(groups[i][j],c)^2
        end
    end
    return sum
end

return cluster
