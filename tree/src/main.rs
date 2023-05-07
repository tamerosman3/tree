use indextree::{Arena, NodeId};
use std::{collections::HashMap, thread, time};

use sysinfo::{ProcessExt, System, SystemExt};

fn main() {
    loop {
        let mut system = System::new_all();
        system.refresh_all();
        let process_map = system.get_processes();
        let mut arena = Arena::new();
        let mut node_map: HashMap<i32, NodeId> = HashMap::new();

        for (pid, process) in process_map {
            let parent_pid = process.parent().unwrap_or(0);
            let process_name = format!("{} ({})", process.name(), pid);

            if let Some(parent_node_id) = node_map.get(&parent_pid) {
                let child_node_id = arena.new_node(process_name);
                parent_node_id.append(child_node_id, &mut arena);
                node_map.insert(*pid, child_node_id);
            } else {
                let parent_node_id = arena.new_node(format!("{} ({})", parent_pid, parent_pid));
                let child_node_id = arena.new_node(process_name);
                parent_node_id.append(child_node_id, &mut arena);
                node_map.insert(parent_pid, parent_node_id);
                node_map.insert(*pid, child_node_id);
            }
        }

        let root_pid = 1;
        let root_node_id = node_map[&root_pid];

        print_tree(&arena, root_node_id, 0);
        println!("____________________________________________________ ,\n");

        // Wait for 1 second before refreshing the system information again
        thread::sleep(time::Duration::from_secs(3));
    }
}

fn print_tree(arena: &Arena<String>, node_id: NodeId, depth: usize) {
    let node = arena.get(node_id).unwrap();
    let padding = "|----> ".repeat(depth * 2);
    println!("{}{}", padding, node.get());

    for child_node_id in node_id.children(arena) {
        print_tree(arena, child_node_id, depth + 1);
    }
}
/*use indextree::{Arena, NodeId};
use sysinfo::{ProcessExt, System, SystemExt};
use std::collections::HashMap;

fn main() {
    let mut system = System::new_all();
    system.refresh_all();
    let process_map = system.get_processes();
    let mut arena = Arena::new();
    let mut node_map: HashMap<i32, NodeId> = HashMap::new();

    for (pid, process) in process_map {
        let parent_pid = process.parent().unwrap_or(0);
        let process_name = format!("{} ({})", process.name(), pid);

        if let Some(parent_node_id) = node_map.get(&parent_pid) {
            let child_node_id = arena.new_node(process_name);
            parent_node_id.append(child_node_id, &mut arena);
            node_map.insert(*pid, child_node_id);
        } else {
            let parent_node_id = arena.new_node(format!("{} ({})", parent_pid, parent_pid));
            let child_node_id = arena.new_node(process_name);
            parent_node_id.append(child_node_id, &mut arena);
            node_map.insert(parent_pid, parent_node_id);
            node_map.insert(*pid, child_node_id);
        }
    }

    let root_pid = 1;
    let root_node_id = node_map[&root_pid];

    print_tree(&arena, root_node_id, 0);
}

fn print_tree(arena: &Arena<String>, node_id: NodeId, depth: usize) {
    let node = arena.get(node_id).unwrap();
    let padding = "|----> ".repeat(depth * 1);
    println!("{}{}", padding, node.get());

    for child_node_id in node_id.children(arena) {
        print_tree(arena, child_node_id, depth + 1);
    }
}
*/