pub fn clist() -> anyhow::Result<()> {
    let contests = crate::contest::get_upcoming_contests()?;

    crate::contest::print_contests(&contests)
}
