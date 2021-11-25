#![doc(html_logo_url = "https://www.onlygfx.com/wp-content/uploads/2015/12/bicycle-1024x602.png")]
//! Este projeto se trata de um projeto de simulação de um serviço conhecido como Bike-Sharing.
//! Onde existem vários pontos de distribuições com bicicletas prontas para serem tomadas por
//! usuários do serviço, assim que os usuários fizerem a viagem que precisam com a bicicleta,
//! eles a devolvem em algum ponto de distribuição que não necessariamente é onde ele buscou
//! a bicicleta no primeiro momento.
//!
//!
//! A simulação se estrutura da seguinte forma:
//!
//! ## Inicialização:
//! - Uma topologia (fixa) é gerada em formato de grafo, onde cada ponto representa um ponto de
//! distribuição.
//! - Os pesos são gerados para cada ponto do grafo, onde o peso simboliza a quantidade de demanda
//! que existe por bicicletas naquele ponto.
//! - A quantidade de bicicletas nos pontos também é gerada aleatóriamente.
//! - [TODO] A quantidade e posições iniciais dos caminhões são geradas aleatóriamentes.
//!
//! Note que estamos falando de uma inicialização. A topologia, demanda e carga são entradas do sistema
//! podendo em tese estar em uma interface gráfica, eles são gerados aleatoriamente neste caso apenas
//! por fins exploratórios do sistema.
//!
//! Numa aplicação real, a topologia é fixa e baseada no mundo fisico, as demandas também são relativamente
//! fixas em amplitude mas variando ao longo do dia. As quantidades de bicicletas são fixas dependendo do estoque
//! da empresa e são distribuidas seguindo a demanda, isso também se aplica aos caminhões de reabastecimento.
//!
//! Após essa etapa inicial de geração, dividimos a execução em "epochs",
//! cada epoch é uma execução de um for loop.
//!
//! Dentro desse loop:
//! 1. Bicicletas em cada nó = Bicicletas em cada nó - Demanda ou 0 se a demanda superar o estoque
//! 2. Os clientes que não conseguiram um bike saem do sistema, ou seja, não foram atendidos.
//! 3. Os clientes escolhem um destino aleatório de forma uniforme e reabastecem esses pontos com suas
//! bicicletas.
//!
//! ## Objetivo da Simulação:
//!
//! Queremos avaliar a performance de heuristicas para reposição de bicicletas, dessa forma queremos responder
//! perguntas como:
//! - Qual o impacto da interrupção do serviço de distribuição?
//! - Qual o melhor algoritmo de distribuição?
//! - Qual o desempenho que os caminhões precisam ter para alcançarmos a funcionalidade desejada?

mod metrics;
mod graph;
mod random;
mod topology_01;

const NUM_ITERATIONS: u32 = 500;

#[derive(serde::Serialize)]
pub struct Metric {
    rider_exits: i32,
    complete_travels: i32
}

fn main() {
    let mut metrics = Vec::new();
    let mut demands = topology_01::generate_demands();
    let mut loads = topology_01::generate_station_loads();
    let num_nodes = demands.len();


    for _ in 0..NUM_ITERATIONS {
        let mut rider_exits = 0;
        let mut complete_travels = 0;

        for node_number in 0..num_nodes {
            // Riders attempt to remove bikes, if there are enough bikes the stations is left
            // with bikes and the riders are satisfied, else, some riders exit the system and
            // the stations are left empty.
            let mut bikers_travelling = 0;

            if loads[&node_number] >= demands[&node_number] {
                bikers_travelling += demands[&node_number];
                loads.insert(node_number, loads[&node_number] - demands[&node_number]);
                demands.insert(node_number, 0);
            } else {
                bikers_travelling += loads[&node_number];
                let bikers_leaving_the_system = demands[&node_number] - loads[&node_number];
                rider_exits += bikers_leaving_the_system;

                demands.insert(node_number, 0);
                loads.insert(node_number, 0);
            }

            // Riders destination is uniformly distributed
            for _ in 0..bikers_travelling {
                let destination = random::uniform_integer_0_end(num_nodes as u32) as usize;
                loads.insert(destination, 1 + loads[&destination]);
            }

            complete_travels += bikers_travelling;
        }
        topology_01::renew_demands(&mut demands);
        metrics.push(Metric { rider_exits, complete_travels });
    }
    metrics::write_to_file(metrics);
}
