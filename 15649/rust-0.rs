use std::io::{ stdin, stdout, BufWriter, Write };

struct Stack<T> {
    stack: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { stack: Vec::new() }
    }

    fn push(&mut self, item: T) {
        self.stack.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let stdout = stdout();
    let mut out = BufWriter::new(stdout.lock());

    let (n, m) = (input.next().unwrap(), input.next().unwrap());
    let mut arr = vec![0; n as usize];
    let mut visited = vec![false; n as usize];
    let mut stack: Stack<i32> = Stack::new();

    for i in 0..n {
        arr[i as usize] = i + 1;
    }

    search(&arr, &mut visited, &mut stack, n, m, 0, &mut out);
}

fn search(
    arr: &[i32],
    visited: &mut Vec<bool>,
    stack: &mut Stack<i32>,
    n: i32,
    m: i32,
    idx: i32,
    out: &mut BufWriter<std::io::StdoutLock>
) {
    if idx == m {
        writeln!(
            out,
            "{}",
            stack.stack
                .iter()
                .map(|&x| x.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        ).unwrap();
        return;
    }

    for i in 0..n {
        if visited[i as usize] {
            continue;
        }
        visited[i as usize] = true;
        stack.push(arr[i as usize]);
        search(arr, visited, stack, n, m, idx + 1, out);
        stack.pop();
        visited[i as usize] = false;
    }
}
