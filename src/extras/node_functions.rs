#[cold]
#[napi]
pub fn get_thread_affinity() -> Vec<u32> {
    let res = match affinity::get_thread_affinity() {
        Ok(res) => res,
        Err(_) => {
            println!("Error getting afinity");
            return vec![];
        }
    };

    res.into_iter().map(|core| core as u32).collect()
}