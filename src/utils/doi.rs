pub enum DoiPublisher {
    Acm,                // 10.1145
    Ieee,               // 10.1109

    Springer,           // 10.1007
    Elsevier,           // 10.1016
    Wiley,              // 10.1002, 10.1111
    Nature,             // 10.1038
    Plos,               // 10.1371
    Arxiv,              // 10.48550
    Sage,               // 10.1177
    TaylorFrancis,      // 10.1080
    Cambridge,          // 10.1017
    Oxford,             // 10.1093

    // Other major Crossref publishers
    ScienceAaas,        // 10.1126
    Acs,                // 10.1021
    Rsc,                // 10.1039
    Iop,                // 10.1088
    Aps,                // 10.1103
    Bmj,                // 10.1136
    Karger,             // 10.1159

    // Large open-access publishers
    Mdpi,               // 10.3390
    Hindawi,            // 10.1155
    Frontiers,          // 10.3389
    Copernicus,         // 10.5194

    // Government / international orgs
    Who,                // 10.2471
    Unesco,             // 10.54675
    Oecd,               // 10.1787
    Eu,                 // 10.2788

    // Preprint and repository services
    Zenodo,             // 10.5281
    Figshare,           // 10.6084
    Osf,                // 10.31219
    Ssrn,               // 10.2139

    // Large Asian publishers
    Jstage,             // 10.508
    Cnki,               // 10.273 (CNKI)
    Airiti,             // 10.6688

    // Fallback
    Unknown,
}

pub struct Doi {
    raw: String,
    publisher: DoiPublisher,
}

impl Doi {
    pub fn new(doi: &str) -> Self {
        let publisher = Self::detect_publisher(doi);
        Self {
            raw: doi.to_string(),
            publisher,
        }
    }

    fn detect_publisher(doi: &str) -> DoiPublisher {
        let prefix = doi.split('/').next().unwrap_or("");
        match prefix {
            "10.1145" => DoiPublisher::Acm,
            "10.1109" => DoiPublisher::Ieee,
            "10.1007" => DoiPublisher::Springer,
            "10.1016" => DoiPublisher::Elsevier,
            "10.1002" | "10.1111" => DoiPublisher::Wiley,
            "10.1038" => DoiPublisher::Nature,
            "10.1371" => DoiPublisher::Plos,
            "10.48550" => DoiPublisher::Arxiv,
            "10.1177" => DoiPublisher::Sage,
            "10.1080" => DoiPublisher::TaylorFrancis,
            "10.1017" => DoiPublisher::Cambridge,
            "10.1093" => DoiPublisher::Oxford,

            // Other major Crossref publishers
            "10.1126" => DoiPublisher::ScienceAaas,
            "10.1021" => DoiPublisher::Acs,
            "10.1039" => DoiPublisher::Rsc,
            "10.1088" => DoiPublisher::Iop,
            "10.1103" => DoiPublisher::Aps,
            "10.1136" => DoiPublisher::Bmj,
            "10.1159" => DoiPublisher::Karger,

            // Large open-access publishers
            "10.3390" => DoiPublisher::Mdpi,
            "10.1155" => DoiPublisher::Hindawi,
            "10.3389" => DoiPublisher::Frontiers,
            "10.5194" => DoiPublisher::Copernicus,

            // Government / international orgs
            "10.2471" => DoiPublisher::Who,
            "10.54675" => DoiPublisher::Unesco,
            "10.1787" => DoiPublisher::Oecd,
            "10.2788" => DoiPublisher::Eu,

            // Preprint and repository services
            "10.5281" => DoiPublisher::Zenodo,
            "10.6084" => DoiPublisher::Figshare,
            "10.31219" => DoiPublisher::Osf,
            "10.2139" => DoiPublisher::Ssrn,

            // Large Asian publishers
            "10.508" => DoiPublisher::Jstage,
            "10.273" => DoiPublisher::Cnki,
            "10.6688" => DoiPublisher::Airiti,

            _ => DoiPublisher::Unknown,
        }
    }

    pub fn construct_pdf_urls(&self) -> Vec<String> {
        let mut urls = Vec::new();

        match self.publisher {
            DoiPublisher::Acm => {
                urls.push(format!("https://dl.acm.org/doi/pdf/{}", self.raw));
            }
            DoiPublisher::Ieee => {
                // IEEE DOI format: 10.1109/5.771073 -> arnumber is 771073
                // PDF URL: https://ieeexplore.ieee.org/stampPDF/getPDF.jsp?tp=&arnumber=771073
                let parts: Vec<&str> = self.raw.split('/').collect();
                if let Some(last_part) = parts.last() {
                    if let Some(arnumber) = last_part.split('.').last() {
                        urls.push(format!(
                            "https://ieeexplore.ieee.org/stampPDF/getPDF.jsp?tp=&arnumber={}",
                            arnumber
                        ));
                    }
                }
            }
            DoiPublisher::Springer => {
                urls.push(format!("https://link.springer.com/content/pdf/{}.pdf", self.raw));
            }
            DoiPublisher::Elsevier => {
                // ScienceDirect not supported - requires PII extraction from HTML
            }
            DoiPublisher::Wiley => {
                urls.push(format!("https://onlinelibrary.wiley.com/doi/pdf/{}", self.raw));
            }
            DoiPublisher::Nature => {
                if let Some(article_id) = self.raw.split('/').nth(1) {
                    urls.push(format!("https://www.nature.com/articles/{}.pdf", article_id));
                }
            }
            DoiPublisher::Plos => {
                urls.push(format!(
                    "https://journals.plos.org/plosone/article/file?id={}&type=printable",
                    self.raw
                ));
            }
            DoiPublisher::Arxiv => {
                if let Some(arxiv_id) = self.raw.strip_prefix("10.48550/arXiv.") {
                    urls.push(format!("https://arxiv.org/pdf/{}.pdf", arxiv_id));
                }
            }
            DoiPublisher::Sage => {
                urls.push(format!("https://journals.sagepub.com/doi/pdf/{}", self.raw));
            }
            DoiPublisher::TaylorFrancis => {
                urls.push(format!("https://www.tandfonline.com/doi/pdf/{}", self.raw));
            }
            DoiPublisher::Cambridge => {
                urls.push(format!(
                    "https://www.cambridge.org/core/services/aop-cambridge-core/content/view/pdf/{}",
                    self.raw
                ));
            }
            DoiPublisher::Oxford => {
            }

            // Other major Crossref publishers
            DoiPublisher::ScienceAaas => {
                urls.push(format!("https://www.science.org/doi/pdf/{}", self.raw));
            }
            DoiPublisher::Acs => {
                urls.push(format!("https://pubs.acs.org/doi/pdf/{}", self.raw));
            }
            DoiPublisher::Rsc => {
                urls.push(format!("https://pubs.rsc.org/en/content/articlepdf/{}", self.raw));
            }
            DoiPublisher::Iop => {
                urls.push(format!("https://iopscience.iop.org/article/{}/pdf", self.raw));
            }
            DoiPublisher::Aps => {
                urls.push(format!("https://journals.aps.org/pdf/{}", self.raw));
            }
            DoiPublisher::Bmj => {
                urls.push(format!("https://www.bmj.com/content/bmj/{}.full.pdf",
                    self.raw.replace("10.1136/", "")));
            }
            DoiPublisher::Karger => {
                urls.push(format!("https://karger.com/Article/Pdf/{}", self.raw));
            }

            // Large open-access publishers
            DoiPublisher::Mdpi => {
                // MDPI handled via DOI redirect + /pdf appending in paper.rs
                // No direct URL generation needed
            }
            DoiPublisher::Hindawi => {
                // Hindawi now hosted on Wiley Online Library
                urls.push(format!("https://onlinelibrary.wiley.com/doi/epdf/{}", self.raw));
            }
            DoiPublisher::Frontiers => {
                urls.push(format!("https://www.frontiersin.org/articles/{}/pdf", self.raw));
            }
            DoiPublisher::Copernicus => {
                urls.push(format!("https://www.{}.net/article.pdf",
                    self.raw.split('/').nth(1).unwrap_or("")));
            }

            // Government / international orgs
            DoiPublisher::Who => {
                urls.push(format!("https://apps.who.int/iris/bitstream/handle/{}/pdf",
                    self.raw.replace("10.2471/", "")));
            }
            DoiPublisher::Unesco => {
                urls.push(format!("https://unesdoc.unesco.org/ark:/48223/{}/PDF",
                    self.raw.replace("10.54675/", "")));
            }
            DoiPublisher::Oecd => {
                urls.push(format!("https://www.oecd-ilibrary.org/deliver/{}.pdf",
                    self.raw.replace("10.1787/", "")));
            }
            DoiPublisher::Eu => {
                urls.push(format!("https://data.europa.eu/doi/{}.pdf", self.raw));
            }

            // Preprint and repository services
            DoiPublisher::Zenodo => {
                if let Some(record_id) = self.raw.strip_prefix("10.5281/zenodo.") {
                    urls.push(format!("https://zenodo.org/record/{}/files/article.pdf", record_id));
                }
            }
            DoiPublisher::Figshare => {
                if let Some(article_id) = self.raw.split('/').nth(1) {
                    urls.push(format!("https://figshare.com/articles/dataset/{}/1/files/{}.pdf",
                        article_id, article_id));
                }
            }
            DoiPublisher::Osf => {
                if let Some(preprint_id) = self.raw.strip_prefix("10.31219/osf.io/") {
                    urls.push(format!("https://osf.io/{}/download", preprint_id));
                }
            }
            DoiPublisher::Ssrn => {
                if let Some(paper_id) = self.raw.strip_prefix("10.2139/ssrn.") {
                    urls.push(format!("https://papers.ssrn.com/sol3/Delivery.cfm?abstractid={}",
                        paper_id));
                }
            }

            // Large Asian publishers
            DoiPublisher::Jstage => {
                urls.push(format!("https://www.jstage.jst.go.jp/article/{}/pdf", self.raw));
            }
            DoiPublisher::Cnki => {
                urls.push(format!("https://doi.org/{}", self.raw));
            }
            DoiPublisher::Airiti => {
                urls.push(format!("https://doi.org/{}", self.raw));
            }

            DoiPublisher::Unknown => {}
        }

        urls
    }
}
