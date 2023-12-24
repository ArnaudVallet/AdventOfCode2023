use gcd::Gcd;

use crate::types::node::Node;
use std::collections::HashMap;
use gcd;

#[derive(Debug)]
pub struct Network {
    pub instructions: Vec<char>,
    pub nodes: HashMap<String, Node>
}

impl Network {
    pub fn add_node(&mut self, input: String) {
        let node_value = input.split('=').next().unwrap().trim();
        let start = input.find('(').unwrap();
        let end = input.find(')').unwrap();
        let node_l_r_str = &input[start+1..end];
        let node_l_r: Vec<&str> = node_l_r_str.split(", ").collect();
        let node = Node {
            value: node_value.to_string(),
            left: node_l_r[0].to_string(),
            right: node_l_r[1].to_string(),
        };
        self.nodes.insert(node_value.to_string(), node);
    }

    pub fn steps(&self, from: String, to: String) -> u128 {
        let mut steps: u128 = 0;
        let mut instruction_index = 0 as usize;
        let mut position = from;
        while position != to {
            if instruction_index >= self.instructions.len() {
                instruction_index = 0
            }
            position = self.destination(position, instruction_index);
            instruction_index += 1;
            steps += 1;
        }
        steps
    }

    pub fn ghost_steps(&self) -> u128 {
        // Find all Nodes ending with "A"
        let nodes: Vec<Node> = self.nodes.iter().filter_map(|(_key, node)| {
            if node.value.ends_with("A") {
                Some(node.clone())
            } else {
                None
            }
        }).collect();
        println!("Nodes: {:#?}", nodes);

        // Solution
        let mut cycles: Vec<Vec<u128>> = vec![];
        
        for node in nodes {
            let mut cycle = vec![];
            let mut current_node = node.clone();

            let mut current_instructions = self.instructions.clone();
            let mut step_count: u128 = 0;
            let mut first_z  = String::new();

            loop {
                while step_count == 0 || !current_node.value.ends_with("Z") {
                    //println!("current_instruction: {} step_count: {}", current_instructions[0], step_count);
                    step_count += 1;
                    if current_instructions[0] == 'L' {
                        let instruction = current_node.left;
                        current_node = self.nodes.get(&instruction).unwrap().clone();
                    } else {
                        let instruction = current_node.right;
                        current_node = self.nodes.get(&instruction).unwrap().clone();
                    } 
                    let mut new_instructions = current_instructions[1..].to_vec();
                    new_instructions.push(current_instructions[0]);
                    current_instructions = new_instructions;
                    //println!("Node: {:?} current_node: {:?}", node, current_node);
                }

                cycle.push(step_count);
                println!("Cycle: {:?}", cycle);

                if first_z == "" {
                    first_z = current_node.clone().value;
                    step_count = 0;
                } else if current_node.value == first_z {
                    break;
                }    
            }
            cycles.push(cycle);
            println!("Cycles: {:?}", cycles);
        }

        let mut nums: Vec<u128> = cycles.iter().map(|cycle| cycle[0]).collect();
        println!("nums: {:?}", nums);


        let mut lcm = nums.pop().unwrap();
        for num in nums {
            lcm = lcm * num / lcm.gcd(num);
        }

        println!("LCM: {}", lcm);





        // let mut steps: u128 = 0;
        // let mut instruction_index = 0 as usize;
        // let mut positions: Vec<String> = nodes.iter().map(|&ref node| node.clone().value).collect();
        // while !self.ghost_arrived(&positions) {
        //     if instruction_index >= self.instructions.len() {
        //         instruction_index = 0
        //     } 
        //     positions = positions
        //         .iter()
        //         .map(|p| self.destination(p.to_string(), instruction_index))
        //         .collect();
        //     instruction_index += 1;
        //     steps += 1;
        // }
        1
    }

    fn ghost_arrived(&self, positions: &Vec<String>) -> bool {
        if positions.iter().any(|position| position.chars().last().unwrap() == 'Z') {
            println!("Some ending by 'Z': {:?}", positions);
        }
        positions.iter().all(|position| position.chars().last().unwrap() == 'Z')
    }

    fn destination(&self, position: String, instruction_index: usize) -> String {
        let instruction = self.instructions.get(instruction_index).unwrap();
        let search_node = self.nodes.get(&position).unwrap();
        if instruction == &'L' {
            search_node.left.to_string()
        } else {
            search_node.right.to_string()
        }  
    }
}