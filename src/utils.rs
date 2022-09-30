pub fn is_sender_an_owner(
    from: &Option<&teloxide::types::User>,
    owner_id: teloxide::types::UserId,
) -> bool {
    if let Some(user) = from {
        user.id == owner_id
    } else {
        false
    }
}
