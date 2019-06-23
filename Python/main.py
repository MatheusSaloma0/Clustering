import io
import point
import cluster

points = io.readPoints_from("entrada.txt")
limit = io.readLimit_from("distancia.txt")
groups = cluster.clustering(points,limit)
io.printGroups(groups,"saida.txt")
io.printResult(cluster.sse(groups),"result.txt")
