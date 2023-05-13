use std::collections::BTreeSet;

pub struct Pyramid {
    nodes: Vec<(u64, usize)>,
}

impl Pyramid {
    /// Create a list with the first index of a list containing a pyramid
    pub fn levels_of_a_pyramid(len: usize) -> Result<Vec<usize>, String> {
        if len.eq(&0) {
            return Ok(vec![]);
        };
        let levels: BTreeSet<_> = (0..=(len))
            .scan(0, |l, cur| {
                *l += cur;
                Some(*l)
            })
            .collect();
        if levels.contains(&(len)) {
            Ok(levels.into_iter().filter(|n| n.lt(&len)).collect())
        } else {
            Err("Not a pyramid.".to_owned())
        }
    }

    /// Get how many levels there is on the pyramid
    pub fn levels(&self) -> u64 {
        Self::levels_of_a_pyramid(self.nodes.len()).unwrap().len() as u64
    }

    /// Find the biggest sum of a values in a pyramid.
    pub fn biggest_sum(&self) -> u64 {
        let mut copy = self.nodes.clone();
        self.nodes
            .iter()
            .enumerate()
            .rev()
            .map(|(i, d)| {
                let left_child = d.1;
                let left = d.0 + copy.get(left_child).map_or(0, |n| n.0);
                let right = d.0 + copy.get(left_child.saturating_add(1)).map_or(0, |n| n.0);
                let max = left.max(right);
                copy[i] = (max, d.1);
                max
            })
            .last()
            .unwrap()
    }
}

impl TryFrom<&[u64]> for Pyramid {
    type Error = String;
    fn try_from(value: &[u64]) -> Result<Self, Self::Error> {
        let mut levels = Self::levels_of_a_pyramid(value.len())?
            .into_iter()
            .peekable();
        let nodes = value
            .iter()
            .enumerate()
            .scan(0_usize, |off, (i, v)| {
                if levels.peek().map_or(false, |l| i.eq(l)) {
                    levels.next();
                    *off = 0;
                }

                let ooff = if let Some(nlevel) = levels.peek() {
                    nlevel + *off
                } else {
                    usize::MAX
                };
                *off += 1;
                Some((*v, ooff))
            })
            .collect();

        Ok(Self { nodes })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn pyramid_levels_test() {
        assert_eq!(Pyramid::levels_of_a_pyramid(0), Ok(vec![]));
        assert_eq!(Pyramid::levels_of_a_pyramid(1), Ok(vec![0]));
        assert_eq!(
            Pyramid::levels_of_a_pyramid(2),
            Err("Not a pyramid.".to_string())
        );
        assert_eq!(Pyramid::levels_of_a_pyramid(3), Ok(vec![0, 1]));
    }
}
