use headless_chrome::Tab;
use seahorse::color;
use std::{sync::Arc, thread, time::Duration};

pub struct Jobcan {
    email: String,
    password: String,
}

impl Jobcan {
    pub fn new(email: String, password: String) -> Self {
        Self { email, password }
    }

    pub fn login(&self, tab: &Arc<Tab>) -> Result<(), failure::Error> {
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

        tab.wait_for_url("https://ssl.jobcan.jp/employee")?;

        thread::sleep(Duration::from_secs(2));

        Ok(())
    }

    pub fn get_status(&self, tab: &Arc<Tab>) -> Result<String, failure::Error> {
        let status = tab
            .wait_for_element_with_custom_timeout("div#working_status", Duration::from_secs(60))?
            .get_description()?
            .find(|n| n.node_name == "#text")
            .unwrap()
            .node_value
            .to_owned();
        Ok(status)
    }

    pub fn punch_in(&self, tab: &Arc<Tab>) -> Result<String, failure::Error> {
        let before_status = self.get_status(&tab)?;

        tab.wait_for_element_with_custom_timeout("p#adit-button-push", Duration::from_secs(60))?
            .click()?;
        thread::sleep(Duration::from_secs(3));

        let after_status = self.get_status(&tab)?;

        Ok(format!(
            "{} -> {}",
            color::yellow(before_status),
            color::green(after_status)
        ))
    }
}
