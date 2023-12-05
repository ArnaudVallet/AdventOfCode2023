use crate::row::Row;

#[derive(Debug, Clone)]
pub struct Gear {
    row_index: u8,
    index: u8,
    overlaps: Vec<i32>,
}

impl Gear {
    pub fn new(row_index: u8, index: u8) -> Gear {
        Gear { row_index, index, overlaps: vec![] }
    }

    pub fn get_row_index(&self) -> u8 {
        self.row_index
    }

    pub fn get_overlaps(self) -> Vec<i32> {
        self.overlaps
    }

    pub fn is_engine(&self) -> bool {
        self.overlaps.len() > 1
    }

    pub fn find_gear_adjacent_numbers_from(&mut self, prev_row: &Row, actual_row: &Row, next_row: &Row) {
        let gear_range = (self.index -1, self.index +1);

        // Iterate previous Row Numbers
        for number in prev_row.clone().get_numbers() {
            let number_range = (number.get_start_index(), number.get_end_index() -1); // -1 cause uninclusive
            let ranges_overlap = Gear::ranges_overlap(gear_range.0, gear_range.1, number_range.0, number_range.1);
            if ranges_overlap {
                self.overlaps.push(number.get_value().parse::<i32>().unwrap());
            } 
        }

        // Iterate actual Row Numbers
        for number in actual_row.clone().get_numbers() {
            let number_range = (number.get_start_index(), number.get_end_index() -1); // -1 cause uninclusive
            let ranges_overlap = Gear::ranges_overlap(gear_range.0, gear_range.1, number_range.0, number_range.1);
            if ranges_overlap {
                self.overlaps.push(number.get_value().parse::<i32>().unwrap());
            } 
        }

        // Iterate next Row Numbers
        for number in next_row.clone().get_numbers() {
            let number_range = (number.get_start_index(), number.get_end_index() -1); // -1 cause uninclusive
            let ranges_overlap = Gear::ranges_overlap(gear_range.0, gear_range.1, number_range.0, number_range.1);
            if ranges_overlap {
                self.overlaps.push(number.get_value().parse::<i32>().unwrap());
            } 
        }
    }

    pub(crate) fn ranges_overlap<T>(start1: T, end1: T, start2: T, end2: T) -> bool where T: PartialOrd {
        if end1 < start2 || end2 < start1 {
            false
        } else {
            true
        }
    }

}
