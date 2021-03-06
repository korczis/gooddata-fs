use rustc_serialize::json;

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct ReportContent {
    pub domains: Option<Vec<String>>,
    pub definitions: Option<Vec<String>>,
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct Report {
    pub report: super::MetadataObjectBody<ReportContent>,
}

impl Report {
    pub fn object(&self) -> &super::MetadataObjectBody<ReportContent> {
        &self.report
    }
}

impl super::MetadataObjectGetter<ReportContent> for Report {
    fn object(&self) -> &super::MetadataObjectBody<ReportContent> {
        &self.object()
    }
}

impl Into<String> for Report {
    fn into(self) -> String {
        format!("{}\n", json::as_pretty_json(&self).to_string())
    }
}

pub const NAME: &'static str = "report";

impl super::MetadataObjectRootKey for Report {
    fn root_key() -> String {
        NAME.to_string()
    }
}

impl super::MetadataQuery<super::MetadataQueryBody<Report>> {
    pub fn find_by_identifier(&self, identifier: &String) -> (u32, Option<Report>) {
        let mut i: u32 = 0;
        for item in self.objects().items().into_iter() {
            if item.object().meta().identifier().as_ref().unwrap() == identifier {
                return (i, Some(item.clone()));
            }

            i += 1;
        }

        (0, None)
    }
}
