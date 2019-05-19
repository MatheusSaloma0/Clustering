myio = {}

--[[
 - Cada linha do arquivo eh adicionada a uma tabela. 
 - inputs: o nome de um arquivo.
 - output: uma tabela contendo as linhas do arquivo.
--]]
function myio.readFileLines(filename)
    local file = assert(io.open(filename,"rb"),"Erro na abertura do arquivo")
    lines = {}
    for line in file:lines() do
        lines[#lines+1] = line
    end
    file:close()
    
    return lines
end

--[[
 - Dado um arquivo de entrada, le e armazena a distancia limite.
 - inputs: o nome de um arquivo.
 - output: um numero que corresponde a distancia limite.
--]]
function myio.readLimit(filename)
    local file = assert(io.open(filename,"rb"),"Erro na abertura do arquivo")
    limit = file:read("*n")
    file:close()

    return limit
end

--[[
 - Imprime os resultados calculados pela soma das distancias euclidianas 
 - quadradas (SSE) em um arquivo. 
 - inputs: o nome de um arquivo e o valor da SSE
 - output: nenhum
--]]
function myio.write_sse (filename, value)
    local file = assert(io.open(filename, "w"), "Erro na criacao do arquivo")
    file:write(string.format("%.4f", value))
    file:flush();
    io.close(file)
end

--[[
 - Imprime os grupos determinados pelo algoritmo de clustering em um arquivo. 
 - inputs: o nome de um arquivo e a tabela contendo os grupos.
 - output: nenhum
--]]
function myio.write_groups(filename,groups)
    local file = assert(io.open(filename, "w"), "Erro na criacao do arquivo")

    for i,group in pairs(groups) do 
        for j,point in pairs(group) do
            if j ~= #group then  
                file:write(point.id.." ")
            else 
                file:write(point.id)
            end                
        end
        if i ~= #groups then
            file:write("\n\n")
        end
    end
    
    file:flush();
    io.close(file)
end

return myio