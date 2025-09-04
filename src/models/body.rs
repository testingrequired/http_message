pub type PossibleHttpBody = Option<String>;

pub trait HttpBody {
    fn get_body(&self) -> &PossibleHttpBody;

    fn set_body(&mut self, value: PossibleHttpBody);
}
