use rustc_hash::FxHashMap as HashMap;

const STARTING_DEVICE: &str = "you";
const GOAL: &str = "out";

pub fn part_1(input: &str) -> u128 {
    let device_kv: HashMap<&str, Vec<&str>> = {
        let mut device_kv = HashMap::default();

        for l in input.lines() {
            let (k, value_str) = l.split_once(": ").unwrap();
            let values = value_str.split_whitespace().collect();

            device_kv.insert(k, values);
        }

        device_kv
    };

    let mut memo: HashMap<&str, u128> = HashMap::default();

    dfs(STARTING_DEVICE, &device_kv, &mut memo, GOAL)
}

pub fn part_2(input: &str) -> u128 {
    let device_kv: HashMap<&str, Vec<&str>> = {
        let mut device_kv = HashMap::default();

        for l in input.lines() {
            let (k, value_str) = l.split_once(": ").unwrap();
            let values = value_str.split_whitespace().collect();

            device_kv.insert(k, values);
        }

        device_kv
    };

    let seq_1 = {
        let mut m1 = HashMap::default();
        let s1 = dfs("svr", &device_kv, &mut m1, "fft");

        let mut m2 = HashMap::default();
        let s2 = dfs("fft", &device_kv, &mut m2, "dac");

        let mut m3 = HashMap::default();
        let s3 = dfs("dac", &device_kv, &mut m3, GOAL);

        s1 * s2 * s3
    };
    //
    // SEQUENCE 2: svr -> dac -> fft -> GOAL
    let seq_2 = {
        let mut m1 = HashMap::default();
        let s1 = dfs("svr", &device_kv, &mut m1, "dac");

        let mut m2 = HashMap::default();
        let s2 = dfs("dac", &device_kv, &mut m2, "fft");

        let mut m3 = HashMap::default();
        let s3 = dfs("fft", &device_kv, &mut m3, GOAL);

        s1 * s2 * s3
    };

    seq_2 + seq_1
}

fn dfs<'a>(
    curr: &'a str,
    graph: &HashMap<&'a str, Vec<&'a str>>,
    memo: &mut HashMap<&'a str, u128>,
    goal: &str,
) -> u128 {
    if let Some(&count) = memo.get(curr) {
        return count;
    }

    if curr == goal {
        return 1;
    }

    let mut total_paths = 0;

    if let Some(neighbors) = graph.get(curr) {
        for next in neighbors {
            total_paths += dfs(next, graph, memo, goal);
        }
    }

    // Store in Cache and Return
    memo.insert(curr, total_paths);
    total_paths
}
