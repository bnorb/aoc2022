use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    hash::Hash,
    rc::Rc,
};

use regex::Regex;

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
pub enum Robot {
    Ore(u16),           // Ore cost
    Clay(u16),          // Ore cost
    Obsidian(u16, u16), // Ore cost, clay cost
    Geode(u16, u16),    // Ore cost, obsidian cost
}

impl Robot {
    pub fn parse_blueprint(blueprint: &str) -> [Self; 4] {
        let re = Regex::new(
            r"Blueprint \d+: Each ore robot costs (\d+) ore\. Each clay robot costs (\d+) ore\. Each obsidian robot costs (\d+) ore and (\d+) clay\. Each geode robot costs (\d+) ore and (\d+) obsidian\.",
        ).unwrap();

        let cap = re.captures_iter(blueprint).next().unwrap();
        [
            Self::Ore(cap[1].parse().unwrap()),
            Self::Clay(cap[2].parse().unwrap()),
            Self::Obsidian(cap[3].parse().unwrap(), cap[4].parse().unwrap()),
            Self::Geode(cap[5].parse().unwrap(), cap[6].parse().unwrap()),
        ]
    }

    fn buildable(&self, resources: &[u16; 4]) -> bool {
        match self {
            Self::Ore(ore_cost) => *ore_cost <= resources[0],
            Self::Clay(ore_cost) => *ore_cost <= resources[0],
            Self::Obsidian(ore_cost, clay_cost) => {
                *ore_cost <= resources[0] && *clay_cost <= resources[1]
            }
            Self::Geode(ore_cost, obsidian_cost) => {
                *ore_cost <= resources[0] && *obsidian_cost <= resources[2]
            }
        }
    }

    fn build(&self, robot_counts: &mut [u16; 4], resources: &mut [u16; 4]) {
        match self {
            Self::Ore(ore_cost) => {
                resources[0] -= ore_cost;
                robot_counts[0] += 1
            }
            Self::Clay(ore_cost) => {
                resources[0] -= ore_cost;
                robot_counts[1] += 1
            }
            Self::Obsidian(ore_cost, clay_cost) => {
                resources[0] -= ore_cost;
                resources[1] -= clay_cost;
                robot_counts[2] += 1
            }
            Self::Geode(ore_cost, obsidian_cost) => {
                resources[0] -= ore_cost;
                resources[2] -= obsidian_cost;
                robot_counts[3] += 1
            }
        }
    }

    fn destroy(&self, robot_counts: &mut [u16; 4], resources: &mut [u16; 4]) {
        match self {
            Self::Ore(ore_cost) => {
                resources[0] += ore_cost;
                robot_counts[0] -= 1
            }
            Self::Clay(ore_cost) => {
                resources[0] += ore_cost;
                robot_counts[1] -= 1
            }
            Self::Obsidian(ore_cost, clay_cost) => {
                resources[0] += ore_cost;
                resources[1] += clay_cost;
                robot_counts[2] -= 1
            }
            Self::Geode(ore_cost, obsidian_cost) => {
                resources[0] += ore_cost;
                resources[2] += obsidian_cost;
                robot_counts[3] -= 1
            }
        }
    }

    fn resource_need(&self) -> [u16; 4] {
        match self {
            Self::Ore(ore_cost) => [*ore_cost, 0, 0, 0],
            Self::Clay(ore_cost) => [*ore_cost, 0, 0, 0],
            Self::Obsidian(ore_cost, clay_cost) => [*ore_cost, *clay_cost, 0, 0],
            Self::Geode(ore_cost, obsidian_cost) => [*ore_cost, 0, *obsidian_cost, 0],
        }
    }
}

#[derive(PartialEq, Hash, Eq)]
enum Action {
    Wait(u8),
    BuildRobot(Robot, u8),
}

impl Action {
    fn do_it(&self, robot_counts: &mut [u16; 4], resources: &mut [u16; 4]) {
        match self {
            Self::BuildRobot(robot, _) => robot.build(robot_counts, resources),
            Self::Wait(_) => (),
        }
    }

    fn undo_it(&self, robot_counts: &mut [u16; 4], resources: &mut [u16; 4]) {
        match self {
            Self::BuildRobot(robot, _) => robot.destroy(robot_counts, resources),
            Self::Wait(_) => (),
        }
    }

    fn get_prio(&self) -> u8 {
        match self {
            Self::BuildRobot(_, prio) => *prio,
            Self::Wait(prio) => *prio,
        }
    }
}

impl PartialOrd for Action {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.get_prio().cmp(&other.get_prio()))
    }
}

impl Ord for Action {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

type State = (u8, [u16; 4], [u16; 4]);
type CacheIndex<K0, K1, K2> =
    HashMap<K0, (HashMap<K1, Vec<Rc<State>>>, HashMap<K2, Vec<Rc<State>>>)>;

struct Cache {
    set: HashSet<Rc<State>>,
    time_index: CacheIndex<u8, [u16; 4], [u16; 4]>,
    resource_index: CacheIndex<[u16; 4], u8, [u16; 4]>,
    robot_index: CacheIndex<[u16; 4], u8, [u16; 4]>,
}

impl Cache {
    fn new(initial_state: State) -> Self {
        let mut cache = Self {
            set: HashSet::new(),
            time_index: HashMap::new(),
            robot_index: HashMap::new(),
            resource_index: HashMap::new(),
        };

        cache.save_state(initial_state);
        cache
    }

    fn is_prev_better(prev: &Rc<State>, current: &State) -> bool {
        if prev.0 < current.0 {
            return false;
        }

        if prev
            .1
            .iter()
            .enumerate()
            .any(|(i, res)| *res < current.1[i])
        {
            return false;
        }

        if prev
            .2
            .iter()
            .enumerate()
            .any(|(i, rob)| *rob < current.2[i])
        {
            return false;
        }

        true
    }

    fn check_index<K0, K1, K2>(
        index: &CacheIndex<K0, K1, K2>,
        keys: (&K0, &K1, &K2),
        state: &State,
    ) -> bool
    where
        K0: Hash + Eq,
        K1: Hash + Eq,
        K2: Hash + Eq,
    {
        if let Some((map_0, map_1)) = index.get(keys.0) {
            if let Some(prev_states) = map_0.get(keys.1) {
                if prev_states
                    .iter()
                    .any(|prev| Self::is_prev_better(prev, state))
                {
                    return true;
                }
            }

            if let Some(prev_states) = map_1.get(keys.2) {
                return prev_states
                    .iter()
                    .any(|prev| Self::is_prev_better(prev, state));
            }
        }

        false
    }

    fn save_to_index<K0, K1, K2>(
        index: &mut CacheIndex<K0, K1, K2>,
        keys: (&K0, &K1, &K2),
        state_rc: &Rc<State>,
    ) where
        K0: Hash + Eq + Copy,
        K1: Hash + Eq + Copy,
        K2: Hash + Eq + Copy,
    {
        if !index.contains_key(keys.0) {
            index.insert(*keys.0, (HashMap::new(), HashMap::new()));
        }

        let (map_0, map_1) = index.get_mut(&keys.0).unwrap();
        if !map_0.contains_key(keys.1) {
            map_0.insert(*keys.1, Vec::new());
        }

        if !map_1.contains_key(keys.2) {
            map_1.insert(*keys.2, Vec::new());
        }

        map_0.get_mut(&keys.1).unwrap().push(Rc::clone(state_rc));
        map_1.get_mut(&keys.2).unwrap().push(Rc::clone(state_rc));
    }

    fn check_indexes(&self, state: &State) -> bool {
        if Self::check_index(&self.time_index, (&state.0, &state.1, &state.2), &state) {
            return true;
        }

        if Self::check_index(&self.resource_index, (&state.1, &state.0, &state.2), &state) {
            return true;
        }

        Self::check_index(&self.robot_index, (&state.2, &state.0, &state.1), &state)
    }

    fn save_state(&mut self, state: State) {
        let rc = Rc::new(state);
        Self::save_to_index(&mut self.time_index, (&state.0, &state.1, &state.2), &rc);
        Self::save_to_index(
            &mut self.resource_index,
            (&state.1, &state.0, &state.2),
            &rc,
        );
        Self::save_to_index(&mut self.robot_index, (&state.2, &state.0, &state.1), &rc);

        self.set.insert(rc);
    }

    fn has_better(&mut self, state: State) -> bool {
        let has_better = self.set.contains(&state) || self.check_indexes(&state);
        self.save_state(state);

        has_better
    }
}

pub struct Simulation {
    max_ore_need: u16,
    initial_robot_counts: [u16; 4],
    blueprint: [Robot; 4],
    time_limit: u8,
    max_geodes_cracked: u16,
    cache: Cache,
}

impl Simulation {
    pub fn new(blueprint: [Robot; 4], time_limit: u8) -> Self {
        Self {
            initial_robot_counts: [1, 0, 0, 0],
            blueprint,
            time_limit,
            max_geodes_cracked: 0,
            cache: Cache::new((time_limit, [0, 0, 0, 0], [1, 0, 0, 0])),
            max_ore_need: blueprint
                .iter()
                .fold(0, |max, robot| max.max(robot.resource_need()[0])),
        }
    }

    fn get_actions(
        &self,
        time_left: u8,
        robot_counts: &[u16; 4],
        resources: &[u16; 4],
    ) -> Vec<Action> {
        if time_left <= 1 {
            // no need to build shit
            return vec![Action::Wait(0)];
        }

        // if we can build a geode cracking robot, do only that
        if self.blueprint[3].buildable(resources) {
            return vec![Action::BuildRobot(self.blueprint[3], 0)];
        }

        let mut actions: HashSet<Action> = self.blueprint[..=2]
            .iter()
            .rev()
            .enumerate()
            .filter(|(_, robot)| robot.buildable(resources))
            .map(|(i, robot)| Action::BuildRobot(*robot, i as u8))
            .collect();

        // ====== check robot amounts ======

        // if we have enough obsidian robots to build a geode bot per minute, we don't need more
        if robot_counts[2] >= self.blueprint[3].resource_need()[2] {
            actions.remove(&Action::BuildRobot(self.blueprint[2], 0));
        }

        // if we have enough clay robots to build an obsidian bot per minute, we don't need more
        if robot_counts[1] >= self.blueprint[2].resource_need()[1] {
            actions.remove(&Action::BuildRobot(self.blueprint[1], 1));
        }

        // if we have enough ore robots to build any bot per minute, we don't need more
        if robot_counts[0] >= self.max_ore_need {
            actions.remove(&Action::BuildRobot(self.blueprint[0], 2));
        }

        // ====== check resource amounts ======
        let t = time_left as u16 - 1;
        // if we have enough obsidian to build a geode robot every remaining turn, we don't need more obsidian robots
        if resources[2] + (t * robot_counts[2]) >= t * self.blueprint[3].resource_need()[2] {
            actions.remove(&Action::BuildRobot(self.blueprint[2], 0));
        }

        // if we have enough clay to build an obsidian robot every remaining turn, we don't need more clay robots
        if resources[1] + (t * robot_counts[1]) >= t * self.blueprint[2].resource_need()[1] {
            actions.remove(&Action::BuildRobot(self.blueprint[1], 1));
        }

        // if we have enough ore to build any robot every remaining turn, we don't need more ore robots
        if resources[0] + (t * robot_counts[0]) >= t * self.max_ore_need {
            actions.remove(&Action::BuildRobot(self.blueprint[0], 2));
        }

        if actions.len() == 0 {
            return vec![Action::Wait(0)];
        }

        // we know we can build something

        actions.insert(Action::Wait(3));

        // if we have twice the max ore (including the amount we'll generate next turn), there's no reason to wait
        if resources[0] + robot_counts[0] >= self.max_ore_need * 2 {
            actions.remove(&Action::Wait(3));
        }

        // prioritize actions
        let mut actions: Vec<Action> = actions.into_iter().collect();
        actions.sort();

        actions
    }

    fn gather_resources(robot_counts: &[u16; 4], resources: &mut [u16; 4]) {
        resources
            .iter_mut()
            .enumerate()
            .for_each(|(i, resource)| *resource += robot_counts[i])
    }

    fn put_back_resources(robot_counts: &[u16; 4], resources: &mut [u16; 4]) {
        resources
            .iter_mut()
            .enumerate()
            .for_each(|(i, resource)| *resource -= robot_counts[i])
    }

    fn run_step(&mut self, time_left: u8, resources: &mut [u16; 4], robot_counts: &mut [u16; 4]) {
        if time_left == 0 {
            if resources[3] > self.max_geodes_cracked {
                self.max_geodes_cracked = resources[3];
            }
            return;
        }

        let actions = self.get_actions(time_left, robot_counts, resources);
        Self::gather_resources(robot_counts, resources);

        for action in actions {
            action.do_it(robot_counts, resources);

            let state = (time_left - 1, *resources, *robot_counts);
            if !self.cache.has_better(state) {
                self.run_step(time_left - 1, resources, robot_counts);
            }

            action.undo_it(robot_counts, resources);
        }

        Self::put_back_resources(robot_counts, resources);
    }

    pub fn run(&mut self) -> u16 {
        let mut resources = [0, 0, 0, 0];
        let mut robot_counts = self.initial_robot_counts.clone();

        self.run_step(self.time_limit, &mut resources, &mut robot_counts);

        self.max_geodes_cracked
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_blueprint() {
        let bp = "Blueprint 1: Each ore robot costs 1 ore. Each clay robot costs 2 ore. Each obsidian robot costs 4 ore and 8 clay. Each geode robot costs 6 ore and 15 obsidian.";
        assert_eq!(
            Robot::parse_blueprint(bp),
            [
                Robot::Ore(1),
                Robot::Clay(2),
                Robot::Obsidian(4, 8),
                Robot::Geode(6, 15)
            ]
        );
    }

    #[test]
    fn cache_works() {
        let mut cache = Cache::new((24, [1, 0, 0, 0], [1, 0, 0, 0]));
        assert_eq!(cache.has_better((24, [1, 0, 0, 0], [1, 0, 0, 0])), true);
        assert_eq!(cache.has_better((23, [1, 0, 0, 0], [1, 0, 0, 0])), true);
        assert_eq!(cache.has_better((24, [0, 0, 0, 0], [1, 0, 0, 0])), true);
        assert_eq!(cache.has_better((24, [1, 0, 0, 0], [0, 0, 0, 0])), true);
        assert_eq!(cache.has_better((24, [1, 1, 0, 0], [1, 0, 0, 0])), false);
        assert_eq!(cache.has_better((24, [1, 1, 0, 0], [1, 0, 0, 0])), true);
        assert_eq!(cache.has_better((24, [0, 1, 0, 0], [1, 0, 0, 0])), true);
        assert_eq!(cache.has_better((24, [0, 1, 1, 0], [1, 0, 0, 0])), false);
    }
}
