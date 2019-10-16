use crate::configure::Target;

pub trait Client {
    fn update(target: &Target);
}
