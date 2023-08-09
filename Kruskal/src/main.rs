use std::io::{self, Write};
use std::cmp::Ordering;

const MAX_VERTIX: usize = 20;
const MAX_NAME_LEN: usize = 10;

static mut VERTIX: [[f32; MAX_VERTIX]; MAX_VERTIX] = [[0.0; MAX_VERTIX]; MAX_VERTIX];
static mut NAMES: [[char; MAX_NAME_LEN]; MAX_VERTIX] = [['\0'; MAX_NAME_LEN]; MAX_VERTIX];
static mut SELECTING: [bool; MAX_VERTIX] = [false; MAX_VERTIX];
static mut DIST: [f32; MAX_VERTIX] = [f32::MAX; MAX_VERTIX];
static mut PARENT: [usize; MAX_VERTIX] = [0; MAX_VERTIX];

fn main() {
    clear_console();

    unsafe {
        reset_matrix();
        pede_vertix();
        pede_edges();
        print_matrix();
        let mst = kruskal();
        println!("Minimum Spanning Tree (MST) using  Kruskal Algorithm:");
        print_mst(&mst);
        }
        }
        
        unsafe fn print_mst(mst: &Vec<(usize, usize, f32)>) {
            for &(u, v, weight) in mst {
                let name_u: String = NAMES[u].iter().collect();
                let name_v: String = NAMES[v].iter().collect();
                println!("Edge: {} - {} | Weight: {:.2}", name_u, name_v, weight);
            }
        }
        
        
        unsafe fn kruskal() -> Vec<(usize, usize, f32)> {
        let mut edges: Vec<(usize, usize, f32)> = Vec::new();
        let mut mst: Vec<(usize, usize, f32)> = Vec::new();
        
        // Coleta todas as arestas do grafo
        for i in 0..MAX_VERTIX{
        for j in 0..MAX_VERTIX {
            if VERTIX[i][j] != 0.0 {
                edges.push((i, j, VERTIX[i][j]));
            }
        }
        }
        
        // Ordena as arestas em ordem crescente de peso
        edges.sort_by(|(_, _, w1), (_, _, w2)| w1.partial_cmp(w2).unwrap_or(Ordering::Equal));
        
        // Inicializa uma estrutura de conjunto disjunto (disjoint-set)
        let mut parent: Vec<usize> = (0..MAX_VERTIX).collect();
        
        // Função auxiliar para encontrar o pai de um vértice no conjunto disjunto
        fn find(parent: &mut Vec<usize>, v: usize) -> usize {
        if parent[v] != v {
            parent[v] = find(parent, parent[v]);
        }
        parent[v]
        }
        
        // Função auxiliar para realizar a união de dois conjuntos no conjunto 
        fn union(parent: &mut Vec<usize>, rank: &mut Vec<usize>, x: usize, y: usize) {
        let xroot = find(parent, x);
        let yroot = find(parent, y);
        
        if rank[xroot] < rank[yroot] {
            parent[xroot] = yroot;
        } else if rank[xroot] > rank[yroot] {
            parent[yroot] = xroot;
        } else {
            parent[yroot] = xroot;
            rank[xroot] += 1;
        }
        }
        
        
        for edge in edges {
        let (u, v, weight) = edge;
        let uroot = find(&mut parent, u);
        let vroot = find(&mut parent, v);
        
        if uroot != vroot {
            union(&mut parent, &mut vec![0; MAX_VERTIX], uroot, vroot);
            mst.push((u, v, weight));
        }
        }
        
        mst
        }
        
        unsafe fn imprime_mst(mst: &Vec<(usize, usize, f32)>) {
        for &(u, v, weight) in mst {
        let name_u: String = NAMES[u].iter().collect();
        let name_v: String = NAMES[v].iter().collect();
        println!("Aresta: {} - {} | Peso: {:.2}", name_u, name_v, weight);
        }
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

unsafe fn reset_matrix() {
    for i in 0..MAX_VERTIX {
        for j in 0..MAX_VERTIX {
            VERTIX[i][j] = 0.0;
        }
    }
}

unsafe fn pede_vertix() {
    let mut quant = 0;
    let mut teste = 'S';

    println!("\n\nlet's create one  matrix for graph until 20 vértix.");
    while quant < MAX_VERTIX && (teste == 'S' || teste == 's') {
        println!("\n\n Current matrix :\n\n");
        print_matrix();
        println!("\n\n\t\tWant to inform um Vertix?(S/N): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        teste = input.trim().chars().next().unwrap_or('\0').to_ascii_uppercase();

        if teste == 'S' {
            println!("`Please, enter the name of the vertix until até 10 letters: ");
            let mut name_vertice = String::new();
            io::stdin().read_line(&mut name_vertice).unwrap();
            let name_vertice = name_vertice.trim();

            for (i, c) in name_vertice.chars().enumerate() {
                if i >= MAX_NAME_LEN {
                    break;
                }
                NAMES[quant][i] = c;
            }

            quant += 1;
        }
    }
}

unsafe fn pede_edges() {
    let mut i = 0;
    while NAMES[i][0] != '\0' {
        let mut vert = 0;
        while vert >= 0 && vert < MAX_VERTIX {
            println!("\n\n Current matrix :\n\n");
            print_matrix();
            println!(
                "For the {}th Vertex: {} Which VERTIX it connects to:(21 indicates it doesn't connect to any more.)\n\n",
                i + 1,
                NAMES[i].iter().collect::<String>()
            );

            for (i, name) in NAMES.iter().enumerate() {
                if name[0] != '\0' {
                    println!("{}º:  {}", i + 1, name.iter().collect::<String>());
                }
            }

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            vert = input.trim().parse::<usize>().unwrap() - 1;

            if vert >= 0 && vert < MAX_VERTIX as usize {
                println!(
                    "Please enter the value of the edge between the VERTIX: {} and {}: ",
                    NAMES[i].iter().collect::<String>(),
                    NAMES[vert as usize].iter().collect::<String>()
                );

                let mut value_edges = String::new();
                io::stdin().read_line(&mut value_edges).unwrap();
                let value_edges = value_edges.trim().parse::<f32>().unwrap();

                VERTIX[i][vert as usize] = value_edges;
                VERTIX[vert as usize][i] = value_edges;
            }
        }
        i += 1;
    }
}

unsafe fn print_matrix() {
    let mut qtd = 0;
    for i in 0..MAX_VERTIX {
        if NAMES[i][0] != '\0' {
            for &c in &NAMES[i] {
                if c == '\0' {
                    break;
                }
                println!("{}", c);
            }
            qtd += 1;
        }
        println!("\t");
    }

    for i in 0..qtd {
        println!();
        for &c in &NAMES[i] {
            if c == '\0' {
                break;
            }
            print!("{}", c);
        }
        for j in 0..qtd {
            if VERTIX[i][j] != 0.0 {
                print!("\t{:.2}", VERTIX[i][j]);
            } else {
                print!("\t-");
            }
        }
    }
    println!("\n\n");
}