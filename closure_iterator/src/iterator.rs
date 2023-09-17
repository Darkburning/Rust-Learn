#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

#[allow(unused)]
fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // into_iter方法会消耗集合并返回拥有所有权的迭代器，不再允许访问原始集合
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

struct Counter {
    count: u32,
}

impl Counter {
    #[allow(unused)]
    fn new() -> Self {
        Self { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iterator_next() {
        let v = vec![1, 2, 3, 4];

        let mut v_iter = v.iter();

        assert_eq!(v_iter.next(), Some(&1));
        assert_eq!(v_iter.next(), Some(&2));
        assert_eq!(v_iter.next(), Some(&3));
        assert_eq!(v_iter.next(), Some(&4));
        assert_eq!(v_iter.next(), None);
    }

    #[test]
    fn test_iterator_sum() {
        let v = vec![1, 2, 3, 4];

        let v_iter_sum: i32 = v.iter().sum();

        assert_eq!(v_iter_sum, 10)
    }

    #[test]
    fn test_iterator_map() {
        let v = vec![1, 2, 3, 4];

        let v_iter_map: Vec<_> = v.iter().map(|f| f + 1).collect();

        assert_eq!(v_iter_map, vec![2, 3, 4, 5]);
    }

    #[test]
    fn test_filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_my_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }

    #[test]
    fn test_count_iterator() {
        let mut counter = Counter::new();

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }

    #[test]
    fn using_other_iterator_trait_methods() {
        let sum1: u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();
        let sum2: u32 = Counter::new()
            .zip(Counter::new().skip(2))
            .map(|(a, b)| a * a + b * b)
            .filter(|x| x % 2 == 0)
            .sum();
        assert_eq!(18, sum1);
        assert_eq!(64, sum2);
    }
}
