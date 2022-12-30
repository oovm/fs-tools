use self::flatten::FsFlatten;

mod flatten;

#[derive(Debug, Clone)]
pub struct FSTools {
    sub: Option<FSCommands>,
}

#[derive(Debug, Clone)]
pub enum FSCommands {
    Flatten(Box<FsFlatten>)
}

pub struct SharedArgs {

}