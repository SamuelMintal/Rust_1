mod solution;
use solution::Simulation;

fn main() {

    let mut simulation = Simulation::new();

    let prg = simulation.new_city("Prague");
    let brn = simulation.new_city("Brno");
    let pls = simulation.new_city("Pilsen");
    let _d1 = simulation.new_road(&prg, &brn, 100);
    let _d2 = simulation.new_road(&brn, &pls, 100);
    let _d3 = simulation.new_road(&pls, &prg, 100);


    simulation.add_people(&prg, &brn, 50);
    simulation.add_people(&brn, &pls, 100);    
    simulation.new_bus(&[&prg, &brn, &pls, &prg, &brn, &pls]);
    

    
    for event in simulation.execute(101) {
        let name = event.city().name();
        let people_got_off = event.got_off();
        let people_got_on = event.got_on();  
        
        println!("name: {}", name);
        println!("people_got_off: {}", people_got_off);
        println!("people_got_on: {}", people_got_on);
        println!("---------------------------")
    }

    for _ in 0..10 {
        println!("XXXXXXXXXXXXXXXXXXXXXXXXXXXX");
        for event in simulation.execute(100) {
            let name = event.city().name();
            let people_got_off = event.got_off();
            let people_got_on = event.got_on();  
            
            println!("name: {}", name);
            println!("people_got_off: {}", people_got_off);
            println!("people_got_on: {}", people_got_on);
            println!("---------------------------")
        }
    }
}