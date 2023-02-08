use std::collections::{BTreeSet, HashMap, HashSet, LinkedList};

fn parse_graph(input: &str) -> HashMap<String, Vec<String>> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split("; ");
            let first = split.next().unwrap();
            let second = split.next().unwrap();

            let name = &first[6..8];
            let start = if second.contains("valves") { 23 } else { 22 };

            let valves: Vec<String> = second[start..]
                .split(", ")
                .map(|p| String::from(p))
                .collect();

            (name, valves)
        })
        .fold(HashMap::new(), |mut map, (name, valves)| {
            map.insert(String::from(name), valves);
            map
        })
}

fn parse_rates(input: &str) -> HashMap<String, u16> {
    input
        .lines()
        .map(|line| {
            let first = line.split(";").next().unwrap();
            let name = &first[6..8];
            let rate: u16 = first[23..].parse().unwrap();

            (name, rate)
        })
        .filter(|(_, rate)| *rate > 0)
        .fold(HashMap::new(), |mut map, (name, rate)| {
            map.insert(String::from(name), rate);
            map
        })
}

fn find_shortest_routes(
    graph: HashMap<String, Vec<String>>,
    rates: &HashMap<String, u16>,
) -> HashMap<String, Vec<(String, u16)>> {
    let mut cache = HashMap::new();

    let mut bfs = |start: &str, end: &str| -> u16 {
        if let Some(cached) = cache.get(&(String::from(start), String::from(end))) {
            return *cached;
        }

        let mut queue = LinkedList::from([(start, 0)]);
        let mut visited = HashSet::from([start]);

        while let Some((current, steps)) = queue.pop_front() {
            if current == end {
                cache.insert((String::from(start), String::from(end)), steps);
                cache.insert((String::from(end), String::from(start)), steps);
                return steps;
            }

            let unvisited: Vec<&str> = graph
                .get(current)
                .unwrap()
                .iter()
                .map(|next| next.as_str())
                .filter(|next| !visited.contains(next))
                .collect();

            unvisited.iter().for_each(|next| {
                visited.insert(next);
                queue.push_back((next, steps + 1));
            })
        }

        unreachable!()
    };

    let names: Vec<&str> = rates.iter().map(|(name, _)| name.as_str()).collect();
    let mut valve_graph = HashMap::new();

    for start in names.iter() {
        let mut ends = Vec::new();

        for end in names.iter() {
            if start == end {
                continue;
            }

            let distance = bfs(start, end);
            ends.push((String::from(*end), distance))
        }

        valve_graph.insert(String::from(*start), ends);
    }

    valve_graph.insert(
        String::from("START"),
        names
            .into_iter()
            .map(|end| (String::from(end), bfs("AA", end)))
            .collect(),
    );

    valve_graph
}

#[aoc_generator(day16)]
fn input_generator(input: &str) -> (HashMap<String, Vec<(String, u16)>>, HashMap<String, u16>) {
    let graph = parse_graph(input);
    let rates = parse_rates(input);

    (find_shortest_routes(graph, &rates), rates)
}

fn dfs(
    graph: &HashMap<String, Vec<(String, u16)>>,
    rates: &HashMap<String, u16>,
    current: &str,
    time_left: u16,
    mut released: u16,
    opened: &mut BTreeSet<String>,
    max_released: &mut u16,
    all_maxes: &mut HashMap<String, u16>,
) {
    if current != "START" {
        released += time_left * rates.get(current).unwrap();
    }

    if released > *max_released {
        *max_released = released;
    }

    if opened.len() > 0 {
        let mut iter = opened.iter();
        let hash = String::from(iter.next().unwrap());
        let hash = iter.fold(hash, |mut str, curr| {
            str.push(':');
            str.push_str(curr);
            str
        });

        match all_maxes.get(&hash) {
            None => {
                all_maxes.insert(hash, released);
            }
            Some(max) => {
                if released > *max {
                    all_maxes.insert(hash, released);
                }
            }
        }
    }

    if time_left == 0 || graph.len() == opened.len() {
        return;
    }

    let nexts: Vec<&(String, u16)> = graph
        .get(current)
        .unwrap()
        .iter()
        .filter(|(next, distance)| !opened.contains(next.as_str()) && time_left > distance + 1)
        .collect();

    nexts.into_iter().for_each(|(next, distance)| {
        opened.insert(next.clone());
        dfs(
            graph,
            rates,
            &next,
            time_left - distance - 1,
            released,
            opened,
            max_released,
            all_maxes,
        );
        opened.remove(next);
    });
}

#[aoc(day16, part1)]
fn most_steam_alone(
    (valve_graph, rates): &(HashMap<String, Vec<(String, u16)>>, HashMap<String, u16>),
) -> u16 {
    let mut max_released = 0;
    let mut opened = BTreeSet::new();
    let mut all = HashMap::new();
    dfs(
        valve_graph,
        rates,
        "START",
        30,
        0,
        &mut opened,
        &mut max_released,
        &mut all,
    );

    max_released
}

#[aoc(day16, part2)]
fn most_steam_together(
    (valve_graph, rates): &(HashMap<String, Vec<(String, u16)>>, HashMap<String, u16>),
) -> u16 {
    let mut max_released = 0;
    let mut opened = BTreeSet::new();
    let mut all = HashMap::new();
    dfs(
        valve_graph,
        rates,
        "START",
        26,
        0,
        &mut opened,
        &mut max_released,
        &mut all,
    );

    let mut all: Vec<(String, u16)> = all.into_iter().collect();
    all.sort_by(|(_, a), (_, b)| b.cmp(a));

    for (i, (hash, released)) in all.iter().enumerate() {
        let set: HashSet<&str> = HashSet::from_iter(hash.split(':'));

        for (hash_ele, released_ele) in all.iter().skip(i + 1) {
            if hash_ele.split(':').all(|valve| !set.contains(valve)) {
                max_released = max_released.max(released + released_ele);
                break;
            }
        }
    }

    max_released
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_rates() {
        let str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB";

        assert_eq!(
            parse_rates(str),
            HashMap::from([(String::from("BB"), 13), (String::from("CC"), 2)])
        )
    }

    #[test]
    fn can_parse_graph() {
        let str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB";

        assert_eq!(
            parse_graph(str),
            HashMap::from([
                (
                    String::from("AA"),
                    vec![String::from("DD"), String::from("II"), String::from("BB")]
                ),
                (
                    String::from("BB"),
                    vec![String::from("CC"), String::from("AA")]
                ),
                (
                    String::from("CC"),
                    vec![String::from("DD"), String::from("BB")]
                ),
            ])
        );
    }

    #[test]
    fn can_parse_valve_graph() {
        let str = "Valve AA has flow rate=0; tunnels lead to valves DD, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA";

        let map = find_shortest_routes(parse_graph(str), &parse_rates(str));

        assert!(map.contains_key("BB"));
        assert!(map.contains_key("CC"));
        assert!(map.contains_key("DD"));

        assert_eq!(map.get("BB").unwrap().len(), 2);
        assert!(map.get("BB").unwrap().contains(&(String::from("CC"), 1)));
        assert!(map.get("BB").unwrap().contains(&(String::from("DD"), 2)));

        assert_eq!(map.get("CC").unwrap().len(), 2);
        assert!(map.get("CC").unwrap().contains(&(String::from("BB"), 1)));
        assert!(map.get("CC").unwrap().contains(&(String::from("DD"), 1)));

        assert_eq!(map.get("DD").unwrap().len(), 2);
        assert!(map.get("DD").unwrap().contains(&(String::from("CC"), 1)));
        assert!(map.get("DD").unwrap().contains(&(String::from("BB"), 2)));
    }
}
