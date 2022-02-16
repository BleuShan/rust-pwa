use super::*;
use crate::typed_header::{
    self,
    AcceptEncoding,
};
use std::path::{
    Path,
    PathBuf,
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
use rocket::{
    fs::NamedFile,
    response::Responder,
    Response,
};

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
    #[instrument(field(tmp = std::any::type_name::<Self>()), skip_all)]
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
            Some(path) if path.is_dir() => {
                let index_path = path.join("index.html");
                let file = NamedFile::open(index_path).await.ok();

                Outcome::from_or_forward(req, data, file)
            }
            Some(path) => {
                let file = NamedFile::open(path).await.ok();
                Outcome::from_or_forward(req, data, file)
            }
            _ => Outcome::forward(data),
        }
    }
}

#[derive(Debug)]
pub struct NamedFileResolver(PathBuf);
impl<'r> Responder<'r, 'static> for NamedFileResolver {
    fn respond_to(self, request: &'r Request<'_>) -> rocket::response::Result<'static> {
        let Self(path) = self;
        let encodings: Vec<AcceptEncoding> = request
            .headers()
            .get(header::ACCEPT_ENCODING.as_str())
            .flat_map(|s| s.split(","))
            .map(AcceptEncoding::from_str)
            .map(|item| item.unwrap())
            .collect();

        if !path.exists() {
            return Err(http::Status::NotFound);
        }

        let mut response = Response::build();

        response.ok()
    }
}

impl FromStr for FileServer {
    type Err = FileServerError;
    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        let root = PathBuf::from(s);
        if !root.is_dir() {
            return Err(FileServerError::InvalidRoot(root));
        }
        Ok(Self {
            root,
            rank: Self::DEFAULT_RANK,
        })
    }
}

impl<P> From<P> for FileServer
where
    P: AsRef<Path>,
{
    fn from(path: P) -> Self {
        Self::new(path)
    }
}

impl FileServer {
    const DEFAULT_RANK: isize = 10;
    #[track_caller]
    pub fn new<P>(path: P) -> Self
    where
        P: AsRef<Path>,
    {
        let root = path.as_ref().to_path_buf();
        if !root.is_dir() {
            error!("root: \"{}\" is not a directory", root.display());
            panic!()
        }
        Self {
            root,
            rank: Self::DEFAULT_RANK,
        }
    }

    #[track_caller]
    pub fn rank(mut self, rank: isize) -> Self {
        self.rank = rank;
        self
    }
}

#[derive(Error, Debug)]
pub enum FileServerError {
    #[error("\"{}\" is not a valid root", .0.display())]
    InvalidRoot(PathBuf),
    #[error(transparent)]
    IO(#[from] IOError),
}
