# Estrutura representando um ponto de d coordenadas(dimensoes).
# Cada ponto apresenta um identificador(index) que corresponde a ordem deste
# durante o processo de leitura do arquivo contendo todos os pontos.
class Point:
    # Inicializa um ponto.
    def __init__(self, coordenates, index): 
        self.coordenates = coordenates
        self.index = index 

    # Metodo ToString do ponto.
    def __str__(self):
        return str(self.index)
    
    # Calcula a distancia euclidiana entre dois pontos.
    def euclideanDistance(self,point):
        sum = 0.0

        for i in range(len(self.coordenates)):
            sum += (self.coordenates[i] - point.coordenates[i])**2
        return sum**(0.5)
