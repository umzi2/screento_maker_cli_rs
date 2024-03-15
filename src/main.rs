mod utils;

use std::path::{Path, PathBuf};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use crate::utils::screenton::Screenton;
use crate::utils::image_ut::{read,save};

fn process(inp: &PathBuf, out:&PathBuf, mut scr:Screenton){
    let mut array = read(inp);
    scr.run(&mut array);
    save(array,out)
}

fn main() {
    let (inp,out,dot)=utils::parse::parse_args();
    let inp = Path::new(&inp);
    let out = Path::new(&out);
    let dot_center = dot/2;
    let src= Screenton::new(dot,dot_center,dot_center);
    let paths = utils::paths::path2vec_img(inp);
    let pb = ProgressBar::new(paths.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}, {per_sec})").expect("REASON")
            .progress_chars("#>-"),
    );
    paths.par_iter().for_each(|path| {
        let src_copy = src.clone();
        let input_img = inp.join(path);
        let out_img = out.join(path);
        process(&input_img,&out_img,src_copy);

        // Увеличиваем значение прогресса прогресс бара
        pb.inc(1);
    });
    pb.finish();


}