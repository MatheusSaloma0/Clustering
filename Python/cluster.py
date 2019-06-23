import point
 
# Retorna o ponto Lider de um grupo
def group_leader(group):
    return group[0]

# Consiste em separar um conjunto de pontos em N grupos, sendo que
# os pontos pertencentes a um mesmo grupo possuem uma distancia euclidiana
# menor ou igual a um limite pre-determinado.
def clustering(points,limit):
    groups = [ [points[0]] ]

    for i in range(1,len(points)):
        leader = True

        for j in range(len(groups)):
            if (points[i].euclideanDistance(group_leader(groups[j])) <= limit):
                groups[j].append(points[i])
                leader = False
                break
        if leader:
            newGroup = [points[i]]
            groups.append(newGroup)
    return groups

# O centroide eh o ponto representativo de um grupo e eh calculado
# como o centro de massa do grupo.
def centroid (group):
    coords = []
    group_size = len(group)
    for i in range(len(group_leader(group).coordenates)):
        coords.append(0)
        for j in range(group_size):
            coords[i] += group[j].coordenates[i]
        coords[i] /= float(group_size)
    return point.Point(coords,0)

# Calcula a soma das distancias euclidianas quadradas(SSE) entre os pontos
# pertencentes a cada um dos grupos. Para tanto, eh utilizado o centroide de cada grupo.
def sse(groups):
    sum = 0.0

    for i in range(len(groups)):
        c = centroid(groups[i])
        for j in range(len(groups[i])):
            sum += (groups[i][j].euclideanDistance(c))**2
    return sum