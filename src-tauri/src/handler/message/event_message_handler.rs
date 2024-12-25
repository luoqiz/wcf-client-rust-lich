use async_trait::async_trait;
use quickxml_to_serde::{xml_string_to_json, Config};

use crate::{handler::event_entity::{Event, EventHandler}, plugin::qingqing_plugin::{free_day, mul_day, one_day}, service::global_service::GLOBAL, wcferry::wcf};

/// 日志打印
pub struct EventMessageHandler {
    pub id: String,
}

#[async_trait]
impl EventHandler for EventMessageHandler {
    async fn handle(&mut self, event: Event) {
        if let Event::ClientMessage(ref msg) = event {
            if msg.r#type == 1 {
                
                log::debug!("[{}] 接收到事件推送：{:?}", self.id, msg);
                if !msg.content.contains("青青户外报名情况")  {
                    return
                }

                // 解析xml 判断是否是at自己的信息
                let mut is_at_me = false;

                let global = GLOBAL.get().unwrap();
                let wechat_service = global.wechat_service.clone();
                let self_wx_id = wechat_service.lock().unwrap().get_self_wxid();

                let json = xml_string_to_json(msg.xml.clone(), &Config::new_with_defaults()).unwrap();
                let msgsource = json.get("msgsource");
                if msgsource.is_some(){
                    // 事件推送中包含 @某人，记录 @某人列表
                    let at_user_list = msgsource.unwrap().get("atuserlist");
                    if at_user_list.is_some() {
                        
                        let at_user_list = at_user_list.unwrap();
                        for at_user_item in at_user_list.as_str().unwrap().split(",") {
                            if self_wx_id == at_user_item {
                                is_at_me = true;
                            }
                        }
                    }
                }

                // 如果不是@我，则停止处理
                if !is_at_me {
                    return;
                }
                
                let res = one_day().await.unwrap();
                let one_text_msg = wcf::TextMsg{
                    msg: format!("@{} \r\n{}",msg.sender.clone(),res),
                    receiver: msg.roomid.clone(),
                    aters: msg.sender.clone()
                };

                let res = mul_day().await.unwrap();
                let mul_text_msg = wcf::TextMsg{
                    msg: format!("@{} \r\n{}",msg.sender.clone(),res),
                    receiver: msg.roomid.clone(),
                    aters: msg.sender.clone()
                };

                let res = free_day().await.unwrap();
                let free_text_msg = wcf::TextMsg{
                    msg: format!("@{} \r\n{}",msg.sender.clone(),res),
                    receiver: msg.roomid.clone(),
                    aters: msg.sender.clone()
                };

                let wechat_service = global.wechat_service.clone();
                let mut wechat_service_lock = wechat_service.lock().unwrap();
                wechat_service_lock.send_text(one_text_msg);
                wechat_service_lock.send_text(mul_text_msg);
                wechat_service_lock.send_text(free_text_msg);
            }
        }
    }
}



