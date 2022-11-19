#[derive(Debug)]
pub struct ProgramArgs {
    pub fir_img_loc: String,
    pub sec_img_loc: String,
    pub out_img_loc: String,
  }
  
  impl ProgramArgs {
    pub fn new() -> Self {
        ProgramArgs {
            fir_img_loc: get_nth_arg(1),
            sec_img_loc: get_nth_arg(2),
            out_img_loc: get_nth_arg(3),
      }
    }
  }
  
  fn get_nth_arg(n: usize) -> String {
    std::env::args().nth(n).unwrap()
  }
  