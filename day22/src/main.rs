const INPUT: &str = include_str!("input.txt");
const EXAMPLE: &str = include_str!("example.txt");

fn main() {
    print_answer("one (example)", &one(EXAMPLE), "5");
    print_answer("one", &one(INPUT), "517");
    print_answer("two (example)", &two(EXAMPLE), "7");
    print_answer("two", &two(INPUT), "61276");
}

fn print_answer(name: &str, actual: &str, expected: &str) {
    match actual == expected {
        true => println!("{name}: {actual} (OK)"),
        false => println!("{name}: {actual} (ERROR: expected {expected})"),
    }
}

fn one(input: &str) -> String {
    World::parse(input)
        .apply_gravity()
        .0
        .removeable_count()
        .to_string()
}

fn two(input: &str) -> String {
    let world = World::parse(input).apply_gravity().0;

    world
        .blocks
        .iter()
        .map(|block| world.blocks_fallen_removing_block(block))
        .sum::<usize>()
        .to_string()
}

struct World {
    blocks: Vec<Block>,
}

impl World {
    fn parse(input: &str) -> Self {
        Self {
            blocks: input.lines().map(|line| Block::parse(line)).collect(),
        }
    }

    fn apply_gravity(&self) -> (Self, usize) {
        let mut dropped = 0usize;
        let mut height_map = HeightMap::from(self);

        let mut new_blocks = self.blocks.clone();
        new_blocks.sort_by_key(|block| block.from.z);
        new_blocks.iter_mut().for_each(|block| {
            if height_map.drop(block) {
                dropped += 1
            }
        });

        (Self { blocks: new_blocks }, dropped)
    }

    fn blocks_fallen_removing_block(&self, block_to_remove: &Block) -> usize {
        let world = Self {
            blocks: self
                .blocks
                .clone()
                .into_iter()
                .filter(|block| block != block_to_remove)
                .collect(),
        };

        world.apply_gravity().1
    }

    fn removeable_count(&self) -> usize {
        self.blocks
            .iter()
            .map(|candidate| {
                (
                    candidate,
                    self.blocks
                        .iter()
                        .filter(|supported_by_candidate_candidate| {
                            candidate.is_supporting(supported_by_candidate_candidate)
                        })
                        .collect::<Vec<&Block>>(),
                )
            })
            .map(|(candidate, supported_by_candidate_vec)| {
                (
                    candidate,
                    supported_by_candidate_vec
                        .iter()
                        .map(|supported_by_candidate| {
                            (
                                *supported_by_candidate,
                                self.blocks
                                    .iter()
                                    .filter(|supporting_supported_by_candidate_candidate| {
                                        supporting_supported_by_candidate_candidate
                                            .is_supporting(supported_by_candidate)
                                    })
                                    .collect::<Vec<&Block>>(),
                            )
                        })
                        .collect::<Vec<(&Block, Vec<&Block>)>>(),
                )
            })
            .filter(|(_, supported_by_candidate_vec)| {
                let all_supported_blocks_supported_by_another_block = supported_by_candidate_vec
                    .iter()
                    .all(|(_, supporting_supported)| supporting_supported.len() > 1);

                all_supported_blocks_supported_by_another_block
            })
            .count()
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}

impl Point {
    fn parse(input: &str) -> Self {
        let values: Vec<usize> = input
            .split(',')
            .map(|segment| segment.parse().expect("a number"))
            .collect();

        Self {
            x: values[0],
            y: values[1],
            z: values[2],
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
struct Block {
    from: Point,
    to: Point,
}

impl Block {
    fn parse(input: &str) -> Self {
        let (from, to) = input.split_once('~').expect("to have a tilde");

        Self {
            from: Point::parse(from),
            to: Point::parse(to),
        }
    }

    fn drop(&mut self, height: usize) {
        self.from.z -= height;
        self.to.z -= height;
    }

    fn z_size(&self) -> usize {
        self.to.z - self.from.z + 1
    }

    fn is_supporting(&self, supported: &Block) -> bool {
        self.to.z + 1 == supported.from.z
            && self.from.x <= supported.to.x
            && self.to.x >= supported.from.x
            && self.from.y <= supported.to.y
            && self.to.y >= supported.from.y
    }
}

#[derive(Copy, Clone)]
struct Size {
    x: usize,
    y: usize,
}

struct HeightMap {
    size: Size,
    map: Vec<usize>,
}

impl HeightMap {
    fn from(world: &World) -> Self {
        let size = world
            .blocks
            .iter()
            .fold(Size { x: 0, y: 0 }, |acc, block| Size {
                x: acc.x.max(block.to.x + 1),
                y: acc.y.max(block.to.y + 1),
            });

        Self {
            size,
            map: (0..(size.x * size.y)).map(|_| 0).collect(),
        }
    }

    fn height_at_point(&self, x: usize, y: usize) -> usize {
        self.map[y * self.size.x + x]
    }

    fn height_at_block(&self, block: &Block) -> usize {
        (block.from.y..=block.to.y).fold(0, |acc, y| {
            acc.max(
                (block.from.x..=block.to.x).fold(0, |acc, x| self.height_at_point(x, y).max(acc)),
            )
        })
    }

    fn set_height_at_point(&mut self, x: usize, y: usize, height: usize) {
        self.map[y * self.size.x + x] = height
    }

    fn set_height_at_block(&mut self, block: &Block, height: usize) {
        (block.from.y..=block.to.y).for_each(|y| {
            (block.from.x..=block.to.x).for_each(|x| self.set_height_at_point(x, y, height))
        })
    }

    fn drop(&mut self, block: &mut Block) -> bool {
        let current_height = self.height_at_block(block);
        let new_height = current_height + block.z_size();
        self.set_height_at_block(block, new_height);

        let dropping = block.from.z - current_height - 1;

        block.drop(dropping);

        dropping != 0
    }
}
