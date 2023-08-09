
use std::io::{self, Write};

const MAX_VERTICES: usize = 20;
const MAX_NOME_LEN: usize = 10;
const INFINITY: f32 = std::f32::INFINITY;

static mut VERTICES: [[f32; MAX_VERTICES]; MAX_VERTICES] = [[0.0; MAX_VERTICES]; MAX_VERTICES];
static mut NOMES: [[char; MAX_NOME_LEN]; MAX_VERTICES] = [['\0'; MAX_NOME_LEN]; MAX_VERTICES];

fn main() {
    clear_console();

    unsafe {
        zera_matriz();
        pede_vertices();
        pede_arestas();
        dijkstra_algorithm(0); 
        imprime_matriz();
    }
}


unsafe fn dijkstra_algorithm(source: usize) {
    let mut distance = [INFINITY; MAX_VERTICES];
    let mut visited = [false; MAX_VERTICES];

    distance[source] = 0.0;

    for _ in 0..MAX_VERTICES {
        let u = min_distance(&distance, &visited);
        visited[u] = true;

        for v in 0..MAX_VERTICES {
            if !visited[v] && VERTICES[u][v] > 0.0 {
                let new_distance = distance[u] + VERTICES[u][v];
                if new_distance < distance[v] {
                    distance[v] = new_distance;
                }
            }
        }
    }

    println!("\nShortest distances from source vertex ({} - {}):", source + 1, NOMES[source].iter().collect::<String>());
    for i in 0..MAX_VERTICES {
        if NOMES[i][0] != '\0' {
            println!(
                "To {} ({} - {}): {:.2}",
                i + 1,
                NOMES[i].iter().collect::<String>(),
                NOMES[source].iter().collect::<String>(),
                distance[i]
            );
        }
    }
}

unsafe fn min_distance(distance: &[f32; MAX_VERTICES], visited: &[bool; MAX_VERTICES]) -> usize {
    let mut min_distance = INFINITY;
    let mut min_vertex = 0;

    for v in 0..MAX_VERTICES {
        if !visited[v] && distance[v] < min_distance {
            min_distance = distance[v];
            min_vertex = v;
        }
    }

    min_vertex
}
fn clear_console() {
    #[cfg(target_os = "windows")]
    {
        let _ = std::process::Command::new("cmd").arg("/c").arg("cls").status();
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = std::process::Command::new("clear").status();
    }
}

unsafe fn zera_matriz() {
    for i in 0..MAX_VERTICES {
        for j in 0..MAX_VERTICES {
            VERTICES[i][j] = 0.0;
        }
    }
}

unsafe fn pede_vertices() {
    let mut quant = 0;
    let mut teste = 'S';

    println!("\n\nVamos criar uma matriz de um grafo de até 20 vértices.");
    while quant < MAX_VERTICES && (teste == 'S' || teste == 's') {
        println!("\n\nMatriz atual:\n\n");
        imprime_matriz();
        print!("\n\n\t\tDeseja informar um Vértice?(S/N): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        teste = input.trim().chars().next().unwrap_or('\0').to_ascii_uppercase();

        if teste == 'S' {
            println!("Por favor, informe o nome do vértice com até 10 letras: ");
            let mut nome_vertice = String::new();
            io::stdin().read_line(&mut nome_vertice).unwrap();
            let nome_vertice = nome_vertice.trim();

            for (i, c) in nome_vertice.chars().enumerate() {
                if i >= MAX_NOME_LEN {
                    break;
                }
                NOMES[quant][i] = c;
            }

            quant += 1;
        }
    }
}

unsafe fn pede_arestas() {
    let mut i = 0;
    while NOMES[i][0] != '\0' {
        let mut vert = 0;
        while vert >= 0 && vert < MAX_VERTICES {
            println!("\n\nMatriz atual:\n\n");
            imprime_matriz();
            print!(
                "Para o {}º Vértice: {} A quais vértices ele se conecta:(21 indica que não se conecta a mais nenhum.)\n\n",
                i + 1,
                NOMES[i].iter().collect::<String>()
            );

            for (i, nome) in NOMES.iter().enumerate() {
                if nome[0] != '\0' {
                    println!("{}º:  {}", i + 1, nome.iter().collect::<String>());
                }
            }

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            vert = input.trim().parse::<usize>().unwrap() - 1;

            if vert >= 0 && vert < MAX_VERTICES as usize {
                println!(
                    "Por favor, informe o valor da aresta entre os vértices: {} e {}: ",
                    NOMES[i].iter().collect::<String>(),
                    NOMES[vert as usize].iter().collect::<String>()
                );

                let mut valor_aresta = String::new();
                io::stdin().read_line(&mut valor_aresta).unwrap();
                let valor_aresta = valor_aresta.trim().parse::<f32>().unwrap();

                VERTICES[i][vert as usize] = valor_aresta;
                VERTICES[vert as usize][i] = valor_aresta;
            }
        }
        i += 1;
    }
}

unsafe fn imprime_matriz() {
    let mut qtd = 0;
    for i in 0..MAX_VERTICES {
        if NOMES[i][0] != '\0' {
            for &c in &NOMES[i] {
                if c == '\0' {
                    break;
                }
                print!("{}", c);
            }
            qtd += 1;
        }
        print!("\t");
    }

    for i in 0..qtd {
        println!();
        for &c in &NOMES[i] {
            if c == '\0' {
                break;
            }
            print!("{}", c);
        }
        for j in 0..qtd {
            if VERTICES[i][j] != 0.0 {
                print!("\t{:.2}", VERTICES[i][j]);
            } else {
                print!("\t-");
            }
        }
    }
    println!("\n\n");
}