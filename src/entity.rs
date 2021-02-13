pub type LawNumber = i32;

#[derive(Clone, Debug, serde::Deserialize)]
pub struct Law {
    pub id: LawNumber,
    pub name: String,
    pub text: String,
    pub legitimacy: Option<String>,
}
