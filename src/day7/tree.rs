use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub struct Node {
    sub_dirs: HashMap<String, Rc<RefCell<Self>>>,
    total_file_size: u32,
    total_size: Option<u32>,
}

impl Node {
    pub fn new() -> Self {
        Node {
            sub_dirs: HashMap::new(),
            total_file_size: 0,
            total_size: None,
        }
    }

    pub fn flatten_children(&self) -> Vec<Rc<RefCell<Self>>> {
        let mut res = Vec::new();

        self.sub_dirs.iter().for_each(|(_, child)| {
            res.push(Rc::clone(child));
            res.extend(child.borrow().flatten_children())
        });

        res
    }

    pub fn size(&mut self) -> u32 {
        match self.total_size {
            Some(total) => total,
            None => {
                let total = self
                    .sub_dirs
                    .iter()
                    .fold(self.total_file_size, |sum, (_, child)| {
                        sum + child.borrow_mut().size()
                    });

                self.total_size = Some(total);
                total
            }
        }
    }

    pub fn add_child_if_absent(
        &mut self,
        name: &str,
        node: Rc<RefCell<Node>>,
    ) -> Rc<RefCell<Node>> {
        match self.sub_dirs.get(name) {
            Some(child) => Rc::clone(child),
            None => {
                self.sub_dirs.insert(String::from(name), Rc::clone(&node));
                node
            }
        }
    }
}

#[derive(Debug, PartialEq)]
enum Command<'a> {
    MoveToRoot,
    MoveUp,
    MoveInto(&'a str),
    List(Vec<&'a str>),
}

impl<'a> Command<'a> {
    pub fn cd(target: &'a str) -> Self {
        match target {
            "/" => Self::MoveToRoot,
            ".." => Self::MoveUp,
            dir => Self::MoveInto(dir),
        }
    }

    pub fn ls(items: Vec<&'a str>) -> Self {
        Self::List(items)
    }
}

pub fn parse(input: &str) -> Rc<RefCell<Node>> {
    build_nodes(parse_commands(input))
}

fn parse_commands(input: &str) -> Vec<Command> {
    let mut cmds = Vec::new();
    let mut lines = input.lines().peekable();

    while let Some(line) = lines.next() {
        let mut parts = line.split(' ');
        assert_eq!(parts.next().unwrap(), "$"); // other lines can only come after an ls command

        let cmd = match parts.next() {
            Some("cd") => Command::cd(parts.next().unwrap()),
            Some("ls") => {
                let mut v = Vec::new();
                while let Some(next_line) = lines.peek() {
                    if next_line.starts_with("$") {
                        break;
                    }
                    v.push(lines.next().unwrap())
                }

                Command::ls(v)
            }
            other => panic!("invalid input: {}", other.unwrap_or("nothing")),
        };

        cmds.push(cmd);
    }

    cmds
}

fn build_nodes(cmds: Vec<Command>) -> Rc<RefCell<Node>> {
    let root = Rc::new(RefCell::new(Node::new()));
    let mut stack: Vec<Rc<RefCell<Node>>> = Vec::new();

    let mut iter = cmds.iter();

    while let Some(cmd) = iter.next() {
        match cmd {
            Command::MoveToRoot => stack.clear(),
            Command::MoveUp => _ = stack.pop(),
            Command::MoveInto(dir) => {
                let current = Rc::clone(stack.last().unwrap_or(&root));

                stack.push(
                    current
                        .borrow_mut()
                        .add_child_if_absent(dir, Rc::new(RefCell::new(Node::new()))),
                );
            }
            Command::List(items) => items.iter().for_each(|item| {
                let parts: Vec<&str> = item.split(' ').collect();
                let (first, name) = match &parts[..2] {
                    &[first, name] => (first, name),
                    _ => unreachable!(),
                };

                let current = stack.last().unwrap_or(&root);
                match first {
                    "dir" => {
                        _ = current
                            .borrow_mut()
                            .add_child_if_absent(name, Rc::new(RefCell::new(Node::new())))
                    }
                    size => {
                        let size: u32 = size.trim().parse().unwrap();
                        current.borrow_mut().total_file_size += size;
                    }
                };
            }),
        }
    }

    root
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_calculate_size() {
        let commands = vec![
            Command::MoveToRoot,
            Command::List(vec!["dir a", "14848514 b.txt", "8504156 c.dat", "dir d"]),
            Command::MoveInto("a"),
            Command::List(vec!["584 i"]),
            Command::MoveUp,
        ];

        let root = build_nodes(commands);
        assert_eq!(root.borrow().total_size, None);
        assert_eq!(root.borrow_mut().size(), 14848514 + 8504156 + 584);
        assert_eq!(root.borrow().total_size, Some(14848514 + 8504156 + 584));
    }

    #[test]
    fn can_build_nodes() {
        let commands = vec![
            Command::MoveToRoot,
            Command::List(vec!["dir a", "14848514 b.txt", "8504156 c.dat", "dir d"]),
            Command::MoveInto("a"),
            Command::List(vec!["584 i"]),
            Command::MoveUp,
        ];

        let root = build_nodes(commands);

        assert_eq!(root.borrow().total_file_size, 14848514 + 8504156);
        assert_eq!(root.borrow().sub_dirs.len(), 2);

        let child = Rc::clone(root.borrow().sub_dirs.get("a").unwrap());

        assert_eq!(child.borrow().total_file_size, 584);
        assert_eq!(child.borrow().sub_dirs.len(), 0);
    }

    #[test]
    fn can_parse_input() {
        let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
584 i
$ cd ..";

        let expected = vec![
            Command::MoveToRoot,
            Command::List(vec!["dir a", "14848514 b.txt", "8504156 c.dat", "dir d"]),
            Command::MoveInto("a"),
            Command::List(vec!["584 i"]),
            Command::MoveUp,
        ];

        assert_eq!(parse_commands(input), expected);
    }
}
