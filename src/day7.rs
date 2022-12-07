enum Kind {
    Directory,
    File(usize)
}

struct Node {
    name: String,
    kind: Kind,
    parent: Option<usize>,
    next: Option<usize>
}

struct FileSystem {
    cwd: usize,
    nodes: Vec<Node>
}

impl FileSystem {
    fn new() -> Self {
        Self {
            cwd: 0,
            nodes: vec![
                Node {
                    name: "/".to_string(),
                    kind: Kind::Directory,
                    parent: None,
                    next: None
                }
            ]
        }
    }

    fn cd(&mut self, dir: &str) {
        match dir {
            "/" => self.cwd = 0,
            _ => {}
        }
    }
    
    fn add(&mut self, name: &str, kind: Kind) {
        let head = self.nodes[self.cwd].next;
        self.nodes[self.cwd].next = Some(self.nodes.len());
        self.nodes.push(Node {
            name: name.to_string(),
            kind,
            parent: Some(self.cwd),
            next: head
        });
    }
}

pub fn handler(input: &String) -> Result<(), String> {
    let mut fs = FileSystem::new();

    for line in input.lines() {
        let args = line.split_whitespace().collect::<Vec<&str>>();
        if args[0] == "$" && args[1] == "cd" {
            fs.cd(args[2]);
        }
    }

    Ok(())
}