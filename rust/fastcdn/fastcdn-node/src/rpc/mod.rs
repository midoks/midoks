//! RPC模块
//! 
//! 包含proto定义

// 引入生成的proto代码
pub mod hello {
    tonic::include_proto!("hello");
}

pub mod ping {
    tonic::include_proto!("ping");
}