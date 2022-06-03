pub struct Ui {
    crayon: crayon::Crayon,
    enabled: bool,
}

impl Ui {
    #[inline]
    pub fn crayon(&self) -> &crayon::Crayon {
        &self.crayon
    }

    #[inline]
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}