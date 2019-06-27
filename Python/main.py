import i_o
import point
import cluster

points = i_o.readPoints_from("baterias/bateria1/entrada.txt")
limit = i_o.readLimit_from("baterias/bateria1/distancia.txt")
groups = cluster.clustering(points,limit)
i_o.printGroups(groups,"saida.txt")
i_o.printResult(cluster.sse(groups),"result.txt")
