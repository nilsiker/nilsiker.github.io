use yew::Properties;

pub mod about;
pub mod contributions;
pub mod projects;
#[derive(Properties, Eq, PartialEq)]
pub struct PageProps {
    pub hidden: bool,
}
