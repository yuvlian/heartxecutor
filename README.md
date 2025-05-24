i dont like using chat to run lua

you can use this as the serverside implementation reference:

```rust
static SHOULD_SEND_LUA: AtomicBool = AtomicBool::new(false);
static LUA_CONTENT: LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new(String::with_capacity(2000)));
const MAGIC_TIME: u64 = 11112222;

fn insert_lua(content: &str) {
    let mut lua = LUA_CONTENT.lock().unwrap();
    lua.clear();
    lua.push_str(content);
}

fn clear_lua() {
    LUA_CONTENT.lock().unwrap().clear();
}

fn get_lua() -> String {
    LUA_CONTENT.lock().unwrap().clone()
}

pub async fn player_heart_beat(req: &[u8], sink: &mut PacketSink) {
    let req = PlayerHeartBeatCsReq::decode(req).unwrap_or_default();

    if req.client_time_ms == MAGIC_TIME {
        let file = req.lkjmjgdebee.unwrap();
        let file_content = file.value;
        insert_lua(&file_content);
        SHOULD_SEND_LUA.store(true, Ordering::Release);
        return;
    }

    if SHOULD_SEND_LUA.load(Ordering::Acquire) {
        let file_content = get_lua();
        let cur_time = cur_timestamp_ms();

        let rsp = PlayerHeartBeatScRsp {
            client_time_ms: req.client_time_ms,
            server_time_ms: cur_time,
            download_data: Some(ClientDownloadData {
                time: cur_time as i64,
                data: file_content.into(),
                ..Default::default()
            }),
            ..Default::default()
        };

        sink.push_message(PLAYER_HEART_BEAT_SC_RSP, rsp);
        SHOULD_SEND_LUA.store(false, Ordering::Release);
        clear_lua();
        return;
    }

    let rsp = PlayerHeartBeatScRsp {
        client_time_ms: req.client_time_ms,
        server_time_ms: cur_timestamp_ms(),
        ..Default::default()
    };

    sink.push_message(PLAYER_HEART_BEAT_SC_RSP, rsp);
}
```
