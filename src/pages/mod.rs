use yew::Properties;

pub mod about;
#[derive(Properties, PartialEq)]
pub struct PageProps {
    pub hidden: bool,
}
