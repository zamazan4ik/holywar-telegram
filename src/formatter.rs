use crate::entity;

pub fn format_law(law: &entity::Law) -> String {
    let mut formatted_law = format!("{}\n\n{}", law.name, law.text);

    if law.legitimacy.is_some() {
        formatted_law = format!("{}\n\nCтатус: отменена", formatted_law);
    }

    formatted_law
}
