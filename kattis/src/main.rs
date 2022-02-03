use std::io;
use std::io::prelude::*;

fn main() {
    let input = io::stdin();

    let mut is_stack = true;
    let mut is_queue = true;
    let mut is_priority = true;

    let mut stack_bag: Vec<usize> = Vec::new();
    let mut queue_bag: Vec<usize> = Vec::new();
    let mut priority_bag: Vec<usize> = Vec::new();

    let mut counter: usize = 0;
    let mut length: usize = 0;


    for line in input.lock().lines().map(|_line| _line.unwrap()) {
        if counter == 0 {
            length = line.trim().parse().unwrap();
        }
        else {
            let com: Vec<usize> = line.split_whitespace().map(|i| i.trim().parse().unwrap()).collect();
            if com[0] == 1 {
                stack_bag.push(com[1]);
                queue_bag.push(com[1]);
                priority_bag.push(com[1]);
            }
            else {
                if queue_bag.len() == 0 || com[1] != queue_bag[0] {
                    is_queue = false;
                }
                
                else {
                    queue_bag.remove(0);
                }

                if stack_bag.len() == 0 || com[1] != *stack_bag.last().unwrap() {
                    is_stack = false;
                }
                else {
                    stack_bag.remove(stack_bag.len()-1);
                }

                if priority_bag.len() == 0 || com[1] != *priority_bag.iter().max().unwrap() {
                    is_priority = false;
                }
                else {
                    priority_bag.remove(priority_bag.iter().position(|i| *i == com[1]).unwrap());
                }
            }
        }

        counter += 1;

        if counter > length {
            if !is_priority && !is_queue && !is_stack {
                println!("impossible");
            }
            else if is_priority && !is_queue && !is_stack {
                println!("priority queue");
            }
            else if !is_priority && is_queue && !is_stack {
                println!("queue");
            }
            else if !is_priority && !is_queue && is_stack {
                println!("stack");
            }
            else {
                println!("not sure");
            }
            is_priority = true;
            is_queue = true;
            is_stack = true;
            stack_bag = vec![];
            queue_bag = vec![];
            priority_bag = vec![];
            counter = 0;
        }
    }

}
