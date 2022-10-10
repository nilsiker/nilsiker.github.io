use yew::Properties;

pub mod about;
pub mod projects;
pub mod contributions;
#[derive(Properties, Eq, PartialEq)]
pub struct PageProps {
    pub hidden: bool,
}
