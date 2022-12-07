use std::collections::HashMap;

enum Entry<'a> {
    File(usize, &'a str),
    Dir(&'a str),
}

enum Cmd<'a> {
    Cd(&'a str),
    CdOut,
    Ls(Vec<Entry<'a>>),
}

fn parse(input: &str) -> Option<Vec<Cmd>> {
    use Cmd::*;
    use Entry::*;
    let mut cmds = vec![];
    let mut entries = vec![];
    for line in input.lines().skip(1) {
        let mut words = line.split_whitespace();
        match words.next()? {
            "$" => {
                if !entries.is_empty() {
                    cmds.push(Ls(entries));
                    entries = vec![];
                }
                if let "cd" = words.next()? {
                    cmds.push(match words.next()? {
                        ".." => CdOut,
                        dir => Cd(dir),
                    });
                }
            }
            left => {
                let name = words.next()?;
                entries.push(match left {
                    "dir" => Dir(name),
                    size => File(size.parse().ok()?, name),
                });
            }
        }
    }
    if !entries.is_empty() {
        cmds.push(Ls(entries));
    }
    Some(cmds)
}

enum Fs<'a> {
    Dir(HashMap<&'a str, Fs<'a>>),
    File(usize),
}

fn explore<'a>(dir: &mut HashMap<&'a str, Fs<'a>>, cmds: &mut impl Iterator<Item = Cmd<'a>>) {
    while let Some(cmd) = cmds.next() {
        match cmd {
            Cmd::CdOut => return,
            Cmd::Cd(name) => {
                if !dir.contains_key(name) {
                    dir.insert(name, Fs::Dir(HashMap::new()));
                }
                if let Some(Fs::Dir(child)) = dir.get_mut(name) {
                    explore(child, cmds);
                }
            }
            Cmd::Ls(entries) => {
                for entry in entries {
                    match entry {
                        Entry::Dir(name) => {
                            if !dir.contains_key(name) {
                                dir.insert(name, Fs::Dir(HashMap::new()));
                            }
                        }
                        Entry::File(size, name) => {
                            dir.insert(name, Fs::File(size));
                        }
                    }
                }
            }
        }
    }
}

fn get_size(fs: &Fs) -> usize {
    match fs {
        Fs::Dir(dir) => dir.values().map(get_size).sum(),
        &Fs::File(size) => size,
    }
}

fn get_total_small(fs: &Fs) -> usize {
    match fs {
        Fs::Dir(dir) => {
            let size = dir.values().map(get_size).sum();
            let total_small: usize = dir.values().map(get_total_small).sum();
            total_small + if size <= 100000 { size } else { 0 }
        }
        Fs::File(_) => 0,
    }
}

pub fn puzzle1(input: &str) -> String {
    let cmds = parse(input).unwrap();
    let mut root = HashMap::new();
    explore(&mut root, &mut cmds.into_iter());
    get_total_small(&Fs::Dir(root)).to_string()
}

fn get_smallest(fs: &Fs, need: usize) -> Option<usize> {
    match fs {
        Fs::Dir(dir) => dir
            .values()
            .filter_map(|child| get_smallest(child, need))
            .min()
            .or({
                let size = get_size(fs);
                if size >= need {
                    Some(size)
                } else {
                    None
                }
            }),
        Fs::File(_) => None,
    }
}

pub fn puzzle2(input: &str) -> String {
    let cmds = parse(input).unwrap();
    let mut root = HashMap::new();
    explore(&mut root, &mut cmds.into_iter());
    let fs = Fs::Dir(root);
    let total = get_size(&fs);
    get_smallest(&fs, 30000000 - (70000000 - total))
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), "95437");
    }

    #[test]
    fn test_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), "1306611");
    }

    #[test]
    fn test_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), "24933642");
    }

    #[test]
    fn test_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), "13210366");
    }
}
