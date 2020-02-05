// Simpleweather
// Copyright (C) 2020  Konstantin Zhukov
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
use std::cmp;
use unicode_segmentation::UnicodeSegmentation;

use crate::CityWeather;

const CITY_CAPTION: &str = "City";
const CONDITION_CAPTION: &str = "Condition";
const TEMP_CAPTION: &str = "Temperature";
const TEMP_MIN_CAPTION: &str = "Min. temp.";
const TEMP_MAX_CAPTION: &str = "Max. temp.";
const PADDING: usize = 1;

fn calc_min_record_size(string: &str) -> usize {
    UnicodeSegmentation::graphemes(string, true).count() + 2
}

fn calc_column_width(column: &[&str]) -> usize {
    let mut biggest: usize = 0;

    for record in column {
        let record_width = calc_min_record_size(record);
        biggest = cmp::max(biggest, record_width);
    }

    biggest
}

fn calc_columns_number(columns: &[&[&str]]) -> usize {
    let mut num: usize = 0;
    for column in columns {
        num = cmp::max(column.len(), num);
    }
    num
}

fn temp_to_string(temperature: i16) -> String {
    format!("{} °C", temperature)
}

fn print_delimeter_line(column_widths: &[usize]) {
    print!("+");

    if column_widths.len() == 0 {
        println!("+");
        return;
    }

    for i in 0..column_widths.len() {
        let width = column_widths[i];
        for _ in 0..width {
            print!("-");
        }

        print!("+");
        if i == column_widths.len() - 1 {
            println!();
        }
    }
}

fn print_record_line(column_widths: &[usize], record: &[&str]) {
    print!("|");

    if column_widths.len() == 0 {
        println!("|");
        return;
    }

    for i in 0..column_widths.len() {
        let width = column_widths[i];
        let data = record[i];
        let data_length = UnicodeSegmentation::graphemes(data, true).count();

        let extra_padding = (width - 2 - data_length) / 2;
        let padding_right = width - 2 - data_length - extra_padding;

        for _ in 0..(PADDING + extra_padding) {
            print!(" ");
        }

        print!("{}", data);
        for _ in 0..(PADDING + padding_right) {
            print!(" ");
        }

        print!("|");
        if i == column_widths.len() - 1 {
            println!();
        }
    }
}

fn print_table(records: &[&[&str]]) {
    let mut column_widths: Vec<usize> = vec![];
    let columns_num = calc_columns_number(records);

    column_widths.reserve(records[0].len());

    for i in 0..columns_num {
        let mut column = vec![];
        column.reserve(records.len());
        for j in 0..records.len() {
            column.push(records[j][i]);
        }
        column_widths.push(calc_column_width(&column));
    }

    for record in records {
        print_delimeter_line(&column_widths);
        print_record_line(&column_widths, record);
    }
    print_delimeter_line(&column_widths);
}

pub fn print_current_weather(weather: &CityWeather) {
    let temp_str = temp_to_string(weather.temp);
    let temp_min_str = temp_to_string(weather.temp_min);
    let temp_max_str = temp_to_string(weather.temp_max);

    let records: [[&str; 5]; 2] = [
        [CITY_CAPTION, CONDITION_CAPTION, TEMP_CAPTION, TEMP_MIN_CAPTION, TEMP_MAX_CAPTION],
        [&weather.name, &weather.condition, &temp_str, &temp_min_str, &temp_max_str]
    ];

    let record_slices: [&[&str]; 2] = [&records[0][..], &records[1][..]];

    print_table(&record_slices);
}