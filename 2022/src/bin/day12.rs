use arrayvec::ArrayVec;
use input_extractor::get::{get_input, AoCDate};
use pathfinding::prelude::bfs;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize);

#[derive(Debug, Clone)]
struct Graph {
    rows: usize,
    columns: usize,
    data: Vec<char>,
}

impl Graph {
    pub fn pos_of_character(&self, character: char) -> Pos {
        let index = self.data.iter().position(|&r| r == character).unwrap();
        self.get_pos_from_index(index)
    }
    pub fn get_pos_from_index(&self, index: usize) -> Pos {
        let j = index % self.columns;
        let i = (index - j) / self.columns;
        Pos(i, j)
    }
    pub fn get_character_value_from_pos(&self, position: &Pos) -> i32 {
        let index = position.0 * self.columns + position.1;
        let item = self.data[index];
        match item {
            'a'..='z' => item as i32 - 'a' as i32 + 1,
            'S' => 100,
            'E' => 'z' as i32 - 'a' as i32 + 1,
            item => unreachable!("Unexpected item: {}", item),
        }
    }

    pub fn get_possible_neighbors(&self, position: &Pos) -> ArrayVec<Pos, 4> {
        let max_index = self.rows * self.columns;
        let mut possible_neighbours = ArrayVec::<Pos, 4>::new();

        if position.0 < self.rows
            && (0..max_index).contains(&((position.0 + 1) * self.columns + position.1))
        {
            possible_neighbours.push(Pos(position.0 + 1, position.1));
        }
        if position.0 > 0
            && (0..max_index).contains(&((position.0 - 1) * self.columns + position.1))
        {
            possible_neighbours.push(Pos(position.0 - 1, position.1));
        }
        if position.1 > 0 && (0..max_index).contains(&(position.0 * self.columns + position.1 - 1))
        {
            possible_neighbours.push(Pos(position.0, position.1 - 1));
        }
        if position.1 < self.columns
            && (0..max_index).contains(&(position.0 * self.columns + position.1 + 1))
        {
            possible_neighbours.push(Pos(position.0, position.1 + 1));
        }
        possible_neighbours
    }
}

impl Pos {
    fn successors(&self, walk_graph: &Graph) -> ArrayVec<Pos, 4> {
        let &Pos(x, y) = self;
        let mut next_positions = ArrayVec::<Pos, 4>::new();

        let value_char_at_pos = walk_graph.get_character_value_from_pos(&Pos(x, y));

        let possible_neighbours = walk_graph.get_possible_neighbors(&Pos(x, y));

        for next_pos in possible_neighbours.iter() {
            let value_char_next = walk_graph.get_character_value_from_pos(next_pos);
            if value_char_next < value_char_at_pos + 2 {
                next_positions.push(next_pos.clone());
            }
        }
        next_positions
    }
}

fn main() {
    let content = get_input(AoCDate {
        year: 2022,
        day: 12,
    });
    let columns: usize = content
        .lines()
        .next()
        .unwrap()
        .chars()
        .collect::<Vec<_>>()
        .len();

    let rows: usize = content.lines().collect::<Vec<_>>().len();

    let mut walk_graph = Graph {
        rows,
        columns,
        data: Vec::with_capacity(rows * columns),
    };
    for line in content.lines() {
        let mut row_array = line.chars().collect();
        walk_graph.data.append(&mut row_array);
    }
    let start = walk_graph.pos_of_character('S');
    let end = walk_graph.pos_of_character('E');
    let result = bfs(&start, |p| p.successors(&walk_graph), |p| *p == end)
        .expect("no path found")
        .len()
        - 1;
    println!("Shortest path {}", result);

    let a_indexes: Vec<_> = walk_graph
        .data
        .iter()
        .enumerate()
        .filter_map(|(ind, &x)| if x == 'a' { Some(ind) } else { None })
        .collect();

    let mut result_part2 = Vec::new();
    for index in a_indexes.iter() {
        let start = walk_graph.get_pos_from_index(*index);
        let path_bfs = bfs(&start, |p| p.successors(&walk_graph), |p| *p == end);
        if let Some(path) = path_bfs { result_part2.push(path.len() - 1) }
    }
    let shortest_path_part2: i32 = *result_part2.iter().min().unwrap() as i32;
    println!("Shortest path from a {}", shortest_path_part2);
}
