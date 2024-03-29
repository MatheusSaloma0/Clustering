myio = require("myio")
point = require("point")
cluster = require("cluster")

lines = {}
lines = myio.readFileLines("entrada.txt")
limit = myio.readLimit("distancia.txt")

if limit < 0 then
    print("O limite deve ser um numero nao negativo")
else 
    points = point.createPointsTable(lines)
    groups = cluster.clustering(points,limit)

    myio.write_groups("saida.txt",groups)
    myio.write_sse("result.txt",cluster.sse(groups))
end
