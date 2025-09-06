use clap::Parser;
use rand::Rng;
use std::collections::{HashMap, HashSet, LinkedList, VecDeque};
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(value_enum)]
    data_structure: DataStructure,

    #[arg(short, long, default_value_t = 1_000_000)]
    size: usize,
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum DataStructure {
    Vec,
    VecDeque,
    LinkedList,
    HashMap,
    HashSet,
}

fn main() {
    let args = Args::parse();
    println!(
        "Starting Checker for {:?} with {} elements...\n",
        args.data_structure, args.size
    );

    match args.data_structure {
        DataStructure::Vec => benchmark_vec(args.size),
        DataStructure::VecDeque => benchmark_vecdeque(args.size),
        DataStructure::LinkedList => benchmark_linkedlist(args.size),
        DataStructure::HashMap => benchmark_hashmap(args.size),
        DataStructure::HashSet => benchmark_hashset(args.size),
    }

    println!("\nChecking complete.");
}

fn benchmark_vec(size: usize) {
    println!("--- Checking Vec<T> ---");
    let mut vec = Vec::with_capacity(size);

    let start = Instant::now();
    for i in 0..size {
        vec.push(i);
    }
    println!("  - Fill (push back): {:?}", start.elapsed());

    let start = Instant::now();
    let sum: usize = vec.iter().sum();
    println!("  - Iteration and Sum: {:?} (Sum: {})", start.elapsed(), sum);

    let mut rng = rand::thread_rng();
    let index_to_access = rng.gen_range(0..size);
    let start = Instant::now();
    let value = vec[index_to_access];
    println!(
        "  - Random Access [{}]: {:?} (Value: {})",
        index_to_access,
        start.elapsed(),
        value
    );

    let start = Instant::now();
    vec.insert(0, size);
    println!("  - Front Insertion (insert at 0): {:?}", start.elapsed());

    let start = Instant::now();
    vec.remove(0);
    println!("  - Front Removal (remove from 0): {:?}", start.elapsed());
}


fn benchmark_vecdeque(size: usize) {
    println!("--- Checking VecDeque<T> ---");
    let mut deque = VecDeque::with_capacity(size);

    let start = Instant::now();
    for i in 0..size {
        deque.push_back(i);
    }
    println!("  - Fill (push_back): {:?}", start.elapsed());

    let start = Instant::now();
    deque.push_front(size);
    println!("  - Front Push (push_front): {:?}", start.elapsed());

    let start = Instant::now();
    let value = deque.pop_front().unwrap();
    println!(
        "  - Front Pop (pop_front): {:?} (Value: {})",
        start.elapsed(),
        value
    );

    let start = Instant::now();
    let sum: usize = deque.iter().sum();
    println!("  - Iteration and Sum: {:?} (Sum: {})", start.elapsed(), sum);

    let mut rng = rand::thread_rng();
    let index_to_access = rng.gen_range(0..size);
    let start = Instant::now();
    let value = deque[index_to_access];
    println!(
        "  - Random Access [{}]: {:?} (Value: {})",
        index_to_access,
        start.elapsed(),
        value
    );
}

fn benchmark_linkedlist(size: usize) {
    println!("--- Checking LinkedList<T> ---");
    let mut list = LinkedList::new();

    let start = Instant::now();
    for i in 0..size {
        list.push_front(i);
    }
    println!("  - Fill (push_front): {:?}", start.elapsed());

    let start = Instant::now();
    let sum: usize = list.iter().sum();
    println!("  - Iteration and Sum: {:?} (Sum: {})", start.elapsed(), sum);

    let mut rng = rand::thread_rng();
    let index_to_access = rng.gen_range(0..size);
    let start = Instant::now();
    let value = list.iter().nth(index_to_access).unwrap();
    println!(
        "  - Find element by 'index' {}: {:?} (Value: {})",
        index_to_access,
        start.elapsed(),
        value
    );
    println!("  - NOTE: Compare this access time to Vec/VecDeque's to see the O(n) nature.");
}

fn benchmark_hashmap(size: usize) {
    println!("--- Checking HashMap<K, V> ---");
    let mut map = HashMap::with_capacity(size);

    let start = Instant::now();
    for i in 0..size {
        map.insert(i, i.to_string());
    }
    println!("  - Insertion: {:?}", start.elapsed());

    let mut rng = rand::thread_rng();
    let key_to_lookup = rng.gen_range(0..size);
    let start = Instant::now();
    let value = map.get(&key_to_lookup).unwrap();
    println!(
        "  - Lookup by key {}: {:?} (Value: {})",
        key_to_lookup,
        start.elapsed(),
        value
    );

    let start = Instant::now();
    let mut count = 0;
    for (_key, _value) in &map {
        count += 1;
    }
    println!("  - Iteration: {:?} ({} items)", start.elapsed(), count);
}

fn benchmark_hashset(size: usize) {
    println!("--- Checking HashSet<T> ---");
    let mut set = HashSet::with_capacity(size);

    let start = Instant::now();
    for i in 0..size {
        set.insert(i);
    }
    println!("  - Insertion: {:?}", start.elapsed());

    let mut rng = rand::thread_rng();
    let value_to_check = rng.gen_range(0..size);
    let start = Instant::now();
    let is_present = set.contains(&value_to_check);
    println!(
        "  - Contains check {}: {:?} (Present: {})",
        value_to_check,
        start.elapsed(),
        is_present
    );

    let start = Instant::now();
    let count = set.iter().count();
    println!("  - Iteration: {:?} ({} items)", start.elapsed(), count);
}

