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
    let mut memo =
        HashMap::with_capacity_and_hasher(graph.device_graph.len(), rustc_hash::FxBuildHasher);

    // SEQUENCE 1: svr -> fft -> dac -> GOAL
    let seq1 = {
        let s1 = dfs("svr", graph, &mut memo, "fft");
        memo.clear();
        let s2 = dfs("fft", graph, &mut memo, "dac");
        memo.clear();
        let s3 = dfs("dac", graph, &mut memo, GOAL);
        s1 * s2 * s3
    };

    memo.clear();

    // SEQUENCE 2: svr -> dac -> fft-> GOAL
    let seq2 = {
        let s1 = dfs("svr", graph, &mut memo, "dac");
        memo.clear();
        let s2 = dfs("dac", graph, &mut memo, "fft");
        memo.clear();
        let s3 = dfs("fft", graph, &mut memo, GOAL);
        s1 * s2 * s3
    };

    seq1 + seq2
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
        neighbors.iter().for_each(|next| {
            total_paths += dfs(next, graph, memo, goal);
        });
    }

    // Store in Cache and Return
    memo.insert(curr, total_paths);
    total_paths
}
