use chrono::{prelude::*, Local};
use headless_chrome::Tab;
use seahorse::color;
use std::error::Error;
use std::{sync::Arc, thread, time::Duration};

pub struct Jobcan {
    email: String,
    password: String,
}

impl Jobcan {
    pub fn new(email: String, password: String) -> Self {
        Self { email, password }
    }

    pub fn login(&self, tab: &Arc<Tab>) -> Result<(), Box<dyn Error>> {
        let url = "https://id.jobcan.jp/users/sign_in?app_key=atd";

        tab.navigate_to(url)?;

        tab.wait_for_element_with_custom_timeout("input#user_email", Duration::from_secs(60))?
            .click()?;
        tab.type_str(&self.email)?;

        tab.wait_for_element_with_custom_timeout("input#user_password", Duration::from_secs(60))?
            .click()?;
        tab.type_str(&self.password)?;

        tab.wait_for_element_with_custom_timeout("input.form__login", Duration::from_secs(60))?
            .click()?;

        // in https://ssl.jobcan.jp/employee
        tab.wait_for_element_with_custom_timeout("h3#working_status", Duration::from_secs(20))?;

        thread::sleep(Duration::from_secs(2));

        Ok(())
    }

    pub fn get_status(&self, tab: &Arc<Tab>) -> Result<String, Box<dyn Error>> {
        let status = tab
            .wait_for_element_with_custom_timeout("h3#working_status", Duration::from_secs(60))?
            .get_description()?
            .find(|n| n.node_name == "#text")
            .unwrap()
            .node_value
            .to_owned();
        Ok(status)
    }

    pub fn punch_in(&self, tab: &Arc<Tab>) -> Result<String, Box<dyn Error>> {
        let before_status = self.get_status(&tab)?;

        tab.wait_for_element_with_custom_timeout("#adit-button-push", Duration::from_secs(60))?
            .click()?;
        thread::sleep(Duration::from_secs(3));

        let after_status = self.get_status(&tab)?;

        Ok(format!(
            "{} -> {}",
            color::yellow(before_status),
            color::green(after_status)
        ))
    }

    pub fn pto(
        &self,
        tab: &Arc<Tab>,
        start_date: NaiveDate,
        end_date: NaiveDate,
        reason: String,
    ) -> Result<(), Box<dyn Error>> {
        tab.wait_for_element_with_custom_timeout("a#menu_order_img", Duration::from_secs(10))?
            .click()?;

        tab.wait_for_element_with_custom_timeout("#menu_order > table:nth-child(1) > tbody:nth-child(1) > tr:nth-child(1) > td:nth-child(1) > a:nth-child(1)", Duration::from_secs(60))?
            .click()?;

        tab.wait_for_element_with_custom_timeout("span.btn", Duration::from_secs(10))?
            .click()?;

        let today = Local::today();

        let diff_start_year: i32 = today.year() - start_date.year();
        let diff_start_month: i32 = today.month() as i32 - start_date.month() as i32;
        let diff_start_day: i32 = today.day() as i32 - start_date.day() as i32;

        tab.wait_for_element("#holiday_id")?.click()?;

        tab.press_key("ArrowDown")?
            .press_key("Enter")?
            .press_key("Tab")?;

        tab.wait_for_element("#holiday_year")?.click()?;
        if diff_start_year > 0 {
            for _ in 0..diff_start_year.abs() {
                tab.press_key("ArrowUp")?;
            }
            tab.press_key("Enter")?.press_key("Tab")?;
        } else {
            for _ in 0..diff_start_year.abs() {
                tab.press_key("ArrowDown")?;
            }
            tab.press_key("Enter")?.press_key("Tab")?;
        }

        tab.wait_for_element("#holiday_month")?.click()?;
        if diff_start_month > 0 {
            for _ in 0..diff_start_month.abs() {
                tab.press_key("ArrowUp")?.press_key("Tab")?;
            }
            tab.press_key("Enter")?;
        } else {
            for _ in 0..diff_start_month.abs() {
                tab.press_key("ArrowDown")?.press_key("Tab")?;
            }
            tab.press_key("Enter")?.press_key("Tab")?;
        }

        tab.wait_for_element("#holiday_day")?.click()?;
        if diff_start_day > 0 {
            for _ in 0..diff_start_day.abs() {
                tab.press_key("ArrowUp")?;
            }
            tab.press_key("Enter")?;
        } else {
            for _ in 0..diff_start_day.abs() {
                tab.press_key("ArrowDown")?;
            }
            tab.press_key("Enter")?.press_key("Tab")?;
        }

        let today = FixedOffset::east(9 * 3600).ymd(
            today.year().max(start_date.year()),
            today.month().max(start_date.month()) as u32,
            today.day().max(start_date.day()) as u32,
        );
        let diff_end_year: i32 = today.year() - end_date.year();
        let diff_end_month: i32 = today.month() as i32 - end_date.month() as i32;
        let diff_end_day: i32 = today.day() as i32 - end_date.day() as i32;

        tab.wait_for_element("#to_holiday_year")?.click()?;
        if diff_end_year > 0 {
            for _ in 0..diff_end_year.abs() {
                tab.press_key("ArrowUp")?;
            }
        } else {
            for _ in 0..diff_end_year.abs() {
                tab.press_key("ArrowDown")?;
            }
            tab.press_key("Enter")?.press_key("Tab")?;
        }

        tab.wait_for_element("#to_holiday_month")?.click()?;
        if diff_end_month > 0 {
            for _ in 0..diff_end_month.abs() {
                tab.press_key("ArrowUp")?;
            }
        } else {
            for _ in 0..diff_end_month.abs() {
                tab.press_key("ArrowDown")?;
            }
            tab.press_key("Enter")?.press_key("Tab")?;
        }

        tab.wait_for_element("#to_holiday_day")?.click()?;
        if diff_end_day > 0 {
            for _ in 0..diff_end_day.abs() {
                tab.press_key("ArrowUp")?;
            }
        } else {
            for _ in 0..diff_end_day.abs() {
                tab.press_key("ArrowDown")?;
            }
            tab.press_key("Enter")?.press_key("Tab")?;
        }

        let script = format!(
            "document.querySelector('textarea:nth-child(1)').textContent = '{}';",
            reason
        );
        tab.evaluate(&script, false)?;
        tab.wait_for_element(".btn")?.click()?;

        tab.wait_for_element_with_custom_timeout("input.btn", Duration::from_secs(15))?
            .click()?;

        Ok(())
    }
}
