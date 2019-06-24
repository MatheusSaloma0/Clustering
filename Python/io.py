import point

# Dado um arquivo de entrada, le um numero de ponto flutuante.
# Esse valor representa a distancia limite.
def readLimit_from(filename):
    with open(filename,'r') as filehandle:
        limit = float(filehandle.read())
    filehandle.closed
    return limit

# Dado um arquivo de entrada onde cada linha representa as coordenadas
# de um ponto, a funcao cria pontos associando as coordenadas de cada linha
# a um identificador e insere esses pontos em uma lista.
def readPoints_from(filename):
    points = []
    with open(filename, 'r') as filehandle:
        filecontents = filehandle.readlines()
      
        for i,line in enumerate(filecontents,start = 1):
            p_coords = [float(x) for x in line[:-1].split()]
            points.append(point.Point(p_coords,i))
            i += 1
    filehandle.closed
    return points

# Imprime os grupos formados pela funcao clustering() em um arquivo.
def printGroups(groups,filename):
    with open(filename, 'w') as filehandle:
        result = ""
        for group in groups:
            result = result + " ".join(x.__str__() for x in group)+"\n\n"
        filehandle.write(result[:-2])
    filehandle.closed

# Imprime o valor calculado pela funcao sse() em um arquivo. 
def printResult(result, filename):
    with open(filename, 'w') as filehandle:
        filehandle.write("{0:.4f}".format(result))
    filehandle.closed
