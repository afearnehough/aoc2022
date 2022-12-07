#[derive(PartialEq)]
enum Kind {
    Directory,
    File(usize)
}

struct Node {
    name: String,
    kind: Kind,
    above: Option<usize>,
    next: Option<usize>,
    below: Option<usize>
    
}

struct FileSystem {
    cwd: usize,
    nodes: Vec<Node>
}

type TraversalResult = (usize, Kind, String);

impl FileSystem {
    fn new() -> Self {
        Self {
            cwd: 0,
            nodes: vec![
                Node {
                    name: "/".to_string(),
                    kind: Kind::Directory,
                    above: None,
                    next: None,
                    below: None
                }
            ]
        }
    }

    fn cd(&mut self, dir: &str) {
        match dir {
            "/" => self.cwd = 0,
            ".." => {
                if let Some(above) = self.nodes[self.cwd].above {
                    self.cwd = above;
                }
            }
            _ => {
                let mut curr = self.nodes[self.cwd].below;
                while let Some(offset) = curr {
                    if self.nodes[offset].kind == Kind::Directory && self.nodes[offset].name == dir {
                        self.cwd = offset;
                        return;
                    } else {
                        curr = self.nodes[offset].next;
                    }
                }
            }
        }
    }
    
    fn add(&mut self, name: &str, kind: Kind) {
        let first = self.nodes[self.cwd].below;
        self.nodes[self.cwd].below = Some(self.nodes.len());
        self.nodes.push(Node {
            name: name.to_string(),
            kind,
            above: Some(self.cwd),
            next: first,
            below: None
        });
    }

    fn traverse(&self, node: usize, result: &mut Vec<TraversalResult>) -> usize {
        match self.nodes[node].kind {
            Kind::Directory => {
                let mut total_size = 0;
                let mut curr = self.nodes[node].below;
                while let Some(offset) = curr {
                    total_size += self.traverse(offset, result);
                    curr = self.nodes[offset].next;
                }

                result.push((total_size, Kind::Directory, self.nodes[node].name.clone()));
                total_size
            },
            Kind::File(size) => size
        }
    }
}

pub fn handler(input: &String) -> Result<(), String> {
    let mut fs = FileSystem::new();

    for line in input.lines() {
        let args = line.split_whitespace().collect::<Vec<&str>>();
        if args[0] == "$" && args[1] == "cd" {
            fs.cd(args[2]);
        } else if args[0] == "dir" {
            fs.add(args[1], Kind::Directory);
        } else if let Ok(size) = args[0].parse::<usize>() {
            fs.add(args[1], Kind::File(size));
        }
    }

    let mut list = Vec::new();
    fs.traverse(0, &mut list);
    
    println!(
        "Part A: {}",
        list.iter()
            .filter(|(size, _, name)| *size < 100000)
            .map(|(size, _, name)| size)
            .sum::<usize>());

    let (total_used_space, _, _) = list.last().unwrap();
    println!(
        "Part B: {}",
        list.iter()
            .filter(|(size, _, name)| *size + (70000000 - total_used_space) >= 30000000)
            .map(|(size, _, name)| size)
            .min()
            .unwrap());

    Ok(())
}