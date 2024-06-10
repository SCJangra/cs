use std::cmp::Ordering;

pub fn quick_sort<T: Ord>(arr: &mut [T]) {
    match arr.len() {
        0 | 1 => return,
        _ => { /* Continue */ }
    }

    let len = arr.len();

    let i = pivot(arr, 0, len - 1);

    quick_sort(&mut arr[0..i]);
    quick_sort(&mut arr[i + 1..len]);
}

#[derive(Clone, Copy)]
enum Position {
    Left,
    Right,
}

fn pivot<T: Ord>(arr: &mut [T], start: usize, end: usize) -> usize {
    let mut left = start;
    let mut right = end;
    let mut pos = Position::Left;

    loop {
        if left >= right {
            break;
        }

        let cmp = arr[left].cmp(&arr[right]);

        match (pos, cmp) {
            (Position::Left, Ordering::Less | Ordering::Equal) => right -= 1,
            (Position::Left, Ordering::Greater) => {
                arr.swap(left, right);
                pos = Position::Right;
                left += 1;
            }
            (Position::Right, Ordering::Less | Ordering::Equal) => left += 1,
            (Position::Right, Ordering::Greater) => {
                arr.swap(left, right);
                pos = Position::Left;
                right -= 1;
            }
        }
    }

    left
}
