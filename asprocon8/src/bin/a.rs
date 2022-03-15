use itertools::Itertools;

const TL: u128 = 2000;
const TL_ROOM: u128 = 100;

fn main() {
    let timer = Timer::init();
    proconio::input! {
        s: usize, c: usize, h: usize, a: usize, b: usize,
        nn: [[usize; c]; s],
        kk: [usize; s],
        aa: [[usize; s]; s],
        bb: [[usize; c]; c],
    }

    let mut conveyor = Conveyor::init(h, &kk);

    dbg!(timer.get_ms());
}

#[derive(Debug, Clone, Copy, Default)]
struct Product {
    surface: usize,
    colour: usize,
}

#[derive(Debug, Clone, Default)]
struct Conveyor {
    len: usize,
    displacement: usize,
    hangers: Vec<Option<usize>>,
    rest_hangers: Vec<usize>,
}

impl Conveyor {
    fn len(&self) -> usize {
        self.len
    }

    fn init(h: usize, kk: &[usize]) -> Self {
        Self {
            len: h,
            displacement: 0,
            hangers: vec![None; h],
            rest_hangers: kk.to_vec(),
        }
    }

    fn get(&self) -> &Option<usize> {
        self.hangers.get(self.displacement % self.len).unwrap()
    }

    fn get_mut(&mut self) -> &Option<usize> {
        self.hangers.get_mut(self.displacement % self.len).unwrap()
    }
}

#[derive(Clone, Debug)]
struct History {
    x: usize,
    operations: Vec<Op>,
}

impl History {
    fn init(h: usize) -> Self {
        Self {
            x: 0,
            operations: Vec::new(),
        }
    }

    fn output(&self) {
        println!("{}", self.operations.len());
        for op in self.operations.iter() {
            println!("{}", op);
        }
    }

    fn check(&self, h: usize, kk: &[usize]) -> Result<usize, &str> {
        let l = self.operations.len();
        let mut conveyor = Conveyor::init(h, &kk);
        let mut x = 0usize;
        for &op in self.operations.iter() {
            match op {
                Op::DoNothing => {}
                Op::PutOffHanger => {
                    if let Some(hanger) = conveyor.hangers[conveyor.displacement] {
                        conveyor.hangers[conveyor.displacement] = None;
                        conveyor.rest_hangers[hanger] += 1;
                        x += 1;
                    } else {
                        return Err("Hanger is not equipped");
                    }
                }
                Op::PlaceProduct(p @ Product { surface, colour }) => {
                    if let Some(hanger) = conveyor.hangers[conveyor.displacement] {
                        conveyor.hangers[conveyor.displacement] = None;
                        conveyor.rest_hangers[hanger] += 1;
                        x += 1;
                    }
                    conveyor.hangers[conveyor.displacement] = Some(surface);
                }
            }
        }

        todo!()
    }
}

#[derive(Clone, Copy, Debug)]
enum Op {
    DoNothing,
    PutOffHanger,
    PlaceProduct(Product),
}

impl std::fmt::Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = match self {
            Op::DoNothing => "-2".to_string(),
            Op::PutOffHanger => "-1".to_string(),
            Op::PlaceProduct(Product { surface, colour }) => format!("{} {}", surface, colour),
        };
        f.write_str(&data)
    }
}

fn score(a: usize, b: usize, x: usize, l: usize, nn: &[Vec<usize>]) -> usize {
    std::cmp::max(
        1,
        10000000 - a * x - b * (l - nn.iter().flatten().sum::<usize>()),
    )
}

struct Timer {
    initialized: std::time::Instant,
}

impl Timer {
    fn init() -> Self {
        Self {
            initialized: std::time::Instant::now(),
        }
    }

    fn get(&self) -> std::time::Duration {
        std::time::Instant::now() - self.initialized
    }

    fn get_ms(&self) -> u128 {
        self.get().as_millis()
    }
}
