// 0.003s on A4-5000 (1.5GHz)
fn main() {
    let present_measurements = include_str!("input.txt").lines()
        .map(|x| Present::new(x)).collect();
    println!("Wrapping Paper Needed: {}", wrapping_paper_needed(&present_measurements));
    println!("Ribbon Length Needed:  {}", ribbon_length_needed(&present_measurements))
}

/// Contains measurements for presents: {length, width, height}.
struct Present { length: usize, width: usize, height: usize }
impl Present {
    /// Creates a new `Present` containing the present's measurements.
    fn new(input: &str) -> Present {
        let item: Vec<&str> = input.split('x').collect();
        Present {
            length: item[0].parse().unwrap(),
            width:  item[1].parse().unwrap(),
            height: item[2].parse().unwrap(),
        }
    }

    /// Calculates the amount of wrapping paper is needed for a present.
    fn required_wrapping_paper(&self) -> usize {
        let sides: [usize; 3] = [
            (self.length * self.width),
            (self.width * self.height),
            (self.height * self.length)
        ];
        let (slack, wrapping_area) = (sides.iter().min().unwrap(), 2*(sides[0]+sides[1]+sides[2]));
        wrapping_area + slack
    }

    /// Calculates the amount of ribbon is needed for a present.
    fn required_ribbon_length(&self) -> usize {
        let mut sides: [usize; 3] = [self.length, self.width, self.height];
        sides.sort();
        let (wrapping_len, ribbon) = (2*(sides[0]+sides[1]), self.length*self.width*self.height);
        wrapping_len + ribbon
    }
}

/// Calculates the total amount of wrapping paper needed for the presents.
fn wrapping_paper_needed(input: &Vec<Present>) -> usize {
    input.iter().fold(0, |sum, next| { sum + next.required_wrapping_paper() })
}

/// Calculates the total amount of ribbon needed for the presents.
fn ribbon_length_needed(input: &Vec<Present>) -> usize {
    input.iter().fold(0, |sum, next| { sum + next.required_ribbon_length() })
}
