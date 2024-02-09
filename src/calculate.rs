use std::collections::HashMap;

pub fn calculate_curve_value(curve: &HashMap<i32, i32>, input: i32) -> i32 {
    let mut smallest = i32::MAX;
    let mut largest = i32::MIN;
    let mut closest_smaller = i32::MIN;
    let mut closest_larger = i32::MAX;

    for (&key, _) in curve.iter() {
        if key < smallest {
            smallest = key;
        }
        if key > largest {
            largest = key;
        }
        if input >= key && key > closest_smaller {
            closest_smaller = key
        }
        if input <= key && key < closest_larger {
            closest_larger = key
        }
    }

    if input < smallest {
        return *curve.get(&smallest).unwrap_or(&0);
    }
    if input > largest {
        return *curve.get(&largest).unwrap_or(&100);
    }

    // println!("Input {}, smallest {}, largest {}, closest smaller {}, closest larget {}", input, smallest, largest, closest_smaller, closest_larget);
    if closest_larger == closest_smaller {
        return *curve.get(&closest_larger).unwrap_or(&100);
    } else {
        let x0 = closest_smaller;
        let x1 = closest_larger;
        let y0 = *curve.get(&x0).unwrap_or(&0);
        let y1 = *curve.get(&x1).unwrap_or(&0);
        return y0 + ((input - x0) * (y1 - y0)) / (x1 - x0);
    }
}
