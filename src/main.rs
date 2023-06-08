#[derive(Debug, PartialEq)]
pub struct Rover {
    x: i32,
    y: i32,
    position: Coordinate,
}

#[derive(Debug, PartialEq)]
pub enum Coordinate {
    N,
    E,
    S,
    W,
}

#[derive(Debug)]
enum Rotate {
    L,
    R,
}

impl From<char> for Rotate {
    fn from(value: char) -> Self {
        match value {
            'L' => Rotate::L,
            'R' => Rotate::R,
            _ => panic!("Incorrect rotation"),
        }
    }
}

impl From<char> for Coordinate {
    fn from(value: char) -> Self {
        if !matches!(value, 'N' | 'E' | 'S' | 'W') {
            panic!("Incorrect Coordinate")
        } else {
            match value {
                'N' => Self::N,
                'E' => Self::E,
                'S' => Self::S,
                _ => Self::W,
            }
        }
    }
}

impl Rover {
    fn compute_location(command: String) -> Vec<Rover> {
        let mut vec_rovers: Vec<Rover> = Vec::new();

        for _ in 0..(command.lines().count() - 1) / 2 {
            vec_rovers.push(Rover {
                x: 0,
                y: 0,
                position: Coordinate::N,
            });
        }

        for (num, rover) in vec_rovers.iter_mut().enumerate() {
            let (boundary, position, movement) = Self::split_command(command.clone(), num);
            rover.set_initial_position(&position);

            for char in movement.iter() {
                if matches!(char, 'L' | 'R') {
                    rover.rotate(*char);
                } else if matches!(char, 'M') {
                    rover.transpose((
                        boundary.first().unwrap().to_digit(10).unwrap() as i32,
                        boundary.get(1).unwrap().to_digit(10).unwrap() as i32,
                    ));
                }
            }

            println!("{}: {:?}", num + 1, rover);
        }
        vec_rovers
    }

    fn split_command(position: String, num: usize) -> (Vec<char>, Vec<char>, Vec<char>) {
        let mut reader = position.split('\n');
        let boundary: Vec<char> = reader
            .next()
            .expect("Positional Argument not formulated correctly")
            .chars()
            .filter(|chars| chars.is_numeric())
            .collect();

        let position: Vec<char> = reader
            .nth(num * 2)
            .expect("Positional argument not formulated correctly")
            .chars()
            .filter(|chars| chars.is_alphabetic() || chars.is_numeric())
            .collect();

        let movement: Vec<char> = reader
            .next()
            .expect("Positional argument not formulated correctly")
            .chars()
            .filter(|chars| chars.is_alphabetic())
            .collect();

        (boundary, position, movement)
    }

    fn set_initial_position(&mut self, position: &[char]) -> &mut Self {
        self.x = position
            .first()
            .and_then(|char| char.to_digit(10))
            .expect("Could not allocate string to int for rover.x") as i32;

        self.y = position
            .get(1)
            .and_then(|char| char.to_digit(10))
            .expect("Could not allocate string to int for rover.y") as i32;

        self.position = std::convert::Into::<Coordinate>::into(*position.get(2).unwrap());

        self
    }

    fn rotate(&mut self, char: char) -> &mut Self {
        match char.into() {
            Rotate::L => match self.position {
                Coordinate::N => self.position = Coordinate::W,
                Coordinate::E => self.position = Coordinate::N,
                Coordinate::S => self.position = Coordinate::E,
                Coordinate::W => self.position = Coordinate::S,
            },
            Rotate::R => match self.position {
                Coordinate::N => self.position = Coordinate::E,
                Coordinate::E => self.position = Coordinate::S,
                Coordinate::S => self.position = Coordinate::W,
                Coordinate::W => self.position = Coordinate::N,
            },
        }
        self
    }

    fn transpose(&mut self, plane_boundary: (i32, i32)) -> &mut Self {
        match self.position {
            Coordinate::N => self.y += 1,
            Coordinate::E => self.x += 1,
            Coordinate::S => self.y -= 1,
            Coordinate::W => self.x -= 1,
        }

        self.x = self.x.clamp(0, plane_boundary.0);
        self.y = self.y.clamp(0, plane_boundary.1);
        self
    }
}

fn main() {
    let arg = std::env::args().nth(1).unwrap();

    arg.as_str().eq("").then(|| panic!("No arguments given"));

    Rover::compute_location(arg);
}

#[cfg(test)]
mod tests {
    // Import the function to be tested
    use crate::Rover;

    #[test]
    fn check_first() {
        let lhs = Rover::compute_location(String::from(
            "5 5 
        1 2 N 
        LMLMLMLMM",
        ));

        let rhs = Rover {
            x: 1,
            y: 3,
            position: crate::Coordinate::N,
        };

        // Perform assertions to validate the expected behavior
        assert_eq!(lhs.first().unwrap(), &rhs);
    }

    #[test]
    fn check_second() {
        let lhs = Rover::compute_location(String::from(
            "5 5 
            3 3 E 
            MMRMMRMRRM",
        ));

        let rhs = Rover {
            x: 5,
            y: 1,
            position: crate::Coordinate::E,
        };

        // Perform assertions to validate the expected behavior
        assert_eq!(lhs.first().unwrap(), &rhs);
    }

    #[test]
    fn check_combined() {
        let lhs = Rover::compute_location(String::from(
            "5 5 
            1 2 N 
            LMLMLMLMM 
            3 3 E 
            MMRMMRMRRM",
        ));

        let rhs_1 = Rover {
            x: 1,
            y: 3,
            position: crate::Coordinate::N,
        };
        let rhs_2 = Rover {
            x: 5,
            y: 1,
            position: crate::Coordinate::E,
        };

        // Perform assertions to validate the expected behavior
        assert_eq!(lhs.first().unwrap(), &rhs_1);
        assert_eq!(lhs.get(1).unwrap(), &rhs_2);
    }
}
