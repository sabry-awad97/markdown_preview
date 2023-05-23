use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Markdown Preview Tool",
    about = "Converts Markdown to HTML and opens it in a web browser for preview."
)]
struct Opt {
    #[structopt(short, long, help = "Markdown file to preview")]
    file: String,
}

fn main() {
    let opt = Opt::from_args();
    
}
