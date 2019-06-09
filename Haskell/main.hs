import System.IO 
import Data.List
import Text.Printf

-- Estrutura representando um ponto de d coordenadas(dimensoes).
-- Cada ponto apresenta um identificador(index) que corresponde a ordem deste 
-- durante o processo de leitura do arquivo contendo todos os pontos.
data Point = Point {
    coordenates :: [Double],
    index :: Int
    } deriving (Show)

readDouble :: String -> Double
readDouble = read 

-- Cria uma lista contendo as coordenadas de cada ponto.
getCoordenates :: String -> [[Double]]
getCoordenates input = [map readDouble $ words line | line <- lines input]

-- Cria uma lista de pontos a partir da lista de coordenadas. 
createPoints :: [[Double]] -> Int -> [Point]
createPoints [] _ = []
createPoints (x:xs) i = [Point{coordenates = x, index = i}] ++ createPoints xs (i+1)

-- Calcula a distancia entre dois pontos
distance :: Point -> Point -> Double
distance (Point [] _) (Point [] _) = 0
distance p1 p2 = sum $ map (^2) $ zipWith (-) (coordenates p1) (coordenates p2)

-- Calcula a distancia euclidiana entre dois pontos
euclidianDistance :: Point -> Point -> Double
euclidianDistance p1 p2 = sqrt $ distance p1 p2

-- Verifica se a distancia euclidiana entre dois pontos atende a uma limite
testDistance :: Double -> Point -> Point -> Bool
testDistance limit p1 p2 = euclidianDistance p1 p2 <= limit

-- A funcao consiste em separar um conjunto de pontos em N grupos, sendo que 
-- os pontos pertencentes a um mesmo grupo possuem uma distancia euclidiana
-- menor ou igual a um limite pre-determinado.
clustering :: [Point] -> Double -> [[Point]]
clustering [] _ = []
clustering (x:xs) limit = [[x] ++ fst(test)] ++  clustering (snd(test)) limit
    where
    test = partition (testDistance limit x) xs 

-- O centroide eh o ponto representativo de um grupo e eh calculado 
-- como o centro de massa do grupo.
centroid :: [Point] -> Point
centroid group = Point {coordenates = coord, index = 0}
    where
    coord = map (/fromIntegral(size)) $ map sum $ transpose aux
    size = length group
    aux = map coordenates group

-- Calcula a soma das distancias euclidianas quadradas (SSE) entre os pontos 
-- pertencentes a cada um dos grupos. Para tanto, eh utilizado o centroide de cada grupo.
sse :: [[Point]] -> Double
sse groups = sum [ distance x (snd(group)) | group <- aux, x <- fst(group)] 
    where
    aux  = [ (group,centroid group) | group <- groups]

-- Gera uma string que representa o grupo 
groupToString :: [Point] -> String
groupToString group = unwords $ map show $ map index group
 
-- Gera uma string que representa cada um dos grupos
groupsToString :: [[Point]] -> String
groupsToString groups = intercalate "\n\n" (map groupToString groups)

main = do 
    contents <- readFile "entrada.txt"
    distance <- readFile "distancia.txt"
    let coords = getCoordenates contents
    let points = createPoints coords 1
    let groups = clustering points (readDouble distance)
    writeFile "saida.txt" $ printf "%s" (groupsToString groups)
    writeFile "result.txt" $ printf "%.4f" (sse groups)