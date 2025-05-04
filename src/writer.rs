use crate::types::{ FileExt, Day, Month };

use chrono::{ DateTime, Datelike, Local };
use rust_xlsxwriter::*;
use native_dialog::DialogBuilder;

#[derive(Debug, Default, Clone)]
pub struct Writer {
    month: DateTime<Local>,
    month_schedule: Vec<Vec<Vec<String>>>,
    elem_num: usize,
}

impl Writer {
    pub fn init(
        &mut self,
        elem_num: usize,
        month: DateTime<Local>,
        month_schedule: &Vec<Vec<Vec<String>>>
    ) {
        self.elem_num = elem_num;
        self.month = month.clone();
        self.month_schedule = month_schedule.clone();
    }

    pub fn export(&self, ext: FileExt) {
        match ext {
            FileExt::Xlsx => self.export_xlsx(),
            FileExt::Csv => {}
        }
    }

    fn export_xlsx(&self) {
        let mut workbook = Workbook::new();

        let white = Color::RGB(0xffffff);
        let wknd_color = Color::RGB(0xfce4d6);

        let date_format = Format::new()
            .set_bold()
            .set_background_color(white)
            .set_border_top(FormatBorder::Thin)
            .set_border_left(FormatBorder::Thin);

        let date_mid_format = date_format.clone()
            .set_border_left(FormatBorder::None);

        let date_right_format = date_mid_format.clone()
            .set_border_right(FormatBorder::Thin);

        let date_weekend_format = Format::new()
            .set_bold()
            .set_background_color(wknd_color)
            .set_border_top(FormatBorder::Thin)
            .set_border_left(FormatBorder::Thin);

        let date_mid_weekend_format = date_weekend_format.clone()
            .set_border_left(FormatBorder::None);

        let date_right_weekend_format = date_mid_weekend_format.clone()
            .set_border_right(FormatBorder::Thin);

        let merge_format = Format::new()
            .set_border(FormatBorder::Thin)
            .set_bold()
            .set_background_color(white)
            .set_align(FormatAlign::Center);

        let merge_weekend_format = Format::new()
            .set_border(FormatBorder::Thin)
            .set_bold()
            .set_background_color(wknd_color)
            .set_align(FormatAlign::Center);

        let element_mid_format = Format::new()
            .set_background_color(white);

        let element_left_format = element_mid_format.clone()
            .set_border_left(FormatBorder::Thin);

        let element_right_format = element_mid_format.clone()
            .set_border_right(FormatBorder::Thin);

        let element_mid_weekend_format = Format::new()
            .set_background_color(wknd_color);

        let element_left_weekend_format = element_mid_weekend_format.clone()
            .set_border_left(FormatBorder::Thin);

        let element_right_weekend_format = element_mid_weekend_format.clone()
            .set_border_right(FormatBorder::Thin);

        let bottom_left_format = Format::new()
            .set_background_color(white)
            .set_border_left(FormatBorder::Thin)
            .set_border_bottom(FormatBorder::Thin);

        let bottom_mid_format = bottom_left_format.clone()
            .set_border_left(FormatBorder::None);

        let bottom_right_format = bottom_mid_format.clone()
            .set_border_right(FormatBorder::Thin);

        let bottom_left_weekend_format = Format::new()
            .set_background_color(wknd_color)
            .set_border_left(FormatBorder::Thin)
            .set_border_bottom(FormatBorder::Thin);

        let bottom_mid_weekend_format = bottom_left_weekend_format.clone()
            .set_border_left(FormatBorder::None);

        let bottom_right_weekend_format = bottom_mid_weekend_format.clone()
            .set_border_right(FormatBorder::Thin);

        let worksheet = workbook.add_worksheet();

        let month: Month = Month::from_u32(self.month.month());
        let year = self.month.year();

        worksheet.write(
            0, 1, format!("Schedule for {} {}", month.as_str(), year)
        ).unwrap();

        for i in 0..7 {
            let day = Day::from_u32(i);
            let index: u16 = (i*3 + 1) as u16;

            worksheet.set_column_width(index, 3).unwrap();
            worksheet.set_column_width(index + 2, 3).unwrap();

            worksheet.merge_range(
                1,
                index,
                1,
                index+2,
                day.as_str(),
                if i == 0 || i == 6 {
                    &merge_weekend_format
                } else {
                    &merge_format
                },
            ).unwrap();
        }

        let mut day_number = 0;
        let mut start_printing_dates = false;
        let mut week_index = 0;

        // fill all the dates in
        for (i, week) in self.month_schedule.iter().enumerate() {
            if i == 0 {
                week_index = 2;
            } else {
                week_index += (self.elem_num + 2) as u32;
            }

            for (j, day) in week.iter().enumerate() {
                let day_index: u16 = (j*3 + 1) as u16;

                for k in 0..self.elem_num {
                    worksheet.write_with_format(
                        week_index + (k as u32) + 1,
                        day_index,
                        "",
                        if j == 0 || j == 6 {
                            &element_left_weekend_format
                        } else {
                            &element_left_format
                        },
                    ).unwrap();

                    worksheet.write_with_format(
                        week_index + (k as u32) + 1,
                        day_index + 1,
                        "",
                        if j == 0 || j == 6 {
                            &element_mid_weekend_format
                        } else {
                            &element_mid_format
                        },
                    ).unwrap();

                    worksheet.write_with_format(
                        week_index + (k as u32) + 1,
                        day_index + 2,
                        "",
                        if j == 0 || j == 6 {
                            &element_right_weekend_format
                        } else {
                            &element_right_format
                        },
                    ).unwrap();
                }

                if !day.is_empty() {
                    start_printing_dates = true;
                } 

                if j == 0 || j == 6 {
                    if start_printing_dates {
                        day_number += 1;

                        worksheet.write_number_with_format(
                            week_index,
                            day_index,
                            day_number,
                            &date_weekend_format,
                        ).unwrap();
                    } else {
                        worksheet.write_with_format(
                            week_index,
                            day_index,
                            "",
                            &date_weekend_format,
                        ).unwrap();
                    }

                    worksheet.write_with_format(
                        week_index,
                        day_index + 1,
                        "",
                        &date_mid_weekend_format,
                    ).unwrap();

                    worksheet.write_with_format(
                        week_index,
                        day_index + 2,
                        "",
                        &date_right_weekend_format,
                    ).unwrap();

                    for (k, elem) in day.iter().enumerate() {
                        worksheet.write_with_format(
                            week_index + (k as u32) + 1,
                            day_index,
                            "",
                            &element_left_weekend_format,
                        ).unwrap();

                        worksheet.write_with_format(
                            week_index + (k as u32) + 1,
                            day_index + 1,
                            elem,
                            &element_mid_weekend_format,
                        ).unwrap();

                        worksheet.write_with_format(
                            week_index + (k as u32) + 1,
                            day_index + 2,
                            "",
                            &element_right_weekend_format,
                        ).unwrap();
                    }
                } else {
                    if start_printing_dates {
                        day_number += 1;

                        worksheet.write_number_with_format(
                            week_index,
                            day_index,
                            day_number,
                            &date_format,
                        ).unwrap();
                    } else {
                        worksheet.write_with_format(
                            week_index,
                            day_index,
                            "",
                            &date_format,
                        ).unwrap();
                    }

                    worksheet.write_with_format(
                        week_index,
                        day_index + 1,
                        "",
                        &date_mid_format,
                    ).unwrap();

                    worksheet.write_with_format(
                        week_index,
                        day_index + 2,
                        "",
                        &date_right_format,
                    ).unwrap();

                    for (k, elem) in day.iter().enumerate() {
                        worksheet.write_with_format(
                            week_index + (k as u32) + 1,
                            day_index,
                            "",
                            &element_left_format,
                        ).unwrap();

                        worksheet.write_with_format(
                            week_index + (k as u32) + 1,
                            day_index + 1,
                            elem,
                            &element_mid_format,
                        ).unwrap();

                        worksheet.write_with_format(
                            week_index + (k as u32) + 1,
                            day_index + 2,
                            "",
                            &element_right_format,
                        ).unwrap();
                    }
                }
            }

            for j in 0..7 {
                let day_index: u16 = (j*3 + 1) as u16;

                if j == 0 || j == 6 {
                    worksheet.write_with_format(
                        week_index + self.elem_num as u32 + 1,
                        day_index,
                        "",
                        &bottom_left_weekend_format,
                    ).unwrap();

                    worksheet.write_with_format(
                        week_index + self.elem_num as u32 + 1,
                        day_index + 1,
                        "",
                        &bottom_mid_weekend_format,
                    ).unwrap();

                    worksheet.write_with_format(
                        week_index + self.elem_num as u32 + 1,
                        day_index + 2,
                        "",
                        &bottom_right_weekend_format,
                    ).unwrap();
                } else {
                    worksheet.write_with_format(
                        week_index + self.elem_num as u32 + 1,
                        day_index,
                        "",
                        &bottom_left_format,
                    ).unwrap();

                    worksheet.write_with_format(
                        week_index + self.elem_num as u32 + 1,
                        day_index + 1,
                        "",
                        &bottom_mid_format,
                    ).unwrap();

                    worksheet.write_with_format(
                        week_index + self.elem_num as u32 + 1,
                        day_index + 2,
                        "",
                        &bottom_right_format,
                    ).unwrap();
                }
            }
        }

        let output_path = DialogBuilder::file()
            .add_filter("Excel File", &[ ".xlsx" ])
            .save_single_file()
            .show()
            .unwrap();

        if let Some(path) = output_path {
            workbook.save(path).unwrap();
        }
    }
}

