run:
	lua main.lua
	diff saida.txt baterias/bateria6/saida.txt
	diff result.txt baterias/bateria6/result.txt