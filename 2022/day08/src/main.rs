use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("inputs/day8/day8.txt").unwrap();
    let input: Vec<_> = input.lines().map(|str| str.as_bytes().to_vec()).collect();
    let mut scanner = Scanner::default();

    // Puzzle 1
    scanner.scan_input(input);
    println!("Puzzle 1: {}", scanner.set.len())
}

#[derive(Default)]
struct Scanner {
    pub set: HashSet<(usize, usize)>,
}

#[derive(Copy, Clone)]
enum AreaId {
    Row(usize),
    Col(usize),
}

impl Scanner {
    pub fn scan_input(&mut self, input: Vec<Vec<u8>>) {
        let rows = input.iter();
        let row_len = input[0].len();

        let col = |id| input.iter().map(move |row| row[id]);
        let cols = (0..row_len).into_iter().map(col);

        for (area_id, row) in rows.enumerate() {
            self.scan(row, AreaId::Row(area_id))
        }

        for (area_id, col) in cols.enumerate() {
            let col: Vec<_> = col.collect();
            self.scan(&col, AreaId::Col(area_id));
        }
    }

    fn insert_visible(&mut self, area_id: AreaId, id: usize) {
        match area_id {
            AreaId::Row(row_id) => self.set.insert((row_id, id)),
            AreaId::Col(col_id) => self.set.insert((id, col_id)),
        };
    }

    fn scan(&mut self, area: &[u8], area_id: AreaId) {
        let mut highest = b'/'; // 47 in ASCII which one less than 0 (lowest possible tree length)
        for (id, &tree) in area.iter().enumerate() {
            if tree > highest {
                self.insert_visible(area_id, id);
                highest = tree;
            }
        }

        let mut highest = b'/';
        for (id, &tree) in area.iter().enumerate().rev() {
            if tree > highest {
                self.insert_visible(area_id, id);
                highest = tree;
            }
        }
    }
}
