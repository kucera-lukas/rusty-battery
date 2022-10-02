use crate::device;

#[derive(Debug)]
pub struct Message {
    pub summary: String,
    pub body: String,

    summary_template: String,
    body_template: String,
}

impl Message {
    pub fn new(
        summary_template: String,
        body_template: String,
        threshold: u8,
        battery: &device::Battery,
        refresh_secs: u64,
    ) -> Self {
        let mut message = Self {
            summary: "".into(),
            body: "".into(),
            summary_template,
            body_template,
        };

        message.setup(threshold, refresh_secs, battery);
        message.update(battery);

        message
    }

    pub fn update(&mut self, battery: &device::Battery) {
        let charge_state = battery.percentage.to_string();

        let replace = |template: &str| {
            log::trace!(
                "message/update: replacing $CHARGE_STATE on \
                template \"{template}\" with {charge_state}"
            );

            template.replace("$CHARGE_STATE", &charge_state)
        };

        self.summary = replace(&self.summary_template);
        self.body = replace(&self.body_template);

        log::debug!("message/update: success");
    }

    fn setup(
        &mut self,
        threshold: u8,
        refresh_secs: u64,
        battery: &device::Battery,
    ) {
        let threshold_s = threshold.to_string();
        let model_s = battery.model.clone();
        let refresh_secs_s = refresh_secs.to_string();

        let replace = |template: &str| {
            log::trace!(
                "message/setup: setting up template \"{template}\" with \
                $THRESHOLD = {threshold_s}, \
                $MODEL = {model_s}, \
                $REFRESH_SECS = {refresh_secs_s}"
            );

            template
                .replace("$THRESHOLD", &threshold_s)
                .replace("$MODEL", &model_s)
                .replace("$REFRESH_SECS", &refresh_secs_s)
        };

        self.summary_template = replace(&self.summary_template);
        self.body_template = replace(&self.body_template);

        log::debug!("message/setup: success");
    }
}
