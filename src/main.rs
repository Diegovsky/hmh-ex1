use std::{
    collections::{BTreeSet as Set, HashMap},
    io::Read,
    str::FromStr,
};

/// Nós são identificados pelo tipo `u32`, que é um inteiro de 32 bits positivo.
///
/// Equivale a um typedef em C++.
pub type Node = u32;
/// Definimos pesos das arestas como sendo inteiros de 32bits positivos.
pub type Weight = u32;
/// Definimos nossas arestas como sendo uma tupla de dois nós e um peso.
pub type Edge = (Node, Node, Weight);

/// Um `trait` que define os métodos que todo grafo deve implementar.
///
/// `Trait`s são análogos a classes abstratas em C++, ou interfaces em outras linguagens
trait Graph {
    fn add_node(&mut self) -> Node;
    fn add_edge(&mut self, a: Node, b: Node, weight: Weight);
    fn edges(&self) -> Set<Edge>;
}

/// Struct que representa um grafo implementado por meio de lista de adjacência.
///
/// A diretiva `derive` implementa traits (interfaces) automaticamente, sendo eles:
///     - Default: Permite a inicialização com valores padrões para todos os campos
///     - Debug: Mostra o tipo e seus campos de forma intuitiva para debug
///     - Clone: Permite criar cópias da struct.
#[derive(Default, Debug, Clone)]
struct GraphAdj {
    next_node: Node,
    nodes: Set<Node>,
    node_edges: Set<Edge>,
}

impl Graph for GraphAdj {
    fn edges(&self) -> Set<Edge> {
        self.node_edges.clone()
    }
    fn add_node(&mut self) -> Node {
        let node = self.next_node;
        self.nodes.insert(node);

        self.next_node += 1;
        node
    }
    fn add_edge(&mut self, a: Node, b: Node, weight: Weight) {
        self.node_edges.insert((a, b, weight));
    }
}

/// Struct que representa um grafo implementado por matriz de adjacência.
#[derive(Default, Debug, Clone)]
struct GraphMat {
    node_count: usize,
    links: Vec<Weight>,
}

impl Graph for GraphMat {
    fn add_node(&mut self) -> Node {
        let new_node = self.node_count as Node;

        let new_node_count = self.node_count + 1;
        // Cria novo vetor cujo tamanho é `(node_count+1) ^ 2`
        let mut new_links = vec![0; new_node_count.pow(2)];

        // Caso hajam nós no vetor, precisamos copiar as informações para o novo.
        if self.node_count > 0 {
            // Cria um iterador que agrupa `new_node_count` elementos por vez do novo vetor.
            // Ou seja, temos um vetor que representa uma linha do vetor a cada iteração.
            let new_lines = new_links.chunks_mut(new_node_count);
            // Cria um iterador que agroupa `node_count` elementos por vez do vetor antigo.
            let old_lines = self.links.chunks_mut(self.node_count);
            for (new_line, old_line) in new_lines.zip(old_lines) {
                // Limita as linhas do novo vetor para que tenham exatamente `node_count` elementos
                new_line[..self.node_count]
                    // Copia os pesos das arestas do antigo vetor para o novo.
                    .copy_from_slice(old_line);
            }
        }

        self.links = new_links;
        self.node_count += 1;

        new_node
    }
    fn edges(&self) -> Set<Edge> {
        self.links
            .iter()
            // Iteramos sobre cópias em vez de referências
            .copied()
            // Adicionamos um contador à cada elemento
            .enumerate()
            // Filtra links cujo peso é 0
            .filter(|(_, weight)| *weight > 0)
            // Transforma uma tupla de posição e peso em `Edge`.
            .filter_map(|(i, weight)| {
                let y = i / self.node_count;
                let x = i % self.node_count;
                // Filtra pesos duplicados abaixo da horizontal
                if x < y {
                    None
                } else {
                    Some((y as Node, x as Node, weight))
                }
            })
            .collect()
    }
    fn add_edge(&mut self, a: Node, b: Node, weight: Weight) {
        // Converte nós em `usizes` para simplificar a indexação.
        let a = a as usize;
        let b = b as usize;
        // Registra a ligação para o nó `a`
        self.links[a * self.node_count + b] = weight;
        // Registra a ligação para o nó `b`
        self.links[b * self.node_count + a] = weight;
    }
}

fn fill_graph(input_data: &[Vec<u32>], graph: &mut dyn Graph) {
    // Separa o vetor entre o primeiro elemento e o resto.
    let (head, tail) = input_data.split_first().expect("Vetor veio vazio");
    // Tenta desestruturar o vetor `head` em dois valores, executando o `else`
    // caso não seja possível.
    let [vertex_count, edge_count] = head[..] else {
        panic!("Esperava que a primeira linha contivesse exatamente dois valores.");
    };

    // Cria `vertex_count` nós.
    for _ in 0..vertex_count {
        // Para simplificar essa parte, pressupõe-se que os nós retornados são criados em órdem
        // crescente com incremento de 1, sendo o primeiro nó `0`.
        graph.add_node();
    }

    // Converte `edge_count` para `usize` para indexação.
    //
    // `usize` é um inteiro positivo cujo tamanho é definido pela arquitetura,
    // comummente utilizado para indexação.
    let edge_count = edge_count as usize;

    // Adiciona `edge_count` arestas ao grafo
    for edge_data in &tail[..edge_count] {
        let [a, b, weight] = edge_data[..] else {
            panic!("Esperava que cada linha de aresta tivesse exatamente três valores.");
        };
        // Adiciona uma aresta entre o nó `a` e o nó `b`
        //
        // Como dito anteriormente, os nós são crescentes e começam em 0, portanto, precisamos
        // subtrair 1 dos identificadores das entradas.
        graph.add_edge(a - 1, b - 1, weight);
    }
}

/// Printa as arestas do grafo
fn print_edges(graph: &dyn Graph) {
    let edges = graph.edges();
    for edge in edges {
        // Como os nós começam em 0, somamos 1 para ficar igual à entrada.
        println!("{} {} {}", edge.0+1, edge.1+1, edge.2);
    }
}

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
