use camino::Utf8PathBuf;

pub fn utf8path_buf_to_vec(utf8path_buf: &Utf8PathBuf) -> Vec<String> {
    utf8path_buf.with_extension("").iter().map(|x| x.to_string()).collect()
}