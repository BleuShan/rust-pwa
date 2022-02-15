use super::*;
use std::{
    default,
    path::{
        Path,
        PathBuf,
    },
};

#[derive(Debug, Clone)]
pub struct FileServer {
    root: PathBuf,
    rank: isize,
}
use http::{
    hyper::header,
    uri,
    Method,
};
use rocket::fs::NamedFile;

impl Into<Vec<Route>> for FileServer {
    fn into(self) -> Vec<Route> {
        let source = figment::Source::File(self.root.clone());
        let mut route = Route::ranked(self.rank, Method::Get, "/<path..>", self);
        route.name = Some(format!("FileServer: {}/", source).into());
        vec![route]
    }
}

#[async_trait]
impl Handler for FileServer {
    async fn handle<'r>(&self, req: &'r Request<'_>, data: Data<'r>) -> Outcome<'r> {
        use uri::{
            fmt::Path,
            Segments,
        };

        let maybe_path = req
            .segments::<Segments<'_, Path>>(0..)
            .ok()
            .and_then(|segments| segments.to_path_buf(false).ok())
            .map(|path| self.root.join(path));

        match maybe_path {
            Some(path) => {
                let encodings: Vec<_> = req
                    .headers()
                    .get(header::ACCEPT_ENCODING.as_str())
                    .map(QItem::from)
                    .filter(|item| item.name() != "gzip" || item.name() != "br")
                    .map(|encoding| match encoding.name() {
                        "gzip" => "gz",
                        "br" => "br",
                        _ => "default",
                    })
                    .collect();

                Outcome::from_or_forward(req, data, NamedFile::open(path).await.ok())
            }
            _ => Outcome::forward(data),
        }
    }
}

impl FileServer {
    const DEFAULT_RANK: isize = 10;

    pub fn new<P>(path: P) -> Result<Self, FileServerError>
    where
        P: AsRef<Path>,
    {
        let root = path.as_ref().to_path_buf();
        if !root.is_dir() {
            return Err(FileServerError::InvalidRoot(root));
        }
        Ok(Self {
            root,
            rank: Self::DEFAULT_RANK,
        })
    }

    pub fn rank(mut self, rank: isize) -> Self {
        self.rank = rank;
        self
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct QItem {
    name: String,
    quality: Option<f32>,
}

impl QItem {
    /// Get a reference to the qitem's name.
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    /// Get the qitem's quality.
    pub fn quality(&self) -> Option<f32> {
        self.quality
    }
}

impl PartialOrd for QItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let other_quality = other.quality().unwrap_or_default();
        let quality = self.quality().unwrap_or_default();

        quality.partial_cmp(&other_quality)
    }
}

impl Ord for QItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let other_quality = other.quality().unwrap_or_default();
        let quality = self.quality().unwrap_or_default();
        quality.total_cmp(&other_quality)
    }
}
impl Eq for QItem {}

impl From<&str> for QItem {
    fn from(s: &str) -> Self {
        let parts: Vec<&str> = s.split(";q=").take(2).collect();
        Self {
            name: parts[0].to_string(),
            quality: f32::from_str(parts[1]).ok(),
        }
    }
}

#[derive(Error, Debug)]
pub enum FileServerError {
    #[error("\"{}\" is not a valid root", .0.display())]
    InvalidRoot(PathBuf),
    #[error(transparent)]
    IO(#[from] IOError),
}
