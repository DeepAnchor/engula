// Copyright 2021 The Engula Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::{
    client::Client,
    proto::{CreateStreamRequest, DeleteStreamRequest},
    stream::Stream,
};
use crate::{async_trait, Result};

#[derive(Clone)]
pub struct Journal {
    client: Client,
}

impl Journal {
    pub async fn connect(addr: &str) -> Result<Journal> {
        let client = Client::connect(addr).await?;
        Ok(Journal { client })
    }
}

#[async_trait]
impl crate::Journal for Journal {
    type Stream = Stream;

    async fn stream(&self, name: &str) -> Result<Stream> {
        Ok(Stream::new(self.client.clone(), name.to_owned()))
    }

    async fn create_stream(&self, name: &str) -> Result<Stream> {
        let input = CreateStreamRequest {
            stream: name.to_owned(),
        };
        self.client.create_stream(input).await?;
        self.stream(name).await
    }

    async fn delete_stream(&self, name: &str) -> Result<()> {
        let input = DeleteStreamRequest {
            stream: name.to_owned(),
        };
        self.client.delete_stream(input).await?;
        Ok(())
    }
}
