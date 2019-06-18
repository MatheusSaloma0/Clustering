path = "baterias/bateria1/"

class Point:
    def __init__(self, coordenates, index): 
        self.coordenates = coordenates
        self.index = index 

# Calcula a distancia euclidiana entre dois pontos.
def euclideanDistance(p1, p2):
    sum = 0.0

    for i in range(len(p1.coordenates)):
        sum += (p1.coordenates[i] - p2.coordenates[i])**2
    return sum**(0.5)

# A funcao consiste em dividir um conjunto de pontos em N grupos, sendo que
# os pontos pertencentes a um mesmo grupo possuem uma distancia euclidiana
# menor ou igual a um limite pre-determinado.
def clustering(points,limit):
    groups = [ [points[0]] ]

    for i in range(1,len(points)):
        leader = True

        for j in range(len(groups)):
            if (euclideanDistance(points[i], groups[j][0]) <= limit):
                groups[j].append(points[i])
                leader = False
                break
            if leader:
                newGroup = [points[i]]
                groups.append(newGroup)
    return groups

# Ler de distancia.txt
with open(path + "distancia.txt",'r') as f_dist:
    limit = f_dist.read()
f_dist.closed

# Lista vazia de pontos
points = []
# Ler de entrada.txt
with open(path + "entrada.txt", 'r') as f_entrada:
    filecontents = f_entrada.readlines()
    i = 1
    for line in filecontents:
        p_coords = line[:-1]
        p_coords = [float(x) for x in p_coords.split()]
        points.append(Point(p_coords,i))
        i = i + 1
f_entrada.closed

# Imprimir lista de pontos (TESTE)
for element in points:
    print(element.coordenates,element.index)

groups = []
groups = clustering(points,limit)

for g in groups:
	for p in g:
		print(p.index)
	print("\n")
