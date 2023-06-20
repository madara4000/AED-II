"""
TRABALHO EM PYTHON DE GRAFOS 
ALUNO:Yago Martins Pintos
Matricula:20100404

ideia fazer implementação em rust,c++,c,java e js no futuro
"""
class Grafo:
    def adicionar_vertice(self, vertice):# função para verifica se ja existe se sim ele vai pro proximo if
        if not hasattr(self, 'vertices'):
            self.vertices = {}
            self.max_vertices = 20

        if len(self.vertices) < self.max_vertices:
            if vertice >= 0:
                self.vertices[vertice] = {}
        else:
            print("Limite máximo de vértices atingido.")

    def adicionar_aresta(self, origem, destino, peso):
        if hasattr(self, 'vertices') and origem in self.vertices and destino in self.vertices:
             self.vertices[origem][destino] = peso

    def mostrar_grafo(self):
        if hasattr(self, 'vertices'):
            for vertice in self.vertices:
                arestas = self.vertices[vertice]
                if arestas:
                    print(f"Vértice {vertice}:")
                    for destino, peso in arestas.items():
                        print(f"  Aresta para o vértice {destino} com peso {peso}")
                else:
                    print(f"Vértice {vertice} não possui arestas.")
                print()
        else:
            print("O grafo está vazio.")
            # Criar o grafo
# Criar o grafo
grafo = Grafo()

# Adicionar vértices
for i in range(6):
    grafo.adicionar_vertice(i)

# Adicionar arestas
grafo.adicionar_aresta(0, 1, 2)
grafo.adicionar_aresta(0, 2, 5)
grafo.adicionar_aresta(1, 3, 3)
grafo.adicionar_aresta(2, 3, 1)
grafo.adicionar_aresta(2, 4, 4)
grafo.adicionar_aresta(2, 6, 5)

# Mostrar os dados armazenados
grafo.mostrar_grafo()
