/*
 * EpiRust
 * Copyright (c) 2020  ThoughtWorks, Inc.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 */

use crate::listeners::listener::Listener;
use std::any::Any;
use crate::disease_state_machine::State;
use crate::environment;
use crate::travel_plan::TravellersByRegion;

#[derive(Serialize, Debug, PartialEq)]
struct CountsByRegion {
    hr: i32,
    destination: String,
    susceptible: i32,
    exposed: i32,
    infected: i32,
    recovered: i32,
}

impl CountsByRegion {
    fn create_from(hr: i32, travellers_by_region: &TravellersByRegion) -> CountsByRegion {
        let mut susceptible = 0;
        let mut exposed = 0;
        let mut infected = 0;
        let mut recovered = 0;
        travellers_by_region.get_travellers_slice().iter().for_each(|traveller| {
            match traveller.state_machine.state {
                State::Susceptible { .. } => { susceptible += 1 }
                State::Exposed { .. } => { exposed += 1 }
                State::Infected { .. } => { infected += 1 }
                State::Recovered { .. } => { recovered += 1 }
                State::Deceased { .. } => { panic!("Deceased citizen should never be travelling!") }
            }
        });
        CountsByRegion {
            hr,
            destination: travellers_by_region.to_engine_id().clone(),
            susceptible,
            exposed,
            infected,
            recovered,
        }
    }

    #[cfg(test)]
    fn new(hr: i32, destination: String, s: i32, e: i32, i: i32, r: i32) -> CountsByRegion {
        CountsByRegion {
            hr,
            destination,
            susceptible: s,
            exposed: e,
            infected: i,
            recovered: r,
        }
    }
}

pub struct TravelCounter {
    counts: Vec<CountsByRegion>,
    output_file_name: String,
}

impl TravelCounter {
    pub fn new(output_file_name: String) -> TravelCounter {
        TravelCounter {
            counts: Vec::new(),
            output_file_name,
        }
    }
}

impl Listener for TravelCounter {
    fn simulation_ended(&mut self) {
        let mut output_path = environment::output_dir();
        output_path.push(&self.output_file_name);
        match crate::listeners::csv_service::write(&output_path, &self.counts) {
            Ok(_) => {}
            Err(e) => { error!("Failed to serialize outgoing travels: {}", e) }
        }
    }

    fn outgoing_travellers_added(&mut self, hr: i32, travellers: &Vec<TravellersByRegion>) {
        let counts_by_region: Vec<CountsByRegion> = travellers.iter().map(|t| {
            CountsByRegion::create_from(hr, t)
        }).collect();
        self.counts.extend(counts_by_region);
    }


    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::disease_state_machine::InfectionSeverity;
    use std::env;
    use crate::travel_plan::Traveller;

    #[test]
    fn should_add_outgoing_travellers() {
        //dump the output file to a temp location
        env::set_var(environment::EPI_OUTPUT_DIR, env::temp_dir().to_str().unwrap());
        let mut counter = TravelCounter::new("test_travel.csv".to_string());

        let travellers_by_region = vec![create_travellers("engine2"), create_travellers("engine3")];
        counter.outgoing_travellers_added(24, &travellers_by_region);
        counter.outgoing_travellers_added(48, &vec![create_travellers("engine2")]);
        counter.simulation_ended();

        assert_eq!(3, counter.counts.len());

        let row1 = CountsByRegion::new(24, "engine2".to_string(), 2, 2, 2, 2);
        let row2 = CountsByRegion::new(24, "engine3".to_string(), 2, 2, 2, 2);
        let row3 = CountsByRegion::new(48, "engine2".to_string(), 2, 2, 2, 2);

        assert_eq!(row1, *counter.counts.get(0).unwrap());
        assert_eq!(row2, *counter.counts.get(1).unwrap());
        assert_eq!(row3, *counter.counts.get(2).unwrap());

        //clear the env variable
        env::remove_var(environment::EPI_OUTPUT_DIR);
    }

    fn create_travellers(region: &str) -> TravellersByRegion {
        let mut travellers = TravellersByRegion::create(&region.to_string());
        for _i in 0..2 {
            let mut s = Traveller::new();
            s.state_machine.state = State::Susceptible {};
            travellers.alloc_citizen(s);

            let mut e = Traveller::new();
            e.state_machine.state = State::Exposed { at_hour: 10 };
            travellers.alloc_citizen(e);

            let mut i = Traveller::new();
            i.state_machine.state = State::Infected { symptoms: true, severity: InfectionSeverity::Mild {} };
            travellers.alloc_citizen(i);

            let mut r = Traveller::new();
            r.state_machine.state = State::Recovered {};
            travellers.alloc_citizen(r);
        }

        travellers
    }
}
