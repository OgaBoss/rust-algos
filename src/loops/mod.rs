// https://www.enjoyalgorithms.com/crack-coding-interviews/
// Find leaders in Array
pub fn find_leaders(array: Vec<i8>) -> Vec<i8> {
    let mut leaders: Vec<i8> = Vec::new();

    for (index, el) in array.iter().enumerate() {
        let mut found_leader = true;

        let start_nest_loop_index = index + 1;

        for i in &array[start_nest_loop_index..] {
            if el < i && el != i {
                found_leader = false
            }
        }

        if found_leader {
            leaders.push(*el)
        }

    }

    return leaders;
}

// https://www.enjoyalgorithms.com/crack-coding-interviews/
// Building facing the sun
pub fn facing_sun(array: Vec<i32>) -> i32 {
    let mut houses = 1;

    let mut cur_max_height = array[0];

    for h in array {
        if cur_max_height < h {
            houses += 1;

            cur_max_height = h
        }
    }

    return houses
}

// https://www.enjoyalgorithms.com/crack-coding-interviews/
// Valid Mountain Array
pub fn is_mountain_array(array: Vec<i32>)-> bool {
    let mut curr_left_value = array[0];

    let mut curr_right_value = array[array.len() - 1];

    let mut is_mountain_value = true;

    let array_range = &array[1..(array.len()-1)];

    let mut left_index = 0;

    let mut right_index = array_range.len() - 1;

    for h in array_range {
        if left_index == right_index  {
            if h < &curr_right_value || h < &curr_left_value {
                is_mountain_value = false;
            }

            break;
        }

        if curr_left_value > *h || curr_right_value > array_range[right_index] {
            is_mountain_value = false;
            break;
        }

        curr_left_value = *h;
        curr_right_value = array_range[right_index];

        left_index = left_index + 1;
        right_index = right_index - 1;
    }

    return is_mountain_value;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_find_leaders() {
        let array = vec![15,17,4,3,5,2];

        let result = find_leaders(array);

        assert_eq!(result, vec![17,5,2]);
    }

    #[test]
    fn count_buildings_facing_sun() {
        let array = vec![7,2,3,4,5,9];

        let result =  facing_sun(array);

        assert_eq!(result, 2)
    }

    #[test]
    fn valid_mountain_array() {
        let array = vec![5,2,1,4];

        let result =  is_mountain_array(array);

        assert_eq!(result, false)
    }
}