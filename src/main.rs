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
//! Nesse caso a demanda é estática e gerada na inicialização.
//!
//! ## Objetivo da Simulação:
//!
//! Queremos avaliar a performance de heuristicas para reposição de bicicletas, dessa forma queremos responder
//! perguntas como:
//! - Qual o impacto da interrupção do serviço de distribuição?
//! - Qual o melhor algoritmo de distribuição?
//! - Qual o desempenho que os caminhões precisam ter para alcançarmos a funcionalidade desejada?
//!
//! ## Todo:
//! - Documentar etapas da geração com mais detalhes dentro das funções
//! - Metrificar resultados para verificar se simulação ainda faz sentido
mod metrics;
mod random;

#[derive(serde::Serialize)]
pub struct Metric {
    rider_exits: i32,
    complete_travels: i32,
}

/// Container para bikes que estão para chegar em um ponto de distribuição,
/// essa abstração é necessária porque os agentes de reposição levam unidades
/// de tempo predefinidos (dependendo do grafo) para chegar de um ponto até outro
struct IncomingBikes {
    /// Quando epochs_to_arrival é igual a zero, as bikes chegaram e este struct se desfaz.
    epochs_to_arrival: u64,
}

struct Vertex {
    bikes_available: u64,
    cyclists_poisson_lambda: f64,
    incoming_bikes: Vec<IncomingBikes>,
}

struct Topology {
    travel_time: u64,
    reposition_travel_time: u64,
    vertexes: Vec<Vertex>,
    /// Served customers by epoch
    served_customers: Vec<u64>,
    /// Rejected customers by epoch
    rejected_customers: Vec<u64>,
}

impl Topology {
    fn new(
        travel_time: u64,
        reposition_travel_time: u64,
        cyclists_arrival_means: Vec<f64>,
        bikes_available: Vec<u64>,
    ) -> Topology {
        if cyclists_arrival_means.len() != bikes_available.len() {
            panic!("Cyclists arrival means must have the same dimension as bikes available.");
        }

        Topology {
            vertexes: (0..bikes_available.len())
                .map(|i| Vertex {
                    bikes_available: bikes_available[i],
                    cyclists_poisson_lambda: cyclists_arrival_means[i],
                    incoming_bikes: vec![],
                })
                .collect(),
            served_customers: vec![],
            rejected_customers: vec![],
            travel_time,
            reposition_travel_time
        }
    }

    /// Os ciclistas são gerados em cada vértice seguindo uma distribuição poisson de média igual
    /// a demanda, se são gerados 20 ciclistas naquele vértice e existem 12 bikes, não sobram bikes
    /// e 8 saem do sistema. Caso existam 12 ciclista e 20 bikes, 8 bikes sobram e nenhum ciclista
    /// sai do sistema.
    fn distribute_bikes(&mut self) {
        let mut served_customers = 0;
        let mut rejected_customers = 0;

        for v in self.vertexes.iter_mut() {
            let generated_bikers = random::poisson_0_to_lambda(v.cyclists_poisson_lambda) as u64;
            if v.bikes_available > generated_bikers {
                served_customers += generated_bikers;
                v.bikes_available -= generated_bikers;
            } else {
                rejected_customers += generated_bikers - v.bikes_available;
                served_customers += v.bikes_available;
                v.bikes_available = 0;
            }
        }
        self.served_customers.push(served_customers);
        self.rejected_customers.push(rejected_customers);
        self.generate_incoming_bikes(served_customers);
    }

    /// Verifica bikes que estavam em transição entre pontos do grafo, caso elas tenham chegado ao destino
    /// incrementar esses locais com as bikes.
    fn check_incoming_bikes(&mut self) {
        for v in self.vertexes.iter_mut() {
            let mut vertex_arrivals = 0;
            for incoming_bike in v.incoming_bikes.iter_mut() {
                incoming_bike.epochs_to_arrival -= 1;

                if incoming_bike.epochs_to_arrival == 0 {
                    vertex_arrivals += 1;
                }
            }
            // Remove bikes that arrived.
            v.incoming_bikes.retain(|ib| ib.epochs_to_arrival > 0);
            v.bikes_available += vertex_arrivals;
        }
    }

    /// Esta função gera a estrutura IncomingBikes, responsável por controlar a movimentação de bikes
    /// no grafo, a estrutura é necessária uma vez que não podemos mover bikes de forma instantanea, existe
    /// um delay de instantes de tempo até a bike alcançar o destino.
    fn generate_incoming_bikes(&mut self, nof_customers: u64) {
        for _ in 0..nof_customers {
            let destination_idx =
                random::uniform_integer_0_end(self.vertexes.len() as u32) as usize;
            self.vertexes[destination_idx]
                .incoming_bikes
                .push(IncomingBikes {
                    epochs_to_arrival: self.travel_time,
                })
        }
    }

    fn build_metrics(&self) -> Vec<Metric> {
        let mut m = vec![];
        for i in 0..self.served_customers.len() {
            let rider_exits = self.rejected_customers[i] as i32;
            let complete_travels = self.served_customers[i] as i32;
            m.push(Metric {
                rider_exits: rider_exits,
                complete_travels: complete_travels,
            })
        }

        m
    }

    fn demand_based_reposition(&mut self, rep: usize, batch_size: u64) {
        for _ in 0..rep {
            let pg = self.find_vertex_with_positive_gap();
            let ng = self.find_vertex_with_negative_gap();

            match (pg, ng) {
                (Some(pgi), Some(ngi)) => {
                    let moved = if self.vertexes[ngi].bikes_available > batch_size {
                        batch_size
                    } else {
                        self.vertexes[ngi].bikes_available
                    };
                    self.vertexes[ngi].bikes_available -= moved;
                    for _ in 0..moved {
                        self.vertexes[pgi].incoming_bikes.push(IncomingBikes {
                            epochs_to_arrival: self.reposition_travel_time
                        });
                    }
                },
                _ => ()
            }
        }
    }

    fn find_vertex_with_positive_gap(&self) -> Option<usize> {
        for i in 0..self.vertexes.len() {
            let gap = self.vertexes[i].cyclists_poisson_lambda - self.vertexes[i].bikes_available as f64;
            if gap > 0.0 {
                return Some(i);
            }
        }

        return None;
    }

    fn find_vertex_with_negative_gap(&self) -> Option<usize> {
        for i in 0..self.vertexes.len() {
            let gap = self.vertexes[i].cyclists_poisson_lambda - self.vertexes[i].bikes_available as f64;
            if gap < 0.0 {
                return Some(i);
            }
        }

        return None;
    }
}

fn main() {
    let travel_time =3 ;
    let reposition_travel_time = 1;
    let mut topology = Topology::new(
        travel_time,
        reposition_travel_time,
        vec![
            4.0, 12.0, 13.0, 3.0, 4.0, 5.0, 3.0, 4.0, 3.0, 3.0, 4.0, 5.0, 3.0, 9.0, 7.0, 3.0, 4.0,
            8.0, 2.0, 3.0,
        ],
        (0..20).map(|_| 9).collect(),
    );

    for _ in 0..150 {
        topology.check_incoming_bikes();
        topology.distribute_bikes();
        topology.demand_based_reposition(4, 1);
    }
    let metrics = topology.build_metrics();
    metrics::write_to_file(metrics, "metrics.json");
}
