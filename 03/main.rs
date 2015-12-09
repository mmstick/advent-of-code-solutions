// 0.027s on A4-5000 (1.5GHz)
fn main() {
    let mut iterator = include_str!("input.txt").chars();
    let mut houses = vec![Position{x:0, y:0}];
    let (mut santa_position, mut robot_position) = (Position{x:0, y:0}, Position{x:0, y:0});
    let mut direction; loop {
        direction = match iterator.next() { Some(x) => x, None => break };
        santa_position.give_direction(&direction);
        if !houses.contains(&santa_position) { houses.push(santa_position); }
        direction = match iterator.next() { Some(x) => x, None => break };
        robot_position.give_direction(&direction);
        if !houses.contains(&robot_position) { houses.push(robot_position); }
    }
    println!("Houses that obtained at least one present: {}", houses.len());
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Position { x: isize, y: isize }
impl Position {
    /// Updates the position of santa by matching the direction.
    fn give_direction(&mut self, direction: &char) {
        match *direction {
            '^' => self.y += 1,
            'v' => self.y -= 1,
            '<' => self.x -= 1,
            '>' => self.x += 1,
            _ => (),
        }
    }
}
