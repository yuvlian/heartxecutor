// this will need to be changed from time to time
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerHeartBeatCsReq {
    #[prost(uint64, tag = "5")]
    pub client_time_ms: u64,
    #[prost(uint32, tag = "6")]
    pub unknown: u32,
    #[prost(message, optional, tag = "7")]
    pub lua_file: ::core::option::Option<ClientUploadData>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientUploadData {
    #[prost(string, tag = "1")]
    pub file_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub file_content: ::prost::alloc::string::String,
}
