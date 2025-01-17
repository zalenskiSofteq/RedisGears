/*
 * Copyright Redis Ltd. 2018 - present
 * Licensed under your choice of the Redis Source Available License 2.0 (RSALv2) or
 * the Server Side Public License v1 (SSPLv1).
 */

use redis_module::{CallResult, RedisResult};

use crate::{Deserialize, Serialize};

use crate::redisgears_plugin_api::redisai_interface::{AIModelInterface, AIScriptInterface};
use crate::redisgears_plugin_api::GearsApiError;

pub trait RedisClientCtxInterface: Send + Sync {
    fn call(&self, command: &str, args: &[&[u8]]) -> CallResult;
    fn get_background_redis_client(&self) -> Box<dyn BackgroundRunFunctionCtxInterface>;
    fn open_ai_model(&self, name: &str) -> Result<Box<dyn AIModelInterface>, GearsApiError>;
    fn open_ai_script(&self, name: &str) -> Result<Box<dyn AIScriptInterface>, GearsApiError>;
}

pub trait ReplyCtxInterface: Send + Sync {
    fn send_reply(&self, reply: RedisResult);
    fn reply_with_error(&self, err: GearsApiError);
    fn as_client(&self) -> &dyn ReplyCtxInterface;
}

#[derive(Clone, Serialize, Deserialize)]
pub enum RemoteFunctionData {
    Binary(Vec<u8>),
    String(String),
}

pub trait BackgroundRunFunctionCtxInterface: Send + Sync {
    fn lock(&self) -> Result<Box<dyn RedisClientCtxInterface>, GearsApiError>;
    fn run_on_key(
        &self,
        key: &[u8],
        job_name: &str,
        inputs: Vec<RemoteFunctionData>,
        on_done: Box<dyn FnOnce(Result<RemoteFunctionData, GearsApiError>)>,
    );
    fn run_on_all_shards(
        &self,
        job_name: &str,
        inputs: Vec<RemoteFunctionData>,
        on_done: Box<dyn FnOnce(Vec<RemoteFunctionData>, Vec<GearsApiError>)>,
    );
}

pub trait RunFunctionCtxInterface: ReplyCtxInterface {
    fn get_args_iter(&self) -> Box<dyn Iterator<Item = &'_ [u8]> + '_>;
    fn get_background_client(&self) -> Result<Box<dyn ReplyCtxInterface>, GearsApiError>;
    fn get_redis_client(&self) -> Box<dyn RedisClientCtxInterface + '_>;
    fn allow_block(&self) -> bool;
}
