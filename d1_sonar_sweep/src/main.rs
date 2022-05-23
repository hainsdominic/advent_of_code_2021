fn ascending(pair: &(i32, i32)) -> bool {
    pair.0 <= pair.1
}

fn sweep(levels: &Vec<i32>) -> Vec<bool> {
    let mut result = vec![false];
    for level in levels.iter().enumerate() {
        let second = levels.get(level.0 + 1);
        if let Some(x) = second {
            result.push(ascending(&(*level.1, *x)))
        }
    }
    return result;
}

fn display(levels: &Vec<i32>, slopes: &Vec<bool>) {
    println!("{} (N/A - no previous measurement)", levels.get(0).unwrap());
    for slope in slopes.iter().enumerate().skip(1) {
        match slope.1 {
            true => println!("{} (increased)", levels.get(slope.0).unwrap()),
            false => println!("{} (decreased)", levels.get(slope.0).unwrap()),
        }
    }
}

fn main() {
    let levels = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    let slopes = sweep(&levels);
    display(&levels, &slopes);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ascending_works_1() {
        let pair = (1, 2);
        let result = ascending(&pair);
        assert_eq!(result, true);
    }

    #[test]
    fn ascending_works_2() {
        let pair = (2, 1);
        let result = ascending(&pair);
        assert_eq!(result, false);
    }

    #[test]
    fn sweep_works_1() {
        let levels = vec![1, 2, 3];
        let result = sweep(&levels);
        assert_eq!(result, vec![false, true, true])
    }
}
