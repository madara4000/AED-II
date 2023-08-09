
/*use std::io::{self, Write};

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
        Prim(); 
        print_matrix();
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

unsafe fn Prim() {
    let mut start_vertex = 0;
    println!("\n\n  Enter the index(starting in 1) of vertix initial for Prim:  ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    start_vertex = input.trim().parse::<usize>().unwrap() - 1;

    SELECTING[start_vertex] = true;
    DIST[start_vertex] = 0.0;

    let mut num_selected = 1;
    let mut current_vertex = start_vertex;

    while num_selected < MAX_VERTIX {
        let mut nearest_vertex = 0;
        let mut min_distance = f32::MAX;

        for i in 0..MAX_VERTIX {
            if !SELECTING[i] && VERTIX[current_vertex][i] != 0.0 && VERTIX[current_vertex][i] < DIST[i] {
                DIST[i] = VERTIX[current_vertex][i];
                PARENT[i] = current_vertex;
            }

            if !SELECTING[i] && DIST[i] < min_distance {
                min_distance = DIST[i];
                nearest_vertex = i;
            }
        }

        SELECTING[nearest_vertex] = true;
        current_vertex = nearest_vertex;
        num_selected += 1;
    }
}
*/
use std::io::{self, Write};

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
        let mst = prim();
        println!("Minimum Spanning Tree (MST) using Prim's algorithm:");
        print_mst(&mst);

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

        
unsafe fn prim() -> Vec<(usize, usize, f32)> {
    let mut mst: Vec<(usize, usize, f32)> = Vec::new();
    let mut in_mst: Vec<bool> = vec![false; MAX_VERTIX];
    let mut min_distance: Vec<f32> = vec![f32::MAX; MAX_VERTIX];
    let mut parent: Vec<usize> = vec![0; MAX_VERTIX];

    // Start the MST from the first vertex
    min_distance[0] = 0.0;

    for _ in 0..MAX_VERTIX {
        let u = find_min_distance(&min_distance, &in_mst);
        in_mst[u] = true;

        for v in 0..MAX_VERTIX {
            let weight = VERTIX[u][v];
            if weight > 0.0 && !in_mst[v] && weight < min_distance[v] {
                parent[v] = u;
                min_distance[v] = weight;
            }
        }
    }

    for v in 1..MAX_VERTIX {
        mst.push((parent[v], v, VERTIX[parent[v]][v]));
    }

    mst
}

fn find_min_distance(min_distance: &Vec<f32>, in_mst: &Vec<bool>) -> usize {
    let mut min_val = f32::MAX;
    let mut min_index = 0;

    for v in 0..MAX_VERTIX {
        if !in_mst[v] && min_distance[v] < min_val {
            min_val = min_distance[v];
            min_index = v;
        }
    }

    min_index
}

unsafe fn print_mst(mst: &Vec<(usize, usize, f32)>) {
    for &(u, v, weight) in mst {
        let name_u: String = NAMES[u].iter().collect();
        let name_v: String = NAMES[v].iter().collect();
        println!("Edge: {} - {} | Weight: {:.2}", name_u, name_v, weight);
    }
}