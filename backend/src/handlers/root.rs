use crate::types::root_msg::RootMsg;

pub async fn root<'a>() -> RootMsg<'a> {
    RootMsg {
        msg: "Test"
    }
}
