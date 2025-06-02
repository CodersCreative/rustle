use crate::data::WORDS_PATH;
use crate::utils::get_path_src;

fn main() {
    include_bytes!(get_path_src(WORDS_PATH.to_string()));
}
