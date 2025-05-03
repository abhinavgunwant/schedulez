use chrono::{DateTime, Datelike, Local};

use crate::reader::ScheduledElement;

use std::collections::HashMap;
use rand::{ rng, seq::SliceRandom };

/// Whether scheduling this element should be avoided.
fn avoid(elem: &ScheduledElement, index: u32) -> bool {
    elem.avoid_days.iter()
        .filter(|x| x.to_u32() == index)
        .count() > 0
}

/// Gets the index of the element with most debt that cannot be avoided for the
/// given day.
///
/// Negative debt need not be considered here.
fn max_debt_index(
    elems: &Vec<ScheduledElement>,
    debts: &Vec<i8>,
    day_of_week: u32
) -> Option<usize> {
    let mut max_debt: i8 = 0;
    let mut max_debt_index = 0;
    let mut max_debt_found = false;

    // Try finding the max debt
    for (i, debt) in debts.iter().enumerate() {
        if *debt == 0 {
            continue;
        }

        if *debt > max_debt && !avoid(&elems[i], day_of_week) {
            max_debt = *debt;
            max_debt_index = i;
            max_debt_found = true;
        }
    }

    if !max_debt_found {
        return None;
    }

    // if found, return
    Some(max_debt_index)
}

/// Processes the scheduling for the month.
///
/// Params:
/// - `elements` The elements to schedule
/// - `month` The month of schedule
/// - `win_len` The "Window Length"
pub fn process(
    elements: &Vec<ScheduledElement>,
    month: DateTime<Local>,
    win_len: usize,
) -> Vec<Vec<Vec<String>>> {
    let mut month_vec: Vec<Vec<Vec<String>>> = Vec::with_capacity(5);

    let mut elem_random = elements.clone();

    elem_random.shuffle(&mut rng());

    let m: u32 = month.month();

    println!("current month is: {0}", m);

    let first_day = month.with_day(1).unwrap();

    let start_day_index = first_day.weekday().number_from_sunday() - 1;

    println!("Start day index: {0}", start_day_index);

    // The number of days that have been scheduled
    let mut total_day_count: u8 = 0;
    let mut window_offset: u32 = 0;

    let mut debts: Vec<i8> = Vec::with_capacity(elem_random.len());

    for _ in 0..elem_random.len() {
        debts.push(0);
    }

    for i in 0..5 {
        let mut week_vec: Vec<Vec<String>> = Vec::with_capacity(7);

        for j in 0..7 {
            if i == 0 && j < start_day_index {
                println!("<1> skipped: i={0}, j={1}", i, j);
                week_vec.push(Vec::<String>::new());
                continue;
            }

            if j == 0 || j == 6 {
                println!("skipped: i={0}, j={1}", i, j);
                total_day_count += 1;
                week_vec.push(Vec::<String>::new());
                continue;
            }

            let mut day_vec: Vec<String>;

            if elem_random.len() < win_len {
                day_vec = elem_random.iter()
                    .filter(|elem| avoid(elem, j))
                    .map(|elem| elem.text.clone())
                    .collect();
            } else {
                day_vec = Vec::with_capacity(elem_random.len());
                let mut index: usize = 0;

                // while day_vec.len() < win_len && !flag && index != window_offset {
                while index < win_len {
                    let abs_index = (index + window_offset as usize) % elem_random.len();

                    if avoid(&elem_random[abs_index], j) {
                        debts[abs_index] += 1;
                    } else {
                        day_vec.push(elem_random[abs_index].text.clone());
                    }

                    index = index + 1;
                }

                while day_vec.len() < win_len {
                    match max_debt_index(&elem_random, &debts, j) {
                        Some(max_debt) => {
                            day_vec.push(elem_random[max_debt].text.clone());
                            debts[max_debt] -= 1;
                        }

                        None => { break; }
                    }
                }

                while day_vec.len() < win_len
                    && index != window_offset as usize
                {
                    let relative_index = (index + window_offset as usize) % elem_random.len();

                    if !avoid(&elem_random[relative_index], j) {
                        day_vec.push(elem_random[relative_index].text.clone());
                        debts[relative_index] -= 1;
                    }

                    index = index + 1;
                }

                window_offset = (window_offset + 1) % win_len as u32;
            }

            week_vec.push(day_vec);
            total_day_count += 1;

            if total_day_count == month.num_days_in_month() {
                break;
            }
        }

        month_vec.push(week_vec);

        if total_day_count == month.num_days_in_month() {
            if month_vec.len() == 4 {
                month_vec.push(Vec::<Vec<String>>::new());
            }

            break;
        }

        println!("debts: {:?}", debts);
    }

    month_vec
}

