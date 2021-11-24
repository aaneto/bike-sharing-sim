#![doc(html_logo_url = "https://www.onlygfx.com/wp-content/uploads/2015/12/bicycle-1024x602.png")]
//! - Initial demand is generated based on a demand level (simulator input)
//! - At each EPOCH, generate Users on each region based on a poisson distribution multiplied by a weight (DEMAND)

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
