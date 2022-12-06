#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ArgMax {
    pub index: usize,
    pub value: i32,
}

impl ArgMax {
    fn new(index: usize, value: i32) -> Self {
        Self { index, value }
    }
}

/// returns the first index-value-pair of the list-item with the highest value
pub fn argmax(input: Vec<i32>) -> ArgMax {
    input
        .iter()
        .enumerate()
        .fold(ArgMax::new(0, i32::MIN), |max, (ind, &val)| {
            if val > max.value {
                return ArgMax::new(ind, val);
            }
            max
        })
}

pub fn arg3max(input: Vec<i32>) -> Vec<ArgMax> {
    let init = vec![
        ArgMax::new(0, i32::MIN),
        ArgMax::new(0, i32::MIN),
        ArgMax::new(0, i32::MIN),
    ];
    input.iter().enumerate().fold(init, |max, (ind, &val)| {
        if max[0].value < val {
            return vec![ArgMax::new(ind, val), max[0], max[1]];
        }
        if max[1].value < val {
            return vec![max[0], ArgMax::new(ind, val), max[1]];
        }
        if max[2].value < val {
            return vec![max[0], max[1], ArgMax::new(ind, val)];
        }
        max
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_argmax() {
        assert_eq!(argmax(vec![1, 5, 2, 4, 3]), ArgMax::new(1, 5));
        assert_eq!(argmax(vec![1, 5, 5, 4, 3]), ArgMax::new(1, 5));
    }

    #[test]
    fn test_arg3max() {
        assert_eq!(
            arg3max(vec![1, 5, 2, 4, 3]),
            vec![ArgMax::new(1, 5), ArgMax::new(3, 4), ArgMax::new(4, 3)]
        );
        assert_eq!(
            arg3max(vec![1, 5, 5, 4, 3]),
            vec![ArgMax::new(1, 5), ArgMax::new(2, 5), ArgMax::new(3, 4)]
        );
    }
}
