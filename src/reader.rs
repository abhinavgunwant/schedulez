use std::path::Path;

use calamine::{ Reader as CalamineReader, open_workbook, Xlsx, Data };

use crate::types::Day;

#[derive(Debug, Default, Clone)]
pub struct ScheduledElement {
    pub text: String,
    pub prefer_days: Vec<Day>,
    pub avoid_days: Vec<Day>,
}

#[derive(Debug, Default, Clone)]
pub struct Reader {
    /// Name of the elements that are being schedules
    ///
    /// On the excel, it's represented by the value next to the
    /// "Scheduled Elements" cell.
    pub elements_name: String,
    pub elements: Vec<ScheduledElement>,
}

impl Reader {
    pub fn read(&mut self, file_path: &Path) {
        println!("Reading workbook {}", file_path.to_str().unwrap());

        let mut workbook: Xlsx<_> = open_workbook(file_path)
            .expect("cannot open file!");

        println!("Reading workbook {}", file_path.to_str().unwrap());

        for (sheet, range) in workbook.worksheets().iter() {
            println!("Reading sheet {}", sheet);

            if !range.is_empty() && range.used_cells().count() > 0 {
                let mut data_found = false;
                let mut row_index: u32 = 0;

                // for cell in range.cells() {
                //     println!("{},{}: {}", cell.0, cell.1, cell.2);
                // }
                let mut name_col_found;
                let mut prefer_col_found;

                for (i,row) in range.rows().enumerate() {
                    if row.is_empty() {
                        continue;
                    }

                    // If it's the header row, regardless if it's on top or
                    // not, ignore this row
                    name_col_found = false;
                    prefer_col_found = false;

                    if let Data::String(first_col) = row[0].clone() {
                        if first_col.to_uppercase() == "NAME" {
                            name_col_found = true;
                        }
                    }

                    if name_col_found {
                        if let Data::String(first_col) = row[1].clone() {
                            if first_col.to_uppercase() == "PREFER DAYS"
                                || first_col.to_uppercase() == "PREFER_DAYS"
                                || first_col.to_uppercase() == "PREFER" {
                                prefer_col_found = true;
                            }
                        }
                    }

                    if prefer_col_found {
                        if let Data::String(first_col) = row[2].clone() {
                            if first_col.to_uppercase() == "AVOID DAYS"
                                || first_col.to_uppercase() == "AVOID_DAYS"
                                || first_col.to_uppercase() == "AVOID" {
                                continue;
                            }
                        }
                    }

                    let mut element = ScheduledElement::default();

                    // Actual data insertion takes place here:
                    for (j,cell) in row.iter().enumerate() {
                        match cell {
                            calamine::Data::String(data) => {
                                if j == 0 {
                                    element.text = data.clone();
                                }

                                if j == 1 {
                                    element.prefer_days = data.split(",")
                                        .map(|s| Day::from(s.trim()))
                                        .collect();
                                }

                                if j == 2 {
                                    element.avoid_days = data.split(",")
                                        .map(|s| Day::from(s.trim()))
                                        .collect();
                                }
                            }

                            _ => {}
                        }
                    }

                    println!("{:?}", element);

                    self.elements.push(element);
                }
            }

            break;
        }
    }
}

