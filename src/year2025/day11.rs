use rustc_hash::FxHashMap as HashMap;

const STARTING_DEVICE: &str = "you";
const GOAL: &str = "out";

pub struct Graph<'a> {
    device_graph: HashMap<&'a str, Vec<&'a str>>,
}

pub fn parse<'a>(input: &'a str) -> Graph<'a> {
    let graph = input
        .lines()
        .map(|l| {
            let (k, value_str) = l.split_once(": ").unwrap();
            (k, value_str.split_whitespace().collect())
        })
        .collect();

    Graph {
        device_graph: graph,
    }
}

pub fn part_1(graph: &Graph) -> u64 {
    let mut memo = HashMap::default();
    dfs(STARTING_DEVICE, graph, &mut memo, GOAL)
}

pub fn part_2(graph: &Graph) -> u64 {
    // SEQUENCE 2: svr -> fft -> dac -> GOAL
    let mut map =
        HashMap::with_capacity_and_hasher(graph.device_graph.len(), rustc_hash::FxBuildHasher);
    let s1 = dfs("svr", graph, &mut map, "fft");

    map.clear();
    let s2 = dfs("fft", graph, &mut map, "dac");

    map.clear();
    let s3 = dfs("dac", graph, &mut map, GOAL);

    let seq_1 = s1 * s2 * s3;

    // SEQUENCE 2: svr -> dac -> fft -> GOAL
    map.clear();
    let s1 = dfs("svr", graph, &mut map, "dac");

    map.clear();
    let s2 = dfs("dac", graph, &mut map, "fft");

    map.clear();
    let s3 = dfs("fft", graph, &mut map, GOAL);

    let seq_2 = s1 * s2 * s3;
    // };

    seq_2 + seq_1
}

fn dfs<'a>(curr: &'a str, graph: &'a Graph, memo: &mut HashMap<&'a str, u64>, goal: &str) -> u64 {
    if let Some(&count) = memo.get(curr) {
        return count;
    }

    if curr == goal {
        return 1;
    }

    let mut total_paths = 0;

    if let Some(neighbors) = graph.device_graph.get(curr) {
        for next in neighbors {
            total_paths += dfs(next, graph, memo, goal);
        }
    }

    // Store in Cache and Return
    memo.insert(curr, total_paths);
    total_paths
}
