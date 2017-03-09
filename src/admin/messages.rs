// Copyright 2017 VMware, Inc. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use rabble::{Pid, NodeId, ClusterStatus, Metric};
use config::Config;
use namespaces::Namespaces;
use namespace_msg::NamespaceId;
use vr::VrCtxSummary;

#[derive(Debug, Clone, PartialEq, RustcEncodable, RustcDecodable)]
pub enum AdminMsg {
    Req(AdminReq),
    Rpy(AdminRpy)
}

#[derive(Debug, Clone, PartialEq, RustcEncodable, RustcDecodable)]
pub enum AdminReq {
    GetConfig,
    Join(NodeId),
    CreateNamespace(Vec<Pid>),
    GetNamespaces,
    GetReplicaState(Pid),
    GetPrimary(NamespaceId),
    GetClusterStatus,
    GetMetrics(Pid)
}

#[derive(Debug, Clone, PartialEq, RustcEncodable, RustcDecodable)]
pub enum AdminRpy {
    Ok,
    Timeout,
    Error(String),
    Config(Config),
    NamespaceId(NamespaceId),
    Namespaces(Namespaces),
    ReplicaState(VrCtxSummary),
    ReplicaNotFound(Pid),
    Primary(Option<Pid>),
    ClusterStatus(ClusterStatus),
    Metrics(Vec<(String, Metric)>)
}

