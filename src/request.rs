use serde_json::json;
use crate::model::*;
use crate::BumpClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostDiffsRequest<'a> {
    pub(crate) client: &'a BumpClient,
    pub url: Option<String>,
    pub previous_url: Option<String>,
    pub previous_definition: Option<String>,
    pub previous_references: Option<Vec<Reference>>,
    pub definition: Option<String>,
    pub references: Option<Vec<Reference>>,
    pub expires_at: Option<String>,
}
impl<'a> PostDiffsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.post("/diffs");
        if let Some(ref unwrapped) = self.url {
            r = r.push_json(json!({ "url" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.previous_url {
            r = r.push_json(json!({ "previous_url" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.previous_definition {
            r = r.push_json(json!({ "previous_definition" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.previous_references {
            r = r.push_json(json!({ "previous_references" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.definition {
            r = r.push_json(json!({ "definition" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.references {
            r = r.push_json(json!({ "references" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.expires_at {
            r = r.push_json(json!({ "expires_at" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn url(mut self, url: &str) -> Self {
        self.url = Some(url.to_owned());
        self
    }
    pub fn previous_url(mut self, previous_url: &str) -> Self {
        self.previous_url = Some(previous_url.to_owned());
        self
    }
    pub fn previous_definition(mut self, previous_definition: &str) -> Self {
        self.previous_definition = Some(previous_definition.to_owned());
        self
    }
    pub fn previous_references(mut self, previous_references: Vec<Reference>) -> Self {
        self.previous_references = Some(previous_references);
        self
    }
    pub fn definition(mut self, definition: &str) -> Self {
        self.definition = Some(definition.to_owned());
        self
    }
    pub fn references(mut self, references: Vec<Reference>) -> Self {
        self.references = Some(references);
        self
    }
    pub fn expires_at(mut self, expires_at: &str) -> Self {
        self.expires_at = Some(expires_at.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetDiffsByIdRequest<'a> {
    pub(crate) client: &'a BumpClient,
    pub id: String,
    pub formats: Option<Vec<String>>,
}
impl<'a> GetDiffsByIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<DiffForApi> {
        let mut r = self.client.client.get(&format!("/diffs/{id}", id = self.id));
        if let Some(ref unwrapped) = self.formats {
            for item in unwrapped {
                r = r.push_query("formats[]", &item.to_string());
            }
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn formats(
        mut self,
        formats: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .formats = Some(
            formats.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetHubsByHubIdOrSlugRequest<'a> {
    pub(crate) client: &'a BumpClient,
    pub hub_id_or_slug: String,
}
impl<'a> GetHubsByHubIdOrSlugRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Hub> {
        let mut r = self
            .client
            .client
            .get(
                &format!("/hubs/{hub_id_or_slug}", hub_id_or_slug = self.hub_id_or_slug),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostVersionsRequest<'a> {
    pub(crate) client: &'a BumpClient,
    pub documentation: String,
    pub hub: String,
    pub documentation_name: String,
    pub auto_create_documentation: bool,
    pub definition: String,
    pub references: Vec<Reference>,
    pub branch_name: String,
    pub previous_version_id: String,
    pub unpublished: bool,
}
impl<'a> PostVersionsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Version> {
        let mut r = self.client.client.post("/versions");
        r = r.push_json(json!({ "documentation" : self.documentation }));
        r = r.push_json(json!({ "hub" : self.hub }));
        r = r.push_json(json!({ "documentation_name" : self.documentation_name }));
        r = r
            .push_json(
                json!({ "auto_create_documentation" : self.auto_create_documentation }),
            );
        r = r.push_json(json!({ "definition" : self.definition }));
        r = r.push_json(json!({ "references" : self.references }));
        r = r.push_json(json!({ "branch_name" : self.branch_name }));
        r = r.push_json(json!({ "previous_version_id" : self.previous_version_id }));
        r = r.push_json(json!({ "unpublished" : self.unpublished }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostVersionsRequired<'a> {
    pub documentation: &'a str,
    pub hub: &'a str,
    pub documentation_name: &'a str,
    pub auto_create_documentation: bool,
    pub definition: &'a str,
    pub references: Vec<Reference>,
    pub branch_name: &'a str,
    pub previous_version_id: &'a str,
    pub unpublished: bool,
}
impl<'a> PostVersionsRequired<'a> {}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostValidationsRequest<'a> {
    pub(crate) client: &'a BumpClient,
    pub documentation: String,
    pub hub: String,
    pub documentation_name: String,
    pub auto_create_documentation: bool,
    pub url: String,
    pub definition: String,
    pub references: Vec<Reference>,
}
impl<'a> PostValidationsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Validation> {
        let mut r = self.client.client.post("/validations");
        r = r.push_json(json!({ "documentation" : self.documentation }));
        r = r.push_json(json!({ "hub" : self.hub }));
        r = r.push_json(json!({ "documentation_name" : self.documentation_name }));
        r = r
            .push_json(
                json!({ "auto_create_documentation" : self.auto_create_documentation }),
            );
        r = r.push_json(json!({ "url" : self.url }));
        r = r.push_json(json!({ "definition" : self.definition }));
        r = r.push_json(json!({ "references" : self.references }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PostValidationsRequired<'a> {
    pub documentation: &'a str,
    pub hub: &'a str,
    pub documentation_name: &'a str,
    pub auto_create_documentation: bool,
    pub url: &'a str,
    pub definition: &'a str,
    pub references: Vec<Reference>,
}
impl<'a> PostValidationsRequired<'a> {}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostPreviewsRequest<'a> {
    pub(crate) client: &'a BumpClient,
    pub definition: String,
    pub references: Option<Vec<Reference>>,
}
impl<'a> PostPreviewsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Preview> {
        let mut r = self.client.client.post("/previews");
        r = r.push_json(json!({ "definition" : self.definition }));
        if let Some(ref unwrapped) = self.references {
            r = r.push_json(json!({ "references" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn references(mut self, references: Vec<Reference>) -> Self {
        self.references = Some(references);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PutPreviewsByPreviewIdRequest<'a> {
    pub(crate) client: &'a BumpClient,
    pub preview_id: String,
    pub definition: String,
    pub references: Option<Vec<Reference>>,
}
impl<'a> PutPreviewsByPreviewIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Preview> {
        let mut r = self
            .client
            .client
            .put(&format!("/previews/{preview_id}", preview_id = self.preview_id));
        r = r.push_json(json!({ "definition" : self.definition }));
        if let Some(ref unwrapped) = self.references {
            r = r.push_json(json!({ "references" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn references(mut self, references: Vec<Reference>) -> Self {
        self.references = Some(references);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetVersionsByVersionIdRequest<'a> {
    pub(crate) client: &'a BumpClient,
    pub version_id: String,
}
impl<'a> GetVersionsByVersionIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .get(&format!("/versions/{version_id}", version_id = self.version_id));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetPingRequest<'a> {
    pub(crate) client: &'a BumpClient,
}
impl<'a> GetPingRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Pong> {
        let mut r = self.client.client.get("/ping");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
