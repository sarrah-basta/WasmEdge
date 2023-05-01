use std::os::raw::c_char;
use std::ffi::{CString, CStr};

fn main() {
    // let read_string = "1\t4\t0\t0\t0\t0\t0\t0\t696\t89\t-1\t\n2\t4\t1\t0\t0\t0\t18\t29\t653\t35\t-1\t\n3\t4\t1\t1\t0\t0\t18\t29\t653\t35\t-1\t\n4\t4\t1\t1\t1\t0\t18\t29\t653\t35\t-1\t\n5\t4\t1\t1\t1\t1\t18\t29\t144\t35\t95\tLOREM\n5\t4\t1\t1\t1\t2\t181\t29\t123\t35\t91\tIPSUM\n5\t4\t1\t1\t1\t3\t323\t29\t153\t35\t91\tDOLOR\n5\t4\t1\t1\t1\t4\t490\t29\t50\t35\t96\tSIT\n5\t4\t1\t1\t1\t5\t553\t30\t118\t33\t96\tAMET\n";
    let read_string = "/home/sarrah/1_data/wasmedge_intern/wasmedge_ai_testing/layoutlmv2_model/img/document.png";
    // parsing original string
    // let read_data = string_to_data(read_string).unwrap();
    // let read_words= read_data.iter().map(|x| &x.text).collect::<Vec<_>>();
    // println!("\nInput Words \n{:?}",read_words );
    let obt_string;
    unsafe{
        obt_string= return_obt_string(read_string); // CStr is obtained
    }
    println!("Plugin creates string in C itself");
    println!("\n\nOUTPUT CSTRING AS IS \n{:?}", obt_string);
    // options : format!("{:?}", obt_string).as_str()
    println!("\n\nOUTPUT CSTRING IN THE REQUIRED FORMAT\n{:?}", obt_string.as_str()); 

    // parsing the obtained string
    let obt_data = string_to_data(obt_string.as_str()).unwrap();
    let obt_words= obt_data.iter().map(|x| &x.text).collect::<Vec<_>>();
    println!("\nObtained Words \n{:?}",obt_words );
}

pub unsafe fn return_obt_string(read_string:&str)->String{
    // string to read
    let cread_string: CString = CString::new(read_string.as_bytes()).expect("");

    // empty buffer to store written string
    let length = cread_string.as_bytes().len();
    // Create a buffer in Rust
    let mut write_vec: Vec<c_char> = vec![0; length as usize];
    // let mut out_vec= [0; 200]; // just results in even more errors
    let write_ptr = write_vec.as_mut_ptr();

    wasi_string_test::return_obtained_string(cread_string.as_ptr(), cread_string.as_bytes().len() as u32, write_ptr, length as u32);

    let cwrite_string : &CStr= CStr::from_ptr(write_ptr as *const c_char);
    let mut output_c_string : String= cwrite_string.to_owned().into_string().unwrap();
    
    return output_c_string;
}

pub mod wasi_string_test {
    use std::os::raw::c_char;
    #[link(wasm_import_module = "wasi_string_test")]
    extern "C" {
        pub fn return_obtained_string(rbuf_ptr: *const c_char, rbuf_len: u32, wbuf_ptr: *const c_char, wbuf_len:u32);
    }
}

#[derive(Debug, PartialEq)]
pub struct Data {
    pub level: i32,
    pub page_num: i32,
    pub block_num: i32,
    pub par_num: i32,
    pub line_num: i32,
    pub word_num: i32,
    pub left: i32,
    pub top: i32,
    pub width: i32,
    pub height: i32,
    pub conf: f32,
    pub text: String,
}
impl FromLine for Data {
    fn from_line(line: &str) -> Option<Self> {
        let mut x = line.split_whitespace();
        Some(Data {
            level: parse_next(&mut x)?,
            page_num: parse_next(&mut x)?,
            block_num: parse_next(&mut x)?,
            par_num: parse_next(&mut x)?,
            line_num: parse_next(&mut x)?,
            word_num: parse_next(&mut x)?,
            left: parse_next(&mut x)?,
            top: parse_next(&mut x)?,
            width: parse_next(&mut x)?,
            height: parse_next(&mut x)?,
            conf: parse_next(&mut x)?,
            text: x.next().unwrap_or("").to_string(),
        })
    }
}

pub fn parse_next<T: std::str::FromStr>(
    iter: &mut std::str::SplitWhitespace<'_>,
) -> Option<T> {
    iter.next()?.parse::<T>().ok()
}

pub trait FromLine: Sized {
    fn from_line(line: &str) -> Option<Self>;

    fn parse(line: &str) -> TessResult<Self> {
        Self::from_line(line).ok_or(TessError::ParseError(format!("invalid line '{}'", line)))
    }
}

pub fn string_to_data(output: &str) -> TessResult<Vec<Data>> {
    println!("okay till 63");
    let data = output
        .lines()
        .into_iter()
        // .skip(1)
        .map(|line| Data::parse(line.into()))
        .collect::<_>();
    println!("okay till 70");
    return data;
}

use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum TessError {
    #[error("Tesseract not found. Please check installation path!")]
    TesseractNotFoundError,

    #[error("Invalid Tesseract version!\n{0}")]
    VersionError(String),

    #[error(
        "Image format not within the list of allowed image formats:\n\
        ['JPEG','JPG','PNG','PBM','PGM','PPM','TIFF','BMP','GIF','WEBP']"
    )]
    ImageFormatError,

    #[error("Please assign a valid image path.")]
    ImageNotFoundError,

    #[error("Could not parse {0}.")]
    ParseError(String),

    #[error("Could not create tempfile.\n{0}")]
    TempfileError(String),

    #[error("Could not save dynamic image to tempfile.\n{0}")]
    DynamicImageError(String),
}

pub type TessResult<T> = Result<T, TessError>;
