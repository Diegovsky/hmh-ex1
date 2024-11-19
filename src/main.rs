use std::str::FromStr;
use ex1::{GraphAdj, GraphMat, fill_graph, print_edges};

fn main() {
    // Obtém o nome do arquivo a partir do argv[1].
    let filename = std::env::args()
        .nth(1)
        .expect("Esperava o nome do arquivo de entrada");

    // Lê o arquivo inteiro e o armazena na memória.
    let input_data = std::fs::read_to_string(filename).expect("Falha ao ler arquivo de entrada");
    let input_data: Vec<Vec<u32>> = input_data
        // Separa a string por fim de linha
        .split('\n')
        .map(|line|
            // Separa cada linha por espaços
            line.split_whitespace()
                // Tenta converter a string em u32
                .map(|num| u32::from_str(num).expect("Número inválido"))
                // Coleta os resultados em um Vec
                .collect::<Vec<u32>>())
        .collect();

    let mut graph_adj = GraphAdj::default();
    let mut graph_mat = GraphMat::default();
    fill_graph(&input_data, &mut graph_adj);
    fill_graph(&input_data, &mut graph_mat);

    println!("Arestas do grafo por matriz de adj:");
    print_edges(&graph_mat);
    println!("Arestas do grafo por lista de adj:");
    print_edges(&graph_adj);
}
