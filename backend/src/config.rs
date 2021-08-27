use lazy_static::lazy_static;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "hview")]
pub struct Config {
    //// Root dir containing files to serve
    #[structopt(short, long, default_value = "./test-fixtures/")]
    pub dir: String,

    //// Basepath: Additional path for URI's (eg, "subdir" adds "/subdir/" to add URIs)
    #[structopt(long, default_value = "/")]
    pub basepath: String,

    //// Host to listen to
    #[structopt(long, short, default_value = "127.0.0.1")]
    pub host: String,

    //// Port to listen to
    #[structopt(long, short, default_value = "8000")]
    pub port: u16,

    //// Verbose log output
    #[structopt(long, short)]
    pub verbose: bool,

    //// Thumbnail format; becomes thumbnail extension as well
    //// Supported formats include image formats supported by imagemagick
    #[structopt(short, long, default_value = "avif")]
    pub format: String,

    //// Read-only; disables modification/deletion of files
    #[structopt(long)]
    pub read_only: bool,

    //// Disable thumbnails
    #[structopt(long, short)]
    pub no_thumbs: bool,
}

lazy_static! {
    pub static ref CFG: Config = Config::from_args();
    pub static ref DIR: PathBuf = PathBuf::from(&CFG.dir);
    pub static ref BASEPATH: &'static str = CFG.basepath.as_str();
    pub static ref THUMB_FORMAT: &'static str = CFG.format.as_str();
}
